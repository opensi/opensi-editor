use opensi_core::prelude::Package;

const PATH: &str = "tests/data/slamjam2.siq";

#[test]
fn open_pack() {
    let package = Package::open_zip_file(PATH);
    assert_eq!(package.is_ok(), true);
}

#[test]
fn read_package_name() {
    let package = Package::open_zip_file(PATH).expect("pack is not found");
    assert_eq!(package.name, "SLAM JAM 2".to_owned());
}

#[test]
fn resave_test() {
    let package_original = Package::open_zip_file(PATH).expect("Pack is not found");
    let bytes = &package_original.to_bytes().expect("Can't serialize package to bytes");
    let package_resaved = Package::from_zip_buffer(bytes).expect("Can't read package from buffer");

    assert_eq!(package_original, package_resaved);
}
