use launcher::config;
use std::ffi::{CStr, CString};

#[test]
fn settings() {
    let mut cfg = config { fps: 30, scale: 0.75, cache: 128 };
    assert!(cfg.valid());
    cfg.fps = 120;
    assert!(!cfg.valid());
    cfg.fps = 60;
    cfg.scale = f32::NAN;
    assert!(!cfg.valid());
    cfg.scale = 0.4;
    assert!(!cfg.valid());
    cfg.scale = 1.0;
    cfg.cache = 513;
    assert!(!cfg.valid());
    assert_eq!(diag::budget(2048, 512), 256);
    assert_eq!(diag::budget(1024, 1), 16);
    assert_eq!(diag::budget(8192, 2048), 512);
}

#[test]
fn bridge() {
    for (data, ok) in [
        (r#"{"op":"config","config":{"fps":30,"scale":0.75,"cache":128},"ram":2048}"#, true),
        (r#"{"op":"verify","src":"."}"#, false),
        (r#"{"op":"unknown"}"#, false),
        ("invalid", false),
    ] {
        let data = CString::new(data).unwrap();
        unsafe {
            let ptr = launcher::iw4_run(data.as_ptr());
            let doc: serde_json::Value = serde_json::from_str(CStr::from_ptr(ptr).to_str().unwrap()).unwrap();
            assert_eq!(doc["ok"], ok);
            launcher::iw4_free(ptr);
        }
    }
    unsafe {
        let ptr = launcher::iw4_run(std::ptr::null());
        assert!(!ptr.is_null());
        launcher::iw4_free(ptr);
        launcher::iw4_free(std::ptr::null_mut());
    }
}
