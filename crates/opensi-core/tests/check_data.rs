use opensi_core::prelude::*;

use std::fs;
use std::path::PathBuf;

const PACKS_DIR: &str = "tests/data";

#[test]
fn open_packs() {
    for pack in get_packs() {
        let package = Package::open_zip_file(&pack);
        let package_name = pack.file_name().unwrap().to_str().unwrap();

        assert!(
            package.is_ok(),
            "Can't open package {}, error is {}",
            package_name,
            package.err().unwrap()
        );
    }
}

#[test]
fn resave_test() {
    for pack in get_packs() {
        let package_original = Package::open_zip_file(&pack).expect("Pack is not found");
        let bytes = &package_original.to_bytes().expect("Can't serialize package to bytes");
        let package_resaved =
            Package::from_zip_buffer(bytes).expect("Can't read package from buffer");

        let package_name = pack.file_name().unwrap().to_str().unwrap();

        assert_eq!(
            package_original, package_resaved,
            "Package {} resaving produced different results",
            package_name
        );
    }
}

fn get_packs() -> Vec<PathBuf> {
    let mut packs = Vec::new();

    if let Ok(entries) = fs::read_dir(PACKS_DIR) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file() && path.extension().unwrap_or_default() == "siq" {
                    packs.push(path);
                }
            }
        }
    }

    packs
}
