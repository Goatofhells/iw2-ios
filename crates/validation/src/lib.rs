use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{collections::HashSet, fs::{self, File}, io::{Read, Write}, path::{Component, Path}};

#[derive(Deserialize, Serialize)]
pub struct Entry {
    pub path: String,
    pub sha256: String,
}

#[derive(Deserialize, Serialize)]
pub struct Manifest {
    pub version: u32,
    pub files: Vec<Entry>,
}

impl Manifest {
    pub fn verify(&self, root: &Path) -> Result<usize, String> {
        if self.version != 1 || self.files.is_empty() {
            return Err("Unsupported installation: no approved file manifest is available".into());
        }
        let mut seen = HashSet::new();
        for item in &self.files {
            let path = Path::new(&item.path);
            if item.path.contains('\\') || item.path.contains(':') || path.is_absolute() || !path.components().all(|part| matches!(part, Component::Normal(_))) || !seen.insert(&item.path) {
                return Err("Invalid or duplicate manifest path".into());
            }
            if item.sha256.len() != 64 || !item.sha256.bytes().all(|chr| chr.is_ascii_hexdigit()) {
                return Err("Invalid SHA-256 digest".into());
            }
            let base = root.canonicalize().map_err(|err| err.to_string())?;
            let path = root.join(path).canonicalize().map_err(|err| err.to_string())?;
            if !path.starts_with(&base) { return Err("File escapes installation directory".into()); }
            let mut file = File::open(path).map_err(|err| err.to_string())?;
            let mut hash = Sha256::new();
            let mut data = [0u8; 65536];
            loop {
                let size = file.read(&mut data).map_err(|err| err.to_string())?;
                if size == 0 { break; }
                hash.update(&data[..size]);
            }
            if format!("{:x}", hash.finalize()) != item.sha256.to_lowercase() {
                return Err(format!("Corrupt or unsupported file: {}", item.path));
            }
        }
        Ok(self.files.len())
    }
}

pub fn extract(src: &Path, dst: &Path) -> Result<usize, String> {
    if dst.exists() { return Err("Import destination already exists".into()); }
    let file = File::open(src).map_err(|err| err.to_string())?;
    let mut zip = zip::ZipArchive::new(file).map_err(|err| err.to_string())?;
    if zip.len() > 20000 { return Err("Archive contains too many entries".into()); }
    let mut total = 0u64;
    let mut seen = HashSet::new();
    for idx in 0..zip.len() {
        let item = zip.by_index(idx).map_err(|err| err.to_string())?;
        let name = item.name();
        let path = item.enclosed_name().ok_or("Unsafe archive path")?;
        if name.contains('\\') || name.contains(':') || path.as_os_str().is_empty() || !seen.insert(path.to_string_lossy().to_lowercase()) || item.unix_mode().is_some_and(|mode| mode & 0o170000 == 0o120000) {
            return Err("Unsafe, duplicate, or symbolic-link archive entry".into());
        }
        total = total.checked_add(item.size()).ok_or("Archive size overflow")?;
        if total > 8 * 1024 * 1024 * 1024 { return Err("Archive exceeds 8 GiB limit".into()); }
    }
    fs::create_dir_all(dst).map_err(|err| err.to_string())?;
    let res = (|| {
        let mut used = 0u64;
        for idx in 0..zip.len() {
            let mut item = zip.by_index(idx).map_err(|err| err.to_string())?;
            let path = dst.join(item.enclosed_name().ok_or("Unsafe archive path")?);
            if item.is_dir() { fs::create_dir_all(&path).map_err(|err| err.to_string())?; continue; }
            fs::create_dir_all(path.parent().ok_or("Invalid archive path")?).map_err(|err| err.to_string())?;
            let mut file = File::options().write(true).create_new(true).open(path).map_err(|err| err.to_string())?;
            let mut data = [0u8; 65536];
            loop {
                let size = item.read(&mut data).map_err(|err| err.to_string())?;
                if size == 0 { break; }
                used += size as u64;
                if used > 8 * 1024 * 1024 * 1024 { return Err("Extracted data exceeds limit".into()); }
                file.write_all(&data[..size]).map_err(|err| err.to_string())?;
            }
        }
        Ok(zip.len())
    })();
    if res.is_err() { let _ = fs::remove_dir_all(dst); }
    res
}
