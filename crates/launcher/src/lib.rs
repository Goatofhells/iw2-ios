use serde::Deserialize;
use std::{ffi::{c_char, CStr, CString}, path::Path};

#[derive(Deserialize)]
pub struct config {
    pub fps: u32,
    pub scale: f32,
    pub cache: u64,
}

impl config {
    pub fn valid(&self) -> bool {
        [30, 60].contains(&self.fps) && self.scale.is_finite() && (0.5..=1.0).contains(&self.scale) && (16..=512).contains(&self.cache)
    }
}

#[derive(Deserialize)]
struct request {
    op: String,
    src: Option<String>,
    dst: Option<String>,
    ram: Option<u64>,
    config: Option<config>,
}

#[no_mangle]
pub unsafe extern "C" fn iw4_run(data: *const c_char) -> *mut c_char {
    let res = std::panic::catch_unwind(|| -> Result<String, String> {
        if data.is_null() { return Err("Missing request".into()); }
        let text = CStr::from_ptr(data).to_str().map_err(|err| err.to_string())?;
        let req: request = serde_json::from_str(text).map_err(|err| err.to_string())?;
        match req.op.as_str() {
            "import" => {
                let src = req.src.ok_or("Missing archive")?;
                let dst = req.dst.ok_or("Missing destination")?;
                let size = validation::extract(Path::new(&src), Path::new(&dst))?;
                Ok(format!("Imported {size} entries. Installation remains unsupported; no approved MW2 manifest is shipped."))
            }
            "verify" => {
                let root = req.src.ok_or("Missing installation")?;
                let data = include_str!("../../../game/manifest.json");
                let doc: validation::manifest = serde_json::from_str(data).map_err(|err| err.to_string())?;
                let size = doc.verify(Path::new(&root))?;
                Ok(format!("Verified {size} files; runtime unavailable"))
            }
            "config" => {
                let cfg = req.config.ok_or("Missing settings")?;
                if !cfg.valid() { return Err("Invalid game configuration".into()); }
                Ok(format!("Cache budget: {} MiB", diag::budget(req.ram.unwrap_or(2048), cfg.cache)))
            }
            _ => Err("Unsupported operation".into()),
        }
    }).unwrap_or_else(|_| Err("Launcher service failed".into()));
    let text = match res {
        Ok(text) => serde_json::json!({"ok": true, "message": text}),
        Err(text) => serde_json::json!({"ok": false, "message": text}),
    };
    CString::new(text.to_string()).unwrap().into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn iw4_free(data: *mut c_char) {
    if !data.is_null() { drop(CString::from_raw(data)); }
}
