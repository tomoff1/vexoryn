mod ftp;
use ftp::{Connection, Entry, Ftp, Result};
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::{
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    net::{IpAddr, Shutdown, SocketAddr, TcpStream},
    path::{Path, PathBuf},
    sync::Mutex,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};
use tauri::{Emitter, Manager};
#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct Service {
    port: u16,
    label: String,
    reachable: bool,
    latency_ms: Option<u128>,
}
#[derive(Serialize)]
struct Candidate {
    ip: String,
    services: Vec<Service>,
}
fn subnet_prefix(subnet: &str) -> Result<[u8; 3]> {
    let (ip, mask) = subnet
        .split_once('/')
        .ok_or("Sous-réseau IPv4 /24 requis")?;
    if mask != "24" {
        return Err("Seuls les sous-réseaux /24 (254 hôtes maximum) sont autorisés".into());
    }
    let octets = ftp::local_ip(ip)?.octets();
    if octets[3] != 0 {
        return Err("Adresse de réseau terminant par .0 requise".into());
    }
    Ok([octets[0], octets[1], octets[2]])
}
static SCAN_RUNNING: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
struct ScanGuard;
impl Drop for ScanGuard {
    fn drop(&mut self) {
        SCAN_RUNNING.store(false, std::sync::atomic::Ordering::Release)
    }
}
#[tauri::command]
async fn discover_consoles(
    app: tauri::AppHandle,
    subnet: String,
    authorized: bool,
) -> Result<Vec<Candidate>> {
    blocking(move || {
        if !authorized {
            return Err("Autorisation explicite du propriétaire du réseau requise".into());
        }
        let prefix = subnet_prefix(&subnet)?;
        if SCAN_RUNNING
            .compare_exchange(
                false,
                true,
                std::sync::atomic::Ordering::AcqRel,
                std::sync::atomic::Ordering::Acquire,
            )
            .is_err()
        {
            return Err("Un scan réseau est déjà en cours".into());
        }
        let _guard = ScanGuard;
        let workers: Vec<_> = (0..16u16)
            .map(|worker| {
                std::thread::spawn(move || {
                    let mut candidates = Vec::new();
                    for host in ((worker + 1)..=254).step_by(16) {
                        let ip = format!("{}.{}.{}.{}", prefix[0], prefix[1], prefix[2], host);
                        let services = diagnose(&ip)?;
                        if services.iter().any(|s| s.reachable) {
                            candidates.push(Candidate { ip, services })
                        }
                    }
                    Ok(candidates)
                })
            })
            .collect();
        let mut candidates = Vec::new();
        for worker in workers {
            let entries: Result<Vec<Candidate>> = worker.join().map_err(|_| "Scan interrompu")?;
            candidates.extend(entries?)
        }
        candidates.sort_by_key(|c| c.ip.parse::<std::net::Ipv4Addr>().ok());
        record(
            &app,
            "discovery",
            &subnet,
            "success",
            &format!(
                "{} TCP candidates; console identity unconfirmed",
                candidates.len()
            ),
        );
        Ok(candidates)
    })
    .await
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Console {
    ip: String,
    port: u16,
    banner: String,
    services: Vec<Service>,
    firmware: Option<String>,
    free_space: Option<u64>,
    temperature: Option<f64>,
}
#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct Progress {
    transfer_id: String,
    bytes: u64,
    total: Option<u64>,
    speed_bytes_per_second: f64,
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct TransferResult {
    bytes: u64,
    sha256: String,
    verification: String,
    remote_confirmed: bool,
    backup_path: Option<String>,
}
#[derive(Serialize)]
struct PayloadResult {
    bytes: u64,
    sha256: String,
    status: String,
}
#[derive(Clone, serde::Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct HistoryEntry {
    timestamp: u64,
    operation: String,
    ip: String,
    status: String,
    message: String,
}
#[derive(Default)]
struct History(Mutex<Vec<HistoryEntry>>);
fn epoch() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}
fn history_path(app: &tauri::AppHandle) -> Result<PathBuf> {
    let p = app.path().app_local_data_dir().map_err(|e| e.to_string())?;
    fs::create_dir_all(&p).map_err(|e| e.to_string())?;
    Ok(p.join("history.json"))
}
fn record(app: &tauri::AppHandle, operation: &str, ip: &str, status: &str, message: &str) {
    let state = app.state::<History>();
    if let Ok(mut entries) = state.0.lock() {
        entries.push(HistoryEntry {
            timestamp: epoch(),
            operation: operation.into(),
            ip: ip.into(),
            status: status.into(),
            message: message.into(),
        });
        if entries.len() > 1000 {
            entries.remove(0);
        }
        if let Ok(p) = history_path(app) {
            if let Ok(bytes) = serde_json::to_vec_pretty(&*entries) {
                let tmp = p.with_extension("tmp");
                if fs::write(&tmp, bytes).is_ok() {
                    if fs::rename(&tmp, &p).is_err() {
                        if fs::copy(&tmp, &p).is_ok() {
                            let _ = fs::remove_file(&tmp);
                        }
                    }
                }
            }
        }
    };
}
async fn blocking<T: Send + 'static>(f: impl FnOnce() -> Result<T> + Send + 'static) -> Result<T> {
    tauri::async_runtime::spawn_blocking(f)
        .await
        .map_err(|e| e.to_string())?
}
fn diagnose(ip: &str) -> Result<Vec<Service>> {
    let addr = ftp::local_ip(ip)?;
    let ports = [
        (1337, "FTP etaHEN (TCP)"),
        (2121, "FTP alternatif (TCP)"),
        (21, "FTP standard (TCP)"),
        (9020, "Chargeur ELF potentiel (TCP)"),
        (9021, "Chargeur ELF etaHEN potentiel (TCP)"),
        (744, "ps5debug potentiel (TCP)"),
    ];
    let workers: Vec<_> = ports
        .into_iter()
        .map(|(port, label)| {
            std::thread::spawn(move || {
                let now = Instant::now();
                let reachable = TcpStream::connect_timeout(
                    &SocketAddr::new(IpAddr::V4(addr), port),
                    Duration::from_millis(750),
                )
                .is_ok();
                Service {
                    port,
                    label: label.into(),
                    reachable,
                    latency_ms: if reachable {
                        Some(now.elapsed().as_millis())
                    } else {
                        None
                    },
                }
            })
        })
        .collect();
    workers
        .into_iter()
        .map(|w| w.join().map_err(|_| "Diagnostic interrompu".into()))
        .collect()
}
#[tauri::command]
async fn network_diagnostics(ip: String) -> Result<Vec<Service>> {
    blocking(move || diagnose(&ip)).await
}
#[tauri::command]
async fn connect_console(app: tauri::AppHandle, connection: Connection) -> Result<Console> {
    blocking(move || {
        let result = (|| {
            let f = Ftp::connect(&connection)?;
            Ok(Console {
                ip: connection.ip.clone(),
                port: connection.port,
                banner: f.banner.clone(),
                services: diagnose(&connection.ip)?,
                firmware: None,
                free_space: None,
                temperature: None,
            })
        })();
        record(
            &app,
            "connect",
            &connection.ip,
            if result.is_ok() { "success" } else { "failed" },
            "FTP connection",
        );
        result
    })
    .await
}
#[tauri::command]
async fn list_remote(connection: Connection, path: String) -> Result<Vec<Entry>> {
    blocking(move || Ftp::connect(&connection)?.list(&path)).await
}
#[tauri::command]
async fn create_remote_dir(
    app: tauri::AppHandle,
    connection: Connection,
    path: String,
) -> Result<()> {
    blocking(move || {
        ftp::path(&path)?;
        Ftp::connect(&connection)?.expect(&format!("MKD {path}"), &[257, 250])?;
        record(&app, "mkdir", &connection.ip, "success", &path);
        Ok(())
    })
    .await
}
#[tauri::command]
async fn delete_remote(
    app: tauri::AppHandle,
    connection: Connection,
    path: String,
    is_dir: bool,
    confirmed: bool,
) -> Result<()> {
    blocking(move || {
        if !confirmed {
            return Err("Confirmation de suppression requise".into());
        }
        ftp::path(&path)?;
        if path.trim_end_matches('/').is_empty() {
            return Err("Suppression de la racine interdite".into());
        }
        let cmd = if is_dir { "RMD" } else { "DELE" };
        let result = Ftp::connect(&connection)?
            .expect(&format!("{cmd} {path}"), &[250])
            .map(|_| ());
        record(
            &app,
            "delete",
            &connection.ip,
            if result.is_ok() { "success" } else { "failed" },
            &path,
        );
        result
    })
    .await
}
fn progress(app: &tauri::AppHandle, id: &str, bytes: u64, total: Option<u64>, start: Instant) {
    let _ = app.emit(
        "transfer-progress",
        Progress {
            transfer_id: id.into(),
            bytes,
            total,
            speed_bytes_per_second: bytes as f64 / start.elapsed().as_secs_f64().max(0.001),
        },
    );
}
fn stream_copy(
    reader: &mut impl Read,
    writer: &mut impl Write,
    app: &tauri::AppHandle,
    id: &str,
    total: Option<u64>,
) -> Result<(u64, String)> {
    let start = Instant::now();
    let mut last = Instant::now();
    let mut bytes = 0;
    let mut hash = Sha256::new();
    let mut buffer = [0u8; 128 * 1024];
    loop {
        let n = reader.read(&mut buffer).map_err(|e| e.to_string())?;
        if n == 0 {
            break;
        }
        writer.write_all(&buffer[..n]).map_err(|e| e.to_string())?;
        hash.update(&buffer[..n]);
        bytes += n as u64;
        if last.elapsed() > Duration::from_millis(150) {
            progress(app, id, bytes, total, start);
            last = Instant::now()
        }
    }
    progress(app, id, bytes, total, start);
    Ok((bytes, format!("{:x}", hash.finalize())))
}
fn remote_entry(ftp: &mut Ftp, p: &str) -> Result<Option<Entry>> {
    let (parent, name) = p.rsplit_once('/').ok_or("Chemin distant invalide")?;
    if name.is_empty() {
        return Err("Nom de fichier distant requis".into());
    }
    let parent = if parent.is_empty() { "/" } else { parent };
    Ok(ftp.list(parent)?.into_iter().find(|e| e.name == name))
}
fn remote_existing(ftp: &mut Ftp, p: &str) -> Result<bool> {
    Ok(remote_entry(ftp, p)?.is_some())
}
fn transfer(
    app: tauri::AppHandle,
    c: Connection,
    remote: String,
    local: String,
    id: String,
    overwrite: bool,
    upload: bool,
) -> Result<TransferResult> {
    ftp::path(&remote)?;
    ftp::clean(&id)?;
    let mut ftp = Ftp::connect(&c)?;
    let result = (|| {
        if upload {
            let mut file = File::open(&local).map_err(|e| format!("Fichier local: {e}"))?;
            let total = file.metadata().map_err(|e| e.to_string())?.len();
            let mut backup = None;
            if let Some(existing) = remote_entry(&mut ftp, &remote)? {
                if existing.is_dir {
                    return Err(
                        "Un dossier distant ne peut pas être remplacé par un fichier".into(),
                    );
                }
                if !overwrite {
                    return Err(
                        "Le fichier distant existe; confirmation de remplacement requise".into(),
                    );
                }
                let p = format!("{remote}.vexoryn-backup-{}", epoch());
                if remote_existing(&mut ftp, &p)? {
                    return Err("Nom de sauvegarde déjà existant; réessayez".into());
                }
                ftp.expect(&format!("RNFR {remote}"), &[350])?;
                ftp.expect(&format!("RNTO {p}"), &[250])?;
                backup = Some(p);
            }
            let mut data = ftp.passive()?;
            ftp.expect(&format!("STOR {remote}"), &[125, 150])?;
            let (bytes, sha256) = stream_copy(&mut file, &mut data, &app, &id, Some(total))?;
            data.shutdown(Shutdown::Write).map_err(|e| e.to_string())?;
            drop(data);
            ftp.finish()?;
            if bytes != total {
                return Err("Le fichier source a changé pendant le transfert".into());
            }
            if let Some(remote_size) = ftp.size(&remote)? {
                if remote_size != bytes {
                    return Err("Taille distante différente après transfert".into());
                }
            }
            Ok(TransferResult {
                bytes,
                sha256,
                verification: "local-sha256; FTP completion; remote-size-if-supported".into(),
                remote_confirmed: true,
                backup_path: backup,
            })
        } else {
            let dest = Path::new(&local);
            if dest.exists() && !overwrite {
                return Err("Le fichier local existe; confirmation requise".into());
            }
            let total = ftp.size(&remote)?;
            let tmp = dest.with_file_name(format!(
                "{}.vexoryn-part-{}",
                dest.file_name()
                    .ok_or("Nom local invalide")?
                    .to_string_lossy(),
                epoch()
            ));
            let mut file = OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&tmp)
                .map_err(|e| e.to_string())?;
            let received: Result<(u64, String)> = (|| {
                let mut data = ftp.passive()?;
                ftp.expect(&format!("RETR {remote}"), &[125, 150])?;
                let result = stream_copy(&mut data, &mut file, &app, &id, total)?;
                drop(data);
                ftp.finish()?;
                file.sync_all().map_err(|e| e.to_string())?;
                if total.is_some_and(|n| n != result.0) {
                    return Err("Téléchargement incomplet".into());
                }
                Ok(result)
            })();
            drop(file);
            let (bytes, sha256) = match received {
                Ok(r) => r,
                Err(e) => return Err(format!("{e}. Fichier partiel conservé (non finalisé).")),
            };
            let mut backup = None;
            if dest.exists() {
                if !overwrite {
                    return Err("Un fichier local est apparu pendant le transfert; téléchargement partiel conservé".into());
                }
                if dest.is_dir() {
                    return Err("Un dossier local ne peut pas être remplacé par un fichier".into());
                }
                let p = dest.with_file_name(format!(
                    "{}.vexoryn-backup-{}",
                    dest.file_name().unwrap().to_string_lossy(),
                    epoch()
                ));
                if p.exists() {
                    return Err("Sauvegarde locale déjà existante".into());
                }
                fs::rename(dest, &p).map_err(|e| e.to_string())?;
                backup = Some("local backup retained".into())
            }
            fs::rename(tmp, dest).map_err(|e| e.to_string())?;
            Ok(TransferResult {
                bytes,
                sha256,
                verification: "local-sha256; FTP completion; remote-size-if-supported".into(),
                remote_confirmed: true,
                backup_path: backup,
            })
        }
    })();
    record(
        &app,
        if upload { "upload" } else { "download" },
        &c.ip,
        if result.is_ok() { "success" } else { "failed" },
        &remote,
    );
    result
}
#[tauri::command]
async fn upload_file(
    app: tauri::AppHandle,
    connection: Connection,
    remote_path: String,
    local_path: String,
    transfer_id: String,
    overwrite: bool,
) -> Result<TransferResult> {
    blocking(move || {
        transfer(
            app,
            connection,
            remote_path,
            local_path,
            transfer_id,
            overwrite,
            true,
        )
    })
    .await
}
#[tauri::command]
async fn download_file(
    app: tauri::AppHandle,
    connection: Connection,
    remote_path: String,
    local_path: String,
    transfer_id: String,
    overwrite: bool,
) -> Result<TransferResult> {
    blocking(move || {
        transfer(
            app,
            connection,
            remote_path,
            local_path,
            transfer_id,
            overwrite,
            false,
        )
    })
    .await
}
fn validate_elf(header: &[u8]) -> Result<()> {
    if header.len() < 64
        || header[..4] != [0x7f, b'E', b'L', b'F']
        || header[4] != 2
        || header[5] != 1
        || header[6] != 1
        || u16::from_le_bytes([header[18], header[19]]) != 62
        || u16::from_le_bytes([header[52], header[53]]) != 64
    {
        return Err("ELF 64 bits x86-64 little-endian valide requis".into());
    }
    Ok(())
}
#[tauri::command]
async fn send_payload(
    app: tauri::AppHandle,
    ip: String,
    port: u16,
    local_path: String,
    confirmed: bool,
) -> Result<PayloadResult> {
    blocking(move || {
        if !confirmed {
            return Err(
                "Confirmez la compatibilité firmware et l'absence de payload incompatible".into(),
            );
        }
        let addr = ftp::local_ip(&ip)?;
        if ![9020, 9021].contains(&port) {
            return Err("Seuls les chargeurs ELF TCP 9020/9021 sont pris en charge".into());
        }
        let mut file = File::open(local_path).map_err(|e| e.to_string())?;
        let size = file.metadata().map_err(|e| e.to_string())?.len();
        if size > 64 * 1024 * 1024 {
            return Err("ELF supérieur à 64 Mio".into());
        }
        let mut header = [0; 64];
        file.read_exact(&mut header).map_err(|e| e.to_string())?;
        validate_elf(&header)?;
        use std::io::{Seek, SeekFrom};
        file.seek(SeekFrom::Start(0)).map_err(|e| e.to_string())?;
        let mut stream = TcpStream::connect_timeout(
            &SocketAddr::new(IpAddr::V4(addr), port),
            Duration::from_secs(3),
        )
        .map_err(|e| e.to_string())?;
        stream
            .set_write_timeout(Some(Duration::from_secs(30)))
            .map_err(|e| e.to_string())?;
        let (bytes, sha256) = stream_copy(&mut file, &mut stream, &app, "payload", Some(size))?;
        stream
            .shutdown(Shutdown::Write)
            .map_err(|e| e.to_string())?;
        if bytes != size {
            return Err(
                "Le fichier ELF a changé pendant le transfert; exécution non confirmée".into(),
            );
        }
        record(
            &app,
            "payload",
            &ip,
            "sent-unconfirmed",
            "ELF TCP sent; execution not confirmed",
        );
        Ok(PayloadResult {
            bytes,
            sha256,
            status: "sent-unconfirmed".into(),
        })
    })
    .await
}
#[tauri::command]
fn get_history(state: tauri::State<'_, History>) -> Result<Vec<HistoryEntry>> {
    state
        .0
        .lock()
        .map(|s| s.iter().rev().cloned().collect())
        .map_err(|_| "Historique verrouillé".into())
}
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .manage(History::default())
        .setup(|app| {
            if let Ok(p) = history_path(app.handle()) {
                if let Ok(bytes) = fs::read(p) {
                    if let Ok(entries) = serde_json::from_slice::<Vec<HistoryEntry>>(&bytes) {
                        let state = app.state::<History>();
                        if let Ok(mut h) = state.0.lock() {
                            *h = entries
                                .into_iter()
                                .rev()
                                .take(1000)
                                .collect::<Vec<_>>()
                                .into_iter()
                                .rev()
                                .collect();
                        };
                    }
                }
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            connect_console,
            list_remote,
            upload_file,
            download_file,
            create_remote_dir,
            delete_remote,
            network_diagnostics,
            discover_consoles,
            send_payload,
            get_history
        ])
        .run(tauri::generate_context!())
        .expect("VEXORYN runtime error");
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn scan_boundary() {
        assert_eq!(subnet_prefix("192.168.1.0/24").unwrap(), [192, 168, 1]);
        for invalid in [
            "8.8.8.0/24",
            "127.0.0.0/24",
            "192.168.1.0/16",
            "192.168.1.2/24",
            "192.168.1.0/24/24",
        ] {
            assert!(subnet_prefix(invalid).is_err())
        }
    }
    #[test]
    fn rejects_non_elf() {
        assert!(validate_elf(&[0; 64]).is_err())
    }
    #[test]
    fn elf_header() {
        let mut h = [0; 64];
        h[..4].copy_from_slice(b"\x7fELF");
        h[4] = 2;
        h[5] = 1;
        h[6] = 1;
        h[18] = 62;
        h[52] = 64;
        assert!(validate_elf(&h).is_ok());
        h[18] = 3;
        assert!(validate_elf(&h).is_err())
    }
}
