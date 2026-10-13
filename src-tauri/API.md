# VEXORYN native API

All commands accept camelCase JSON arguments. Blocking sockets and disk operations run on Tauri's blocking thread pool. Credentials exist only in the current request and are never saved.

`connection`: `{ ip: string, port: number, username: string, password: string }`.

| Command | Additional arguments | Result |
| --- | --- | --- |
| `connect_console` | connection | FTP greeting, IP/port, TCP service observations; firmware/freeSpace/temperature null |
| `list_remote` | connection, path | Entries: name, path, isDir, size (nullable), modified (nullable), raw |
| `create_remote_dir` | connection, path | void; FTP confirmation required |
| `delete_remote` | connection, path, isDir, confirmed | void; refuses root and unconfirmed requests |
| `upload_file` | connection, remotePath, localPath, transferId, overwrite | bytes, sha256, verification, remoteConfirmed, backupPath |
| `download_file` | connection, remotePath, localPath, transferId, overwrite | same |
| `network_diagnostics` | ip | port, label, reachable, latencyMs |
| `discover_consoles` | subnet (`192.168.1.0/24`), authorized | Potential devices: ip, services; requires explicit network owner authorization |
| `send_payload` | ip, port, localPath, confirmed | bytes, sha256, status = sent-unconfirmed |
| `get_history` | none | timestamp (Unix seconds), operation, ip, status, message |

`transfer-progress` event: `{ transferId, bytes, total: number|null, speedBytesPerSecond }`.

## Guarantees and limits

- Only RFC 1918 private IPv4 destinations; no public hosts, hostnames, loopback, IPv6 or Internet exposure. Loopback is used exclusively inside native protocol tests.
- FTP passive data connections always target the authorized control host, ignoring the server's advertised PASV address. Commands reject control characters and parent traversal.
- MLSD is preferred; UNIX LIST is fallback. Unsupported listings fail clearly. Symlinks and non-UNIX legacy listings are not supported yet.
- Binary transfers must receive FTP 226/250 completion. Remote SIZE is compared when supported. SHA-256 describes the bytes actually streamed; it is not a remotely verified checksum. End-to-end checksum verification needs a compatible server hash command or read-back implementation.
- Remote replacements first rename the original to `.vexoryn-backup-TIMESTAMP`, and fail if backup creation fails. These backups remain on the console and are visible in the file explorer. Downloads use a unique partial file and preserve existing destinations as backups before finalization.
- Partial downloads remain on disk if interrupted. Automatic REST resumption, cancellation and rename race protection are future work. No successful result is returned for interrupted or unconfirmed transfers.
- TCP availability does not identify the console, firmware, running payload or cheat engine. PS5 telemetry needs a documented compatible interface.
- Discovery is bounded to one explicitly authorized private /24 subnet, 254 hosts and one scan at a time. It returns TCP service candidates, not certified PS5 identities. It never starts automatically.
- ELF checks cover the 64-byte header: magic, class, byte order, version, x86-64 machine and header size, with a 64 MiB limit. This does not prove firmware compatibility. Sending is limited to raw TCP ELF loaders 9020/9021. The user must explicitly confirm compatibility. TCP completion never claims execution.
- No PKG installer protocol, homebrew enumeration, save access, cheat injection or firmware compatibility inference is fabricated.
- History is local JSON capped at 1,000 entries. It contains no credentials or local file paths; remote paths and LAN IPs describe real operations.

## Primary references

- [FTP RFC 959](https://www.rfc-editor.org/rfc/rfc959)
- [MLSD/SIZE RFC 3659](https://www.rfc-editor.org/rfc/rfc3659)
- [etaHEN official source](https://github.com/etaHEN/etaHEN): current README documents FTP 1337 and ELF loader 9021.
- [Alternative PS5 FTP server](https://github.com/manos555555/ps5-ftp-server): documents port 2121.
- [Tauri Windows installers](https://v2.tauri.app/distribute/windows-installer/): NSIS EXE and WiX MSI.

No third-party payload binaries are bundled.
