use chrono::Datelike;
use quick_xml::de::from_str;
use quick_xml::se::to_string;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::{Error, ErrorKind, Write};
use std::path::Path;
use std::{fs::File, io, io::Read};
use zip::write::SimpleFileOptions;
use zip::{CompressionMethod, ZipArchive, ZipWriter};

use super::ResourceIdv4;
use super::atom::Atomv4;
use super::components::{Infov4, Roundv4};
use crate::package_trait::RoundContainer;
use crate::serde_impl;

/// Complete package structure with meta information about
/// the package and its tree of [`Question`].
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename = "package")]
pub struct Packagev4 {
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
    pub info: Infov4,
    #[serde(default, with = "serde_impl::rounds")]
    pub rounds: Vec<Roundv4>,
    #[serde(default, with = "serde_impl::tags", skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,

    // resources
    #[serde(skip)]
    pub resource: HashMap<ResourceIdv4, Vec<u8>>,
}

/// # Creation of package.
impl Packagev4 {
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
            info: Infov4::default(),
            rounds: vec![],
            tags: vec![],
            resource: HashMap::new(),
        }
    }
}

impl RoundContainer for Packagev4 {
    type Round = Roundv4;

    fn get_rounds(&self) -> &Vec<Self::Round> {
        &self.rounds
    }

    fn get_rounds_mut(&mut self) -> &mut Vec<Self::Round> {
        &mut self.rounds
    }
}

/// # IO and resource methods
impl Packagev4 {
    const CONTENT_TYPE_FILE_CONTENT: &'static str = r#"<?xml version="1.0" encoding="utf-8"?><Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types"><Default Extension="xml" ContentType="si/xml" /></Types>"""#;
    const XML_VERSION_ENCODING: &'static str = r#"<?xml version="1.0" encoding="utf-8"?>"#;

    pub fn get_resource(&self, atom: &Atomv4) -> Option<&Vec<u8>> {
        let resource = atom.resource()?;
        self.resource.get(&resource)
    }

    // Expecting byte array of zip file
    pub fn from_zip_buffer(bytes: impl AsRef<[u8]>) -> Result<Packagev4, Error> {
        let cursor = io::Cursor::new(bytes);
        Self::get_package_from_zip(cursor)
    }

    pub fn open_zip_file(path: impl AsRef<Path>) -> Result<Packagev4, Error> {
        let package_file = File::open(path)?;
        Self::get_package_from_zip(package_file)
    }

    fn get_package_from_zip<T: Read + io::Seek>(source: T) -> Result<Packagev4, Error> {
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

                    match ResourceIdv4::try_new(filename) {
                        Some(key) => {
                            let mut value = Vec::new();
                            zip_file.read_to_end(&mut value)?;

                            resources.insert(key, value);
                        },
                        None => {
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
        package.map(|p| Packagev4 { resource: resources, ..p })
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, Error> {
        let buffer = Vec::new();
        let cursor = io::Cursor::new(buffer);
        let mut zip = ZipWriter::new(cursor);

        // Define file options (e.g., compression method)
        let options = SimpleFileOptions::default().compression_method(CompressionMethod::Deflated);
        let xml = to_string(&self).map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
        zip.start_file("content.xml", options)?;
        zip.write_all(Self::XML_VERSION_ENCODING.as_ref())?;
        zip.write_all(&xml.into_bytes())?;

        zip.start_file("[Content_Types].xml", options)?;
        zip.write_all(Self::CONTENT_TYPE_FILE_CONTENT.as_ref())?;

        let resources = &self.resource;
        for (key, value) in resources.into_iter() {
            zip.start_file(key.path(), options)?;
            zip.write_all(&value)?
        }

        let result = zip.finish()?;

        Ok(result.into_inner())
    }
}
