#![allow(dead_code)]

use std::io::ErrorKind;
use std::path::Path;
use std::{fs::File, io::Read};

use quick_xml::de::from_str;
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
use serde::de::Error;
use zip::ZipArchive;

const CONTROLS_ASCII_SET: &AsciiSet = &CONTROLS.add(b' ');

impl Atom {
    pub fn get_resource(&self, filename: &str) -> Option<Resource> {
        // Atom xml content or ("resource name" as stated in official docs) begins
        // with '@' in package to distinguish plain text and links to
        // resources. This is how it looks like in package:
        //
        // ```xml
        // <atom>Откуда данный опенинг ?</atom>
        // <atom type="voice">@3.mp3</atom>
        // ```
        // All links is just a part of filename, so we want to trim '@' from
        // beginning to make our life easier while working with files from the pack.
        // It also percent-encoded so we need to decode links.

        let body = self.body.as_ref()?;
        let resource_name = &utf8_percent_encode(body, CONTROLS_ASCII_SET).to_string();
        let tmp = std::env::temp_dir().join(filename);
        let variant: &str = self.variant.as_ref()?;

        match variant {
            "voice" => Some(Resource::Audio(tmp.join("Audio").join(resource_name))),
            "image" => Some(Resource::Image(tmp.join("Images").join(resource_name))),
            "video" => Some(Resource::Video(tmp.join("Video").join(resource_name))),
            _ => None,
        }
    }
}

impl Package {

    // Expecting byte array of zip file
    pub fn from_zip_buffer(bytes: impl AsRef<[u8]>) -> Result<Package, io::Error> {
        let cursor = io::Cursor::new(bytes);
        Self::get_package_from_zip(cursor)
    }

    pub fn open_zip_file(path: impl AsRef<Path>) -> Result<Package, io::Error> {
        let package_file = File::open(path)?;
        Self::get_package_from_zip(package_file)
    }

    fn get_package_from_zip<T: Read + io::Seek>(source: T) -> Result<Package, io::Error> {
        let mut zip_archive = ZipArchive::new(source)?;

        let mut xml = zip_archive.by_name("content.xml")?;
        let mut contents = String::new();
        xml.read_to_string(&mut contents)?;

        from_str(&contents).map_err(|e| io::Error::new(ErrorKind::InvalidData, e))
    }

    // TODO: figure out how to do extraction on wasm
    // pub async fn open_with_extraction(path: impl AsRef<Path>) -> Result<Package, std::io::Error> {
    //     let package_name = path.as_ref().file_name().unwrap().to_str().unwrap();
    //     let package_file = File::open(&path)?;
    //     let mut zip = zip::ZipArchive::new(package_file)?;

    //     let xml_path = Self::extract_package(package_name, &mut zip)?;
    //     let xml = std::fs::File::open(xml_path)?;
    //     let reader = std::io::BufReader::new(xml);
    //     from_reader(reader).map_err(|e| std::io::Error::new(ErrorKind::InvalidData, e))
    // }

    // /// Extract package internals into `/tmp/{package_name}/`. Return `PathBuf`
    // /// to `content.xml`.
    // ///
    // /// # Arguments
    // /// * `package_name` - package name to which the package will be extracted
    // /// * `zip` - unpacked archive
    // async fn extract_package(
    //     package_name: &str,
    //     zip: &mut zip::ZipArchive<File>,
    // ) -> Result<std::path::PathBuf, std::io::Error> {
    //         let tmp = std::env::temp_dir().join(package_name);

    //         for i in 0..zip.len() {
    //             let mut zipfile = zip.by_index(i)?;
    //             let mut zipfile_path = zipfile.sanitized_name();
    //             let encoded_name = zipfile_path.file_name().unwrap().to_str().unwrap().to_string();

    //             if encoded_name.starts_with("@") {
    //                 zipfile_path.set_file_name(&encoded_name[1..]);
    //             } else {
    //                 zipfile_path.set_file_name(encoded_name)
    //             };

    //             if let Some(parent) = zipfile_path.parent() {
    //                 let parent_path = tmp.join(parent);
    //                 if !parent.exists() {
    //                     std::fs::create_dir_all(&parent_path)?;
    //                 }
    //             }

    //             let path = &tmp.join(zipfile_path);
    //             let mut fsfile = std::fs::File::create(&path)?;
    //             std::io::copy(&mut zipfile, &mut fsfile)?;
    //         }
    //         Ok(tmp.join("content.xml"))
    // }
}
