use std::{fs, io::Write};
use validation::{entry, manifest};

#[test]
fn hashes() {
    let root = std::env::temp_dir().join(format!("iw4-hash-{}", std::process::id()));
    fs::create_dir_all(&root).unwrap();
    fs::write(root.join("test.bin"), b"abc").unwrap();
    let mut doc = manifest { version: 1, files: vec![entry {
        path: "test.bin".into(),
        sha256: "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad".into(),
    }] };
    assert_eq!(doc.verify(&root).unwrap(), 1);
    fs::write(root.join("test.bin"), b"bad").unwrap();
    assert!(doc.verify(&root).unwrap_err().contains("Corrupt"));
    doc.files[0].path = "../escape".into();
    assert!(doc.verify(&root).is_err());
    doc.files[0].path = "test.bin".into();
    doc.files[0].sha256 = "invalid".into();
    assert!(doc.verify(&root).is_err());
    doc.version = 2;
    assert!(doc.verify(&root).is_err());
    doc.files.clear();
    doc.version = 1;
    assert!(doc.verify(&root).is_err());
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn archives() {
    let root = std::env::temp_dir().join(format!("iw4-zip-{}", std::process::id()));
    fs::create_dir_all(&root).unwrap();
    for (idx, name) in ["safe/data.bin", "../escape.bin", "/escape.bin", "safe\\escape.bin", "C:escape.bin"].iter().enumerate() {
        let src = root.join(format!("{idx}.zip"));
        let dst = root.join(format!("out{idx}"));
        let file = fs::File::create(&src).unwrap();
        let mut zip = zip::ZipWriter::new(file);
        zip.start_file(*name, zip::write::SimpleFileOptions::default()).unwrap();
        zip.write_all(b"original fixture").unwrap();
        zip.finish().unwrap();
        let res = validation::extract(&src, &dst);
        if idx == 0 {
            assert_eq!(res.unwrap(), 1);
            assert_eq!(fs::read(dst.join("safe/data.bin")).unwrap(), b"original fixture");
            assert!(validation::extract(&src, &dst).is_err());
        } else {
            assert!(res.is_err(), "Accepted {name}");
            assert!(!dst.exists());
        }
    }
    let src = root.join("link.zip");
    let file = fs::File::create(&src).unwrap();
    let mut zip = zip::ZipWriter::new(file);
    zip.add_symlink("link", "../outside", zip::write::SimpleFileOptions::default()).unwrap();
    zip.finish().unwrap();
    assert!(validation::extract(&src, &root.join("links")).is_err());
    let src = root.join("case.zip");
    let file = fs::File::create(&src).unwrap();
    let mut zip = zip::ZipWriter::new(file);
    for name in ["File", "file"] {
        zip.start_file(name, zip::write::SimpleFileOptions::default()).unwrap();
        zip.write_all(b"x").unwrap();
    }
    zip.finish().unwrap();
    assert!(validation::extract(&src, &root.join("case")).is_err());
    fs::write(root.join("bad.zip"), b"invalid").unwrap();
    assert!(validation::extract(&root.join("bad.zip"), &root.join("bad")).is_err());
    fs::remove_dir_all(root).unwrap();
}
