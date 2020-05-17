extern crate quick_xml;
extern crate serde;

use std::fs::File;
use std::io::prelude::*;
use std::io::ErrorKind;
use std::path::Path;
use PartialEq;

use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};
use quick_xml::de::{from_reader, from_str, DeError};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize, PartialEq)]
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

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Info {
    pub comments: Option<String>,
    pub extension: Option<String>,
    pub authors: Authors,
    pub sources: Option<Vec<String>>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Authors {
    #[serde(rename = "author", default)]
    pub authors: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Rounds {
    #[serde(rename = "round", default)]
    pub rounds: Vec<Round>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Round {
    pub name: String,
    #[serde(rename = "type", default)]
    pub variant: Option<String>,
    pub info: Option<Info>,
    pub themes: Themes,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Themes {
    #[serde(rename = "theme", default)]
    pub themes: Vec<Theme>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Theme {
    pub name: String,
    pub questions: Questions,
    pub info: Option<Info>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Questions {
    #[serde(rename = "question", default)]
    pub questions: Vec<Question>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Question {
    pub price: usize,
    pub scenario: Scenario,
    pub right: Right,
    pub wrong: Option<Wrong>,
    #[serde(rename = "type", default)]
    pub variant: Option<Variant>,
    pub info: Option<Info>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Variant {
    pub name: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Scenario {
    #[serde(rename = "atom", default)]
    pub atoms: Vec<Atom>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Right {
    #[serde(rename = "answer", default)]
    pub answers: Vec<Answer>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Wrong {
    #[serde(rename = "answer", default)]
    pub answers: Vec<Answer>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Answer {
    #[serde(rename = "$value")]
    pub body: Option<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Atom {
    pub time: Option<f64>,
    #[serde(rename = "type", default)]
    pub variant: Option<String>,
    #[serde(rename = "$value")]
    pub body: Option<String>,
}

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ');

impl Package {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Package, std::io::Error> {
        let package_file = File::open(path)?;
        let mut zip = zip::ZipArchive::new(package_file)?;
        let mut xml = zip.by_name("content.xml")?;
        let mut contents = String::new();
        xml.read_to_string(&mut contents).unwrap();

        Package::parse(&contents).map_err(|e| std::io::Error::new(ErrorKind::InvalidData, e))
    }

    fn parse(xml: &str) -> Result<Package, DeError> {
        let package: Package = from_str(xml)?;
        Result::Ok(package)
    }

    pub fn open_with_extraction<P: AsRef<Path>>(path: P) -> Result<Package, std::io::Error> {
        let package_name = path.as_ref().file_name().unwrap().to_str().unwrap();
        let package_file = File::open(&path)?;
        let mut zip = zip::ZipArchive::new(package_file)?;

        let xml_path = Self::extract_package(package_name, &mut zip)?;
        let xml = std::fs::File::open(xml_path)?;
        let reader = std::io::BufReader::new(xml);
        from_reader(reader).map_err(|e| std::io::Error::new(ErrorKind::InvalidData, e))
    }

    /// Extract package internals into `/tmp/{package_name}/`. Return `PathBuf`
    /// to `content.xml`.
    ///
    /// # Arguments
    /// * `package_name` - package name to which the package will be extracted
    /// * `zip` - unpacked archive
    fn extract_package(
        package_name: &str,
        zip: &mut zip::ZipArchive<File>,
    ) -> Result<std::path::PathBuf, std::io::Error> {
        let tmp = std::env::temp_dir().join(package_name);

        for i in 0..zip.len() {
            let mut zipfile = zip.by_index(i)?;
            let relative_file_path = zipfile.sanitized_name();
            let file_name = &relative_file_path.file_name().unwrap().to_str().unwrap();
            let encoded_name = utf8_percent_encode(&file_name, FRAGMENT).to_string();
            let encoded_name: &str = if encoded_name.starts_with("@") {
                &encoded_name[1..]
            } else {
                &encoded_name
            };

            let absolute_file_path: std::path::PathBuf = if let Some(parent) =
                relative_file_path.parent()
            {
                let mut absolute_parent_path: std::path::PathBuf = [&tmp, parent].iter().collect();
                if !absolute_parent_path.exists() {
                    std::fs::create_dir_all(&absolute_parent_path).unwrap();
                }

                absolute_parent_path.push(encoded_name.to_string());
                absolute_parent_path
            } else {
                let mut absolute_path = std::path::PathBuf::from(&tmp);
                absolute_path.push(encoded_name.to_string());
                absolute_path
            };

            let mut fsfile = std::fs::File::create(&absolute_file_path)?;
            std::io::copy(&mut zipfile, &mut fsfile)?;
        }
        let content = std::path::PathBuf::from("content.xml");
        Ok([tmp, content].iter().collect())
    }
}
