extern crate quick_xml;
extern crate serde;

use std::fs::File;
use std::io::prelude::*;
use std::io::ErrorKind;
use std::path::Path;
use PartialEq;

use quick_xml::de::{from_reader, from_str};
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

#[derive(Debug)]
pub enum Resource {
    Audio(std::path::PathBuf),
    Video(std::path::PathBuf),
    Image(std::path::PathBuf),
}

use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};
const FRAGMENT: & AsciiSet = &CONTROLS.add(b' ');

impl Atom {
    pub fn get_resource(&self, filename: &str) -> Option<Resource> { 
        // Body a.k.a "resource name" as stated by the documentation begins
        // with '@' in package to distinguish plain text and links to
        // resources, thats why we need manually trim '@' from begining.
        // It also percent-encoded so we need to decode this.
        let body = self.body.as_ref()?;
        let resource_name = &utf8_percent_encode(&body, FRAGMENT).to_string()[1..];
        let tmp = std::env::temp_dir().join(filename); // костылик
        let variant = self.variant.as_ref()?;
        let variant: &str= &variant; // :)
        
        match variant {
            "voice" => Some(Resource::Audio(tmp.join("Audio").join(resource_name))),
            "image" => Some(Resource::Image(tmp.join("Images").join(resource_name))),
            "video" => Some(Resource::Video(tmp.join("Video").join(resource_name))),
            _ => None,
        }
    }
}

impl Package {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Package, std::io::Error> {
        let package_file = File::open(path)?;
        let mut zip = zip::ZipArchive::new(package_file)?;
        let mut xml = zip.by_name("content.xml")?;
        let mut contents = String::new();
        xml.read_to_string(&mut contents).unwrap();

        from_str(&contents).map_err(|e| std::io::Error::new(ErrorKind::InvalidData, e))
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
            let mut zipfile_path = zipfile.sanitized_name();
            let encoded_name = zipfile_path.file_name().unwrap().to_str().unwrap().to_string();

            if encoded_name.starts_with("@") {
                zipfile_path.set_file_name(&encoded_name[1..]);
            } else {
                zipfile_path.set_file_name(encoded_name)
            };

            if let Some(parent) = zipfile_path.parent() {
                let parent_path = tmp.join(parent);
                if !parent.exists() {
                    std::fs::create_dir_all(&parent_path)?;
                }
            }

            let path = &tmp.join(zipfile_path);
            let mut fsfile = std::fs::File::create(&path)?;
            std::io::copy(&mut zipfile, &mut fsfile)?;
        }
        Ok(tmp.join("content.xml"))
    }
}
