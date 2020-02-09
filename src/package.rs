extern crate quick_xml;
extern crate serde;

use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use std::io::ErrorKind;
use PartialEq;

use quick_xml::de::{from_str, DeError};
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Package {
    id: String,
    name: String,
    version: String,
    date: Option<String>,
    difficulty: Option<u8>,
    language: Option<String>,
    logo: Option<String>,
    publisher: Option<String>,
    restriciton: Option<String>,
    rounds: Vec<Round>,
    tags: Vec<String>,
    info: Info,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Info {
    comments: String,
    extension: String,
    authors: Vec<String>,
    sources: Vec<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Round {
    name: String,
    variant: String, // fixme: original name "type"
    info: Info,
    themes: Vec<Theme>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Theme {
    name: String,
    questions: Vec<Question>,
    info: Info,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Question {
    price: usize,
    scenario: Vec<Atom>,
    right: Vec<String>,
    wrong: Vec<String>,
    variant: String, // fixme: original name "type"
    info: Info,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Atom {
    time: Option<f64>,
    variant: String, // fixme: original name "type"
}


impl Package {
    pub fn open(path: &Path) -> Result<Package, std::io::Error> {
        let package_file = File::open(path)?;
        let mut zip = zip::ZipArchive::new(package_file)?;
        let mut xml = zip.by_name("content.xml")?;
        let mut contents = String::new();
        xml.read_to_string(&mut contents).unwrap();

        match Package::parse(&contents) {
            Ok(package) => Ok(package),
            Err(e) => {
                let error = std::io::Error::new(ErrorKind::InvalidData, e);
                Err(error)
            }
        }
    }

    fn parse(xml: &String) -> Result<Package, DeError> {
        let package: Package = from_str(xml)?;
        return Result::Ok(package);
    }
}
