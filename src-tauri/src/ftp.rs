//! Minimal passive, binary FTP client. RFC 959 and RFC 3659.
use serde::{Deserialize, Serialize};
use std::{
    io::{BufRead, BufReader, Read, Write},
    net::{IpAddr, Ipv4Addr, SocketAddr, TcpStream},
    time::Duration,
};
pub type Result<T> = std::result::Result<T, String>;
#[derive(Clone, Deserialize)]
pub struct Connection {
    pub ip: String,
    pub port: u16,
    pub username: String,
    pub password: String,
}
#[derive(Clone, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Entry {
    pub name: String,
    pub path: String,
    pub is_dir: bool,
    pub size: Option<u64>,
    pub modified: Option<String>,
    pub raw: String,
}
pub fn local_ip(ip: &str) -> Result<Ipv4Addr> {
    let ip: Ipv4Addr = ip.parse().map_err(|_| "Adresse IPv4 invalide")?;
    if !ip.is_private() {
        return Err("Seules les adresses IPv4 privées du réseau local sont autorisées".into());
    }
    Ok(ip)
}
pub fn clean(value: &str) -> Result<()> {
    if value.is_empty() || value.len() > 4096 || value.chars().any(|c| c.is_control()) {
        Err("Paramètre vide ou caractère de contrôle interdit".into())
    } else {
        Ok(())
    }
}
pub fn path(value: &str) -> Result<()> {
    clean(value)?;
    if !value.starts_with('/') || value.split('/').any(|c| c == ".." || c == ".") {
        Err("Chemin absolu requis; traversal interdit".into())
    } else {
        Ok(())
    }
}
fn bounded_line(reader: &mut impl BufRead) -> Result<String> {
    let mut bytes = Vec::new();
    loop {
        let buffer = reader.fill_buf().map_err(|e| e.to_string())?;
        if buffer.is_empty() {
            return Err("Réponse FTP interrompue".into());
        }
        let newline = buffer.iter().position(|b| *b == b'\n');
        let take = newline.map(|n| n + 1).unwrap_or(buffer.len());
        if bytes.len() + take > 64 * 1024 {
            return Err("Ligne FTP supérieure à 64 Kio".into());
        }
        bytes.extend_from_slice(&buffer[..take]);
        reader.consume(take);
        if newline.is_some() {
            return String::from_utf8(bytes).map_err(|_| "Réponse FTP UTF-8 invalide".into());
        }
    }
}
pub struct Ftp {
    reader: BufReader<TcpStream>,
    ip: Ipv4Addr,
    pub banner: String,
}
impl Ftp {
    pub fn connect(c: &Connection) -> Result<Self> {
        Self::connect_ip(c, local_ip(&c.ip)?)
    }
    fn connect_ip(c: &Connection, ip: Ipv4Addr) -> Result<Self> {
        clean(&c.username)?;
        if c.password.len() > 4096 || c.password.chars().any(|c| c.is_control()) {
            return Err("Mot de passe invalide".into());
        }
        if c.port == 0 {
            return Err("Port invalide".into());
        }
        let stream = TcpStream::connect_timeout(
            &SocketAddr::new(IpAddr::V4(ip), c.port),
            Duration::from_secs(3),
        )
        .map_err(|e| format!("Connexion FTP: {e}"))?;
        stream
            .set_read_timeout(Some(Duration::from_secs(30)))
            .map_err(|e| e.to_string())?;
        stream
            .set_write_timeout(Some(Duration::from_secs(30)))
            .map_err(|e| e.to_string())?;
        let mut f = Self {
            reader: BufReader::new(stream),
            ip,
            banner: String::new(),
        };
        let (code, banner) = f.reply()?;
        if code != 220 {
            return Err(format!("FTP: {banner}"));
        }
        f.banner = banner;
        let (code, msg) = f.command(&format!("USER {}", c.username))?;
        if code == 331 {
            f.expect(&format!("PASS {}", c.password), &[230])?;
        } else if code != 230 {
            return Err(format!("FTP authentification: {msg}"));
        }
        f.expect("TYPE I", &[200])?;
        Ok(f)
    }
    fn reply(&mut self) -> Result<(u16, String)> {
        let first = bounded_line(&mut self.reader)?;
        if first.len() < 4 {
            return Err("Réponse FTP tronquée".into());
        }
        if !first.as_bytes()[..3].iter().all(u8::is_ascii_digit) {
            return Err("Code de réponse FTP invalide".into());
        }
        let code = first[..3]
            .parse::<u16>()
            .map_err(|_| "Réponse FTP invalide")?;
        let mut full = first.clone();
        if first.as_bytes()[3] == b'-' {
            let end = format!("{code} ");
            for _ in 0..1000 {
                let line = bounded_line(&mut self.reader)?;
                let done = line.starts_with(&end);
                full.push_str(&line);
                if full.len() > 1024 * 1024 {
                    return Err("Réponse FTP trop grande".into());
                }
                if done {
                    return Ok((code, full.trim().into()));
                }
            }
            return Err("Réponse FTP trop longue".into());
        }
        Ok((code, full.trim().into()))
    }
    pub fn command(&mut self, cmd: &str) -> Result<(u16, String)> {
        clean(cmd)?;
        self.reader
            .get_mut()
            .write_all(format!("{cmd}\r\n").as_bytes())
            .map_err(|e| e.to_string())?;
        self.reply()
    }
    pub fn expect(&mut self, cmd: &str, codes: &[u16]) -> Result<String> {
        let (code, msg) = self.command(cmd)?;
        if codes.contains(&code) {
            Ok(msg)
        } else {
            Err(format!("FTP {code}: {msg}"))
        }
    }
    pub fn passive(&mut self) -> Result<TcpStream> {
        let reply = self.expect("PASV", &[227])?;
        let port = pasv_port(&reply)?;
        // Ignore the PASV advertised address: data connections must remain on the authorized host.
        let data = TcpStream::connect_timeout(
            &SocketAddr::new(IpAddr::V4(self.ip), port),
            Duration::from_secs(3),
        )
        .map_err(|e| e.to_string())?;
        data.set_read_timeout(Some(Duration::from_secs(30)))
            .map_err(|e| e.to_string())?;
        data.set_write_timeout(Some(Duration::from_secs(30)))
            .map_err(|e| e.to_string())?;
        Ok(data)
    }
    pub fn finish(&mut self) -> Result<()> {
        let (code, msg) = self.reply()?;
        if code == 226 || code == 250 {
            Ok(())
        } else {
            Err(format!("Transfert non confirmé: {msg}"))
        }
    }
    pub fn size(&mut self, p: &str) -> Result<Option<u64>> {
        path(p)?;
        let (code, msg) = self.command(&format!("SIZE {p}"))?;
        Ok(if code == 213 {
            msg[4..].trim().parse().ok()
        } else {
            None
        })
    }
    pub fn list(&mut self, p: &str) -> Result<Vec<Entry>> {
        path(p)?;
        let mut data = self.passive()?;
        let (code, msg) = self.command(&format!("MLSD {p}"))?;
        let mlsd = if [125, 150].contains(&code) {
            true
        } else if [500, 501, 502, 504].contains(&code) {
            drop(data);
            data = self.passive()?;
            self.expect(&format!("LIST {p}"), &[125, 150])?;
            false
        } else {
            return Err(msg);
        };
        let mut bytes = Vec::new();
        (&mut data)
            .take(16 * 1024 * 1024 + 1)
            .read_to_end(&mut bytes)
            .map_err(|e| e.to_string())?;
        drop(data);
        self.finish()?;
        if bytes.len() > 16 * 1024 * 1024 {
            return Err("Listing trop volumineux".into());
        }
        let listing = String::from_utf8(bytes).map_err(|_| "Listing FTP UTF-8 invalide")?;
        listing
            .lines()
            .filter(|l| !l.is_empty())
            .filter_map(|l| parse_entry(p, l, mlsd))
            .collect()
    }
}
pub fn pasv_port(reply: &str) -> Result<u16> {
    let begin = reply.find('(').ok_or("PASV invalide")?;
    let end = reply[begin..].find(')').ok_or("PASV invalide")? + begin;
    let nums: Vec<u16> = reply[begin + 1..end]
        .split(',')
        .map(|n| n.trim().parse::<u16>())
        .collect::<std::result::Result<_, _>>()
        .map_err(|_| "PASV invalide")?;
    if nums.len() != 6 || nums.iter().any(|n| *n > 255) {
        return Err("PASV invalide".into());
    }
    let port = nums[4] * 256 + nums[5];
    if port == 0 {
        return Err("Port PASV nul".into());
    }
    Ok(port)
}
fn parse_entry(parent: &str, line: &str, mlsd: bool) -> Option<Result<Entry>> {
    let (name, is_dir, size, modified) = if mlsd {
        let Some((facts, name)) = line.split_once(' ') else {
            return Some(Err("Ligne MLSD sans nom de fichier".into()));
        };
        let mut dir = None;
        let mut size = None;
        let mut modified = None;
        for fact in facts.split(';').filter(|f| !f.is_empty()) {
            let Some((k, v)) = fact.split_once('=') else {
                return Some(Err("Fait MLSD invalide".into()));
            };
            match k.to_ascii_lowercase().as_str() {
                "type" => match v.to_ascii_lowercase().as_str() {
                    "cdir" | "pdir" => return None,
                    "dir" => dir = Some(true),
                    "file" => dir = Some(false),
                    _ => return Some(Err("Type MLSD non pris en charge".into())),
                },
                "size" => match v.parse::<u64>() {
                    Ok(n) => size = Some(n),
                    Err(_) => return Some(Err("Taille MLSD invalide".into())),
                },
                "modify" => modified = Some(v.into()),
                _ => {}
            }
        }
        let Some(dir) = dir else {
            return Some(Err("Type MLSD manquant".into()));
        };
        (name.to_string(), dir, size, modified)
    } else {
        let fields: Vec<&str> = line.split_whitespace().collect();
        if fields.len() == 2 && fields[0] == "total" && fields[1].parse::<u64>().is_ok() {
            return None;
        }
        let Some(perms) = fields.first() else {
            return Some(Err("Listing LIST vide".into()));
        };
        if !perms.starts_with('d') && !perms.starts_with('-') {
            return Some(Err(format!("Format LIST non pris en charge: {line}")));
        }
        if fields.len() < 9 {
            return Some(Err("Listing LIST invalide".into()));
        }
        let size = match fields[4].parse::<u64>() {
            Ok(n) => Some(n),
            Err(_) => return Some(Err("Taille LIST invalide".into())),
        };
        let mut offset = 0;
        for _ in 0..8 {
            offset += line[offset..]
                .find(|c: char| !c.is_whitespace())
                .unwrap_or(0);
            offset += line[offset..]
                .find(char::is_whitespace)
                .unwrap_or(line.len() - offset)
        }
        (
            line[offset..].trim_start().to_string(),
            perms.starts_with('d'),
            size,
            None,
        )
    };
    if name == "." || name == ".." {
        return None;
    }
    if name.is_empty() || name.contains('/') || name.chars().any(|c| c.is_control()) {
        return Some(Err("Nom FTP invalide".into()));
    }
    Some(Ok(Entry {
        name: name.clone(),
        path: format!("{}/{}", parent.trim_end_matches('/'), name),
        is_dir,
        size,
        modified,
        raw: line.into(),
    }))
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn bounded_reply_and_malformed_list() {
        let bytes = vec![b'x'; 65537];
        assert!(bounded_line(&mut std::io::Cursor::new(bytes)).is_err());
        assert_eq!(
            bounded_line(&mut std::io::Cursor::new(b"220 ok\r\n")).unwrap(),
            "220 ok\r\n"
        );
        for invalid in [
            "not-a-valid-entry",
            "size=oops;type=file; file",
            "size=7; missing-type",
            "type=file; ",
        ] {
            assert!(parse_entry("/", invalid, true).unwrap().is_err())
        }
    }
    #[test]
    fn mock_binary_transfers_and_refused_completion() {
        use std::net::TcpListener;
        let server = TcpListener::bind("127.0.0.1:0").unwrap();
        let port = server.local_addr().unwrap().port();
        let expected: Vec<u8> = (0..131072).map(|n| (n % 256) as u8).collect();
        let server_expected = expected.clone();
        let worker = std::thread::spawn(move || {
            let (stream, _) = server.accept().unwrap();
            let mut reader = BufReader::new(stream);
            reader.get_mut().write_all(b"220 mock\r\n").unwrap();
            let mut passive = None;
            let mut stored = Vec::new();
            loop {
                let mut cmd = String::new();
                if reader.read_line(&mut cmd).unwrap() == 0 {
                    break;
                }
                let response = if cmd.starts_with("USER ") {
                    "230 logged in".into()
                } else if cmd.starts_with("TYPE ") {
                    "200 binary".into()
                } else if cmd.starts_with("PASV") {
                    let l = TcpListener::bind("127.0.0.1:0").unwrap();
                    let p = l.local_addr().unwrap().port();
                    passive = Some(l);
                    format!("227 (8,8,8,8,{},{})", p / 256, p % 256)
                } else if cmd.starts_with("STOR ") {
                    reader.get_mut().write_all(b"150 send\r\n").unwrap();
                    let (mut d, _) = passive.take().unwrap().accept().unwrap();
                    d.read_to_end(&mut stored).unwrap();
                    assert_eq!(stored, server_expected);
                    "226 stored".into()
                } else if cmd.starts_with("RETR ") {
                    reader.get_mut().write_all(b"150 receive\r\n").unwrap();
                    let (mut d, _) = passive.take().unwrap().accept().unwrap();
                    d.write_all(&stored).unwrap();
                    drop(d);
                    if cmd.contains("fail") {
                        "550 failed".into()
                    } else {
                        "226 retrieved".into()
                    }
                } else if cmd.starts_with("SIZE ") {
                    format!("213 {}", stored.len())
                } else {
                    "500 unknown".into()
                };
                reader
                    .get_mut()
                    .write_all(format!("{response}\r\n").as_bytes())
                    .unwrap()
            }
        });
        let c = Connection {
            ip: "127.0.0.1".into(),
            port,
            username: "anonymous".into(),
            password: "".into(),
        };
        let mut f = Ftp::connect_ip(&c, Ipv4Addr::LOCALHOST).unwrap();
        let mut d = f.passive().unwrap();
        f.expect("STOR /binary", &[150]).unwrap();
        d.write_all(&expected).unwrap();
        d.shutdown(std::net::Shutdown::Write).unwrap();
        drop(d);
        f.finish().unwrap();
        assert_eq!(f.size("/binary").unwrap(), Some(expected.len() as u64));
        let mut d = f.passive().unwrap();
        f.expect("RETR /binary", &[150]).unwrap();
        let mut received = Vec::new();
        d.read_to_end(&mut received).unwrap();
        drop(d);
        f.finish().unwrap();
        assert_eq!(received, expected);
        let mut d = f.passive().unwrap();
        f.expect("RETR /fail", &[150]).unwrap();
        let mut received = Vec::new();
        d.read_to_end(&mut received).unwrap();
        drop(d);
        assert!(f.finish().is_err());
        drop(f);
        worker.join().unwrap();
    }
    #[test]
    fn safety() {
        assert!(local_ip("8.8.8.8").is_err());
        assert!(local_ip("127.0.0.1").is_err());
        assert!(local_ip("192.168.1.9").is_ok());
        assert!(clean("x\r\nDELE /").is_err());
        assert!(path("/../etc").is_err());
        assert!(path("/.").is_err())
    }
    #[test]
    fn passive_address() {
        assert_eq!(pasv_port("227 Entering (8,8,8,8,5,57)").unwrap(), 1337);
        assert!(pasv_port("227 (1,2,3,4,256,1)").is_err())
    }
    #[test]
    fn listings() {
        let e = parse_entry(
            "/data",
            "type=file;size=8;modify=20261013000100; a b.elf",
            true,
        )
        .unwrap()
        .unwrap();
        assert_eq!(e.name, "a b.elf");
        assert_eq!(e.size, Some(8));
        let e = parse_entry(
            "/",
            "drwxr-xr-x 1 root root 0 Oct 13 2026 folder name",
            false,
        )
        .unwrap()
        .unwrap();
        assert_eq!(e.name, "folder name");
        assert!(e.is_dir)
    }
    #[test]
    fn mock_login_and_list() {
        use std::net::TcpListener;
        let server = TcpListener::bind("127.0.0.1:0").unwrap();
        let port = server.local_addr().unwrap().port();
        let worker = std::thread::spawn(move || {
            let (stream, _) = server.accept().unwrap();
            let mut reader = BufReader::new(stream);
            reader.get_mut().write_all(b"220 mock\r\n").unwrap();
            let mut passive = None;
            loop {
                let mut cmd = String::new();
                if reader.read_line(&mut cmd).unwrap() == 0 {
                    break;
                }
                let response = if cmd.starts_with("USER ") {
                    "331 password".into()
                } else if cmd.starts_with("PASS ") {
                    "230 logged in".into()
                } else if cmd.starts_with("TYPE ") {
                    "200 binary".into()
                } else if cmd.starts_with("PASV") {
                    let l = TcpListener::bind("127.0.0.1:0").unwrap();
                    let p = l.local_addr().unwrap().port();
                    passive = Some(l);
                    format!("227 (127,0,0,1,{},{})", p / 256, p % 256)
                } else if cmd.starts_with("MLSD") {
                    reader.get_mut().write_all(b"150 opening\r\n").unwrap();
                    let (mut d, _) = passive.take().unwrap().accept().unwrap();
                    d.write_all(b"type=file;size=4; test.elf\r\n").unwrap();
                    drop(d);
                    "226 complete".into()
                } else {
                    "500 unknown".into()
                };
                reader
                    .get_mut()
                    .write_all(format!("{response}\r\n").as_bytes())
                    .unwrap()
            }
        });
        let c = Connection {
            ip: "127.0.0.1".into(),
            port,
            username: "anonymous".into(),
            password: "".into(),
        };
        let mut f = Ftp::connect_ip(&c, Ipv4Addr::LOCALHOST).unwrap();
        assert_eq!(f.list("/").unwrap()[0].name, "test.elf");
        drop(f);
        worker.join().unwrap()
    }
}
