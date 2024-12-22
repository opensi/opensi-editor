use chrono::Datelike;
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};
use quick_xml::de::from_str;
use quick_xml::se::to_string;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::{Error, ErrorKind, Write};
use std::path::Path;
use std::{fs::File, io, io::Read};
use zip::write::FileOptions;
use zip::{CompressionMethod, ZipArchive, ZipWriter};

use super::components::{AtomV5, InfoV5, RoundV5};
use crate::package_trait::RoundContainer;
use crate::serde_impl;

/// Complete package structure with meta information about
/// the package and its tree of [`Question`].
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename = "package")]
pub struct PackageV5 {
    // attributes
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@version")]
    pub version: f32,
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(default, rename = "@date")]
    pub date: String,
    #[serde(default, rename = "@publisher")]
    pub publisher: String,
    #[serde(default, rename = "@difficulty")]
    pub difficulty: u8,
    #[serde(default, rename = "@language", skip_serializing_if = "String::is_empty")]
    pub language: String,
    #[serde(default, rename = "@logo", skip_serializing_if = "Option::is_none")]
    pub logo: Option<String>,
    #[serde(default, rename = "@restriction", skip_serializing_if = "String::is_empty")]
    pub restriction: String,
    #[serde(default, rename = "@xmlns")]
    pub namespace: String,

    // elements
    #[serde(default)]
    pub info: InfoV5,
    #[serde(default, with = "serde_impl::rounds")]
    pub rounds: Vec<RoundV5>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,

    // resources
    #[serde(skip)]
    pub resource: HashMap<Resource, Vec<u8>>,
}

/// # Creation of package.
impl PackageV5 {
    pub fn new() -> Self {
        let utc = chrono::Utc::now();

        Self {
            name: "Новый пакет вопросов".to_string(),
            version: 5.0,
            id: uuid::Uuid::new_v4().to_string(),
            date: format!("{}-{:0>2}-{:0>2}", utc.year(), utc.month(), utc.day()),
            publisher: String::new(),
            difficulty: 5,
            language: String::new(),
            logo: None,
            restriction: String::new(),
            namespace: String::new(),
            info: InfoV5::default(),
            rounds: vec![],
            tags: vec![],
            resource: HashMap::new(),
        }
    }
}

impl RoundContainer for PackageV5 {
    type Round = RoundV5;

    fn get_rounds(&self) -> &Vec<Self::Round> {
        &self.rounds
    }

    fn get_rounds_mut(&mut self) -> &mut Vec<Self::Round> {
        &mut self.rounds
    }
}

/// # IO and resource methods
impl PackageV5 {
    const CONTENT_TYPE_FILE_CONTENT: &'static str = r#"<?xml version="1.0" encoding="utf-8"?><Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types"><Default Extension="xml" ContentType="si/xml" /></Types>"""#;
    const XML_VERSION_ENCODING: &'static str = r#"<?xml version="1.0" encoding="utf-8"?>"#;
    const CONTROLS_ASCII_SET: &'static AsciiSet = &CONTROLS.add(b' ');

    pub fn get_resource(&self, atom: &AtomV5) -> Option<&Vec<u8>> {
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

        let body = atom.body.as_ref()?;
        let resource_name = utf8_percent_encode(body, Self::CONTROLS_ASCII_SET).to_string();
        let variant: &str = atom.variant.as_ref()?;

        let key = match variant {
            "voice" => Some(Resource::Audio(format!("Audio/{}", resource_name))),
            "image" => Some(Resource::Image(format!("Images/{}", resource_name))),
            "video" => Some(Resource::Video(format!("Video/{}", resource_name))),
            _ => None,
        };

        match key {
            Some(key) => self.resource.get(&key),
            None => None,
        }
    }

    // Expecting byte array of zip file
    pub fn from_zip_buffer(bytes: impl AsRef<[u8]>) -> Result<PackageV5, Error> {
        let cursor = io::Cursor::new(bytes);
        Self::get_package_from_zip(cursor)
    }

    pub fn open_zip_file(path: impl AsRef<Path>) -> Result<PackageV5, Error> {
        let package_file = File::open(path)?;
        Self::get_package_from_zip(package_file)
    }

    fn get_package_from_zip<T: Read + io::Seek>(source: T) -> Result<PackageV5, Error> {
        let mut zip_archive = ZipArchive::new(source)?;
        let mut resources = HashMap::new();

        for i in 0..zip_archive.len() {
            let mut zip_file = zip_archive.by_index(i)?;
            if zip_file.is_dir() {
                continue;
            }

            if let Some(filename) = zip_file.enclosed_name() {
                if let Some(filename) = filename.to_str() {
                    if filename == "content.xml" || filename == "[Content_Types].xml" {
                        continue;
                    }

                    match Self::get_resource_type(filename) {
                        Ok(key) => {
                            let mut value = Vec::new();
                            zip_file.read_to_end(&mut value)?;

                            resources.insert(key, value);
                        },
                        Err(_) => {
                            println!("Unknown resource type for {}", filename)
                        },
                    }
                }
            }
        }

        let mut content_file = zip_archive.by_name("content.xml")?;
        let mut contents = String::new();
        content_file.read_to_string(&mut contents)?;

        let package = from_str(&contents).map_err(|e| Error::new(ErrorKind::InvalidData, e));
        package.map(|p| PackageV5 { resource: resources, ..p })
    }

    fn get_resource_type(filename: &str) -> Result<Resource, Error> {
        if filename.starts_with("Audio") {
            Ok(Resource::Audio(filename.to_owned()))
        } else if filename.starts_with("Images") {
            Ok(Resource::Image(filename.to_owned()))
        } else if filename.starts_with("Video") {
            Ok(Resource::Video(filename.to_owned()))
        } else if filename.starts_with("Texts") {
            Ok(Resource::Texts(filename.to_owned()))
        } else {
            Err(Error::new(ErrorKind::InvalidData, "Unknown resource type"))
        }
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, Error> {
        let buffer = Vec::new();
        let cursor = io::Cursor::new(buffer);
        let mut zip = ZipWriter::new(cursor);

        // Define file options (e.g., compression method)
        let options = FileOptions::default().compression_method(CompressionMethod::Deflated);
        let xml = to_string(&self).map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
        zip.start_file("content.xml", options)?;
        zip.write_all(Self::XML_VERSION_ENCODING.as_ref())?;
        zip.write_all(&xml.into_bytes())?;

        zip.start_file("[Content_Types].xml", options)?;
        zip.write_all(Self::CONTENT_TYPE_FILE_CONTENT.as_ref())?;

        let resources = &self.resource;
        for (key, value) in resources.into_iter() {
            zip.start_file(key.extract_key(), options)?;
            zip.write_all(&value)?
        }

        let result = zip.finish()?;

        Ok(result.into_inner())
    }
}

/// Single resource handle inside [`Package`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Resource {
    Audio(String),
    Video(String),
    Image(String),
    Texts(String),
}

impl Resource {
    fn extract_key(&self) -> &str {
        match self {
            Resource::Audio(key)
            | Resource::Video(key)
            | Resource::Image(key)
            | Resource::Texts(key) => key,
        }
    }
}
