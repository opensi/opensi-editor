mod package;
use package::*;
use std::path::Path;

pub fn open<P: AsRef<Path>>(p: P) -> Result<Package, std::io::Error> {
    package::Package::open(p)
}