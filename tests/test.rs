extern crate opensi;
use std::path::Path;
use std::env;

#[test]
fn open_pack() {
    let package = opensi::open("tests/data/slamjam2.siq");
    assert_eq!(package.is_ok(), true);
}