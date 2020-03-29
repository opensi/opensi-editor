extern crate quick_xml;
extern crate serde;

use std::fs::File;
use std::io::prelude::*;
use std::io::ErrorKind;
use std::path::Path;
use PartialEq;

use quick_xml::de::{from_str, DeError};
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Package {
    pub id: String,
    pub name: Option<String>,
    pub version: String,
    pub date: Option<String>,
    pub difficulty: Option<u8>,
    pub language: Option<String>,
    pub logo: Option<String>,
    pub publisher: Option<String>,
    pub restriciton: Option<String>,
    pub rounds: Rounds,
    pub tags: Option<Vec<String>>,
    pub info: Info,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Info {
    pub comments: Option<String>,
    pub extension: Option<String>,
    pub authors: Authors,
    pub sources: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Authors {
    #[serde(rename = "author", default)]
    pub authors: Vec<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Rounds {
    #[serde(rename = "round", default)]
    pub rounds: Vec<Round>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Round {
    pub name: String,
    #[serde(rename = "type", default)]
    pub variant: Option<String>,
    pub info: Option<Info>,
    pub themes: Themes,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Themes {
    #[serde(rename = "theme", default)]
    pub themes: Vec<Theme>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Theme {
    pub name: String,
    pub questions: Questions,
    pub info: Option<Info>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Questions {
    #[serde(rename = "question", default)]
    pub questions: Vec<Question>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Question {
    pub price: usize,
    pub scenario: Scenario,
    pub right: Right,
    pub wrong: Option<Wrong>,
    #[serde(rename = "type", default)]
    pub variant: Option<Variant>,
    pub info: Option<Info>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Variant {
    pub name: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Scenario {
    #[serde(rename = "atom", default)]
    pub atoms: Vec<Atom>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Right {
    #[serde(rename = "answer", default)]
    pub answers: Vec<Answer>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Wrong {
    #[serde(rename = "answer", default)]
    pub answers: Vec<Answer>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Answer {
    #[serde(rename = "$value")]
    pub body: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Atom {
    pub time: Option<f64>,
    #[serde(rename = "type", default)]
    pub variant: Option<String>,
    #[serde(rename = "$value")]
    pub body: Option<String>,
}

impl Package {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Package, std::io::Error> {
        let package_file = File::open(path)?;
        let mut zip = zip::ZipArchive::new(package_file)?;
        let mut xml = zip.by_name("content.xml")?;
        let mut contents = String::new();
        xml.read_to_string(&mut contents).unwrap();

        Package::parse(&contents)
            .map_err(|e| std::io::Error::new(ErrorKind::InvalidData, e))
    }

    fn parse(xml: &str) -> Result<Package, DeError> {
        let package: Package = from_str(xml)?;
        Result::Ok(package)
    }
}
