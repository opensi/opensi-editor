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

use crate::components::{Atom, Info, Question, Round, Theme};
use crate::node::{PackageNode, QuestionIdx, RoundIdx, ThemeIdx};
use crate::serde_impl;

/// Complete package structure with meta information about
/// the package and its tree of [`Question`].
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename = "package")]
pub struct Package {
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
    pub info: Info,
    #[serde(default, with = "serde_impl::rounds")]
    pub rounds: Vec<Round>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,

    // resources
    #[serde(skip)]
    pub resource: HashMap<Resource, Vec<u8>>,
}

/// # Creation of package.
impl Package {
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
            info: Info::default(),
            rounds: vec![],
            tags: vec![],
            resource: HashMap::new(),
        }
    }
}

/// # [`PackageNode`]-based methods
impl Package {
    /// Clone a node and push it afterwards.
    pub fn duplicate_node(&mut self, node: PackageNode) {
        match node {
            PackageNode::Round(idx) => {
                self.duplicate_round(idx);
            },
            PackageNode::Theme(idx) => {
                self.duplicate_theme(idx);
            },
            PackageNode::Question(idx) => {
                self.duplicate_question(idx);
            },
        };
    }

    /// Create a new default node and push it.
    pub fn allocate_node(&mut self, node: PackageNode) {
        match node {
            PackageNode::Round(_) => {
                self.allocate_round();
            },
            PackageNode::Theme(idx) => {
                self.allocate_theme(idx.parent());
            },
            PackageNode::Question(idx) => {
                self.allocate_question(idx.parent());
            },
        };
    }

    /// Remove a single node.
    pub fn remove_node(&mut self, node: PackageNode) {
        match node {
            PackageNode::Round(idx) => {
                self.remove_round(idx);
            },
            PackageNode::Theme(idx) => {
                self.remove_theme(idx);
            },
            PackageNode::Question(idx) => {
                self.remove_question(idx);
            },
        };
    }
}

/// # Methods around [`Round`]
impl Package {
    /// Get [`Round`] by index.
    pub fn get_round(&self, idx: impl Into<RoundIdx>) -> Option<&Round> {
        let idx = idx.into();
        self.rounds.get(*idx)
    }

    /// Get mutable [`Round`] by index.
    pub fn get_round_mut(&mut self, idx: impl Into<RoundIdx>) -> Option<&mut Round> {
        let idx = idx.into();
        self.rounds.get_mut(*idx)
    }

    /// Remove [`Round`] by index and return it.
    pub fn remove_round(&mut self, idx: impl Into<RoundIdx>) -> Option<Round> {
        let idx = idx.into();
        if *idx >= self.rounds.len() {
            return None;
        }
        self.rounds.remove(*idx).into()
    }

    /// Push a new [`Round`] to the end of the package and
    /// return a reference to it.
    pub fn push_round(&mut self, round: Round) -> &mut Round {
        self.rounds.push(round);
        self.rounds.last_mut().unwrap()
    }

    /// Insert a new [`Round`] at position and return a
    /// reference to it.
    pub fn insert_round(&mut self, idx: impl Into<RoundIdx>, round: Round) -> Option<&mut Round> {
        let idx = idx.into();
        if *idx > self.rounds.len() {
            return None;
        }
        self.rounds.insert(*idx, round);
        Some(&mut self.rounds[*idx])
    }

    /// Clone a [`Round`], push it afterwards and return
    /// a reference to the new round.
    pub fn duplicate_round(&mut self, idx: impl Into<RoundIdx>) -> Option<&mut Round> {
        let idx = idx.into();
        self.get_round(idx).cloned().and_then(|round| self.insert_round(idx.next(), round))
    }

    /// Create a new default [`Round`], push it and return
    /// a reference to it.
    pub fn allocate_round(&mut self) -> &mut Round {
        self.push_round(Round::default())
    }

    /// Check if [`Round`] by index exist.
    pub fn contains_round(&self, idx: impl Into<RoundIdx>) -> bool {
        let idx = idx.into();
        *idx < self.rounds.len()
    }

    /// Return amount of [`Round`]s in this package.
    pub fn count_rounds(&self) -> usize {
        self.rounds.len()
    }
}

/// # Methods around [`Theme`]
impl Package {
    /// Get [`Theme`] in [`Round`] by indices.
    pub fn get_theme(&self, idx: impl Into<ThemeIdx>) -> Option<&Theme> {
        let idx = idx.into();
        self.get_round(idx.parent()).and_then(|round| round.themes.get(*idx))
    }

    /// Get mutable [`Theme`] in [`Round`] by indices.
    pub fn get_theme_mut(&mut self, idx: impl Into<ThemeIdx>) -> Option<&mut Theme> {
        let idx = idx.into();
        self.get_round_mut(idx.round_index).and_then(|round| round.themes.get_mut(*idx))
    }

    /// Check if [`Theme`] by indices exist.
    pub fn contains_theme(&self, idx: impl Into<ThemeIdx>) -> bool {
        let idx = idx.into();
        self.get_round(idx.parent()).map(|round| *idx < round.themes.len()).unwrap_or_default()
    }

    /// Remove [`Theme`] in [`Round`] by indices.
    pub fn remove_theme(&mut self, idx: impl Into<ThemeIdx>) -> Option<Theme> {
        let idx = idx.into();
        let round = self.get_round_mut(idx.round_index)?;
        if *idx >= round.themes.len() {
            return None;
        }
        round.themes.remove(*idx).into()
    }

    /// Push a new [`Theme`] to the end of the [`Round`] and
    /// return a reference to it.
    pub fn push_theme(&mut self, idx: impl Into<RoundIdx>, theme: Theme) -> Option<&mut Theme> {
        let idx = idx.into();
        let round = self.get_round_mut(idx)?;
        round.themes.push(theme);
        round.themes.last_mut().unwrap().into()
    }

    /// Insert a new [`Theme`] at position and return a
    /// reference to it.
    pub fn insert_theme(&mut self, idx: impl Into<ThemeIdx>, theme: Theme) -> Option<&mut Theme> {
        let idx = idx.into();
        let round = self.get_round_mut(idx.round_index)?;
        if *idx > round.themes.len() {
            return None;
        }
        round.themes.insert(*idx, theme);
        Some(&mut round.themes[*idx])
    }

    /// Clone a [`Theme`], push it afterwards and return
    /// a reference to the new theme.
    pub fn duplicate_theme(&mut self, idx: impl Into<ThemeIdx>) -> Option<&mut Theme> {
        let idx = idx.into();
        self.get_theme(idx).cloned().and_then(|theme| self.insert_theme(idx.next(), theme))
    }

    /// Create a new default [`Theme`], push it to the [`Round`]
    /// and return a reference to it.
    pub fn allocate_theme(&mut self, idx: impl Into<RoundIdx>) -> Option<&mut Theme> {
        let idx = idx.into();
        self.push_theme(idx, Theme::default())
    }

    /// Return amount of [`Theme`]s in a [`Round`].
    pub fn count_themes(&self, idx: impl Into<RoundIdx>) -> usize {
        self.get_round(idx).map(|round| round.themes.len()).unwrap_or_default()
    }
}

/// # Methods around [`Theme`].
impl Package {
    /// Get [`Question`] in [`Theme`] in [`Round`] by indices.
    pub fn get_question(&self, idx: impl Into<QuestionIdx>) -> Option<&Question> {
        let idx = idx.into();
        self.get_theme(idx.parent()).and_then(|theme| theme.questions.get(*idx))
    }

    /// Get mutable [`Question`] in [`Theme`] in [`Round`] by indices.
    pub fn get_question_mut(&mut self, idx: impl Into<QuestionIdx>) -> Option<&mut Question> {
        let idx = idx.into();
        self.get_theme_mut(idx.parent()).and_then(|theme| theme.questions.get_mut(*idx))
    }

    /// Check if [`Question`] by indices exist.
    pub fn contains_question(&self, idx: impl Into<QuestionIdx>) -> bool {
        let idx = idx.into();
        self.get_theme(idx.parent()).map(|theme| *idx < theme.questions.len()).unwrap_or_default()
    }

    /// Remove [`Question`] in [`Theme`] in [`Round`] by indices.
    pub fn remove_question(&mut self, idx: impl Into<QuestionIdx>) -> Option<Question> {
        let idx = idx.into();
        let theme = self.get_theme_mut(idx.parent())?;
        if *idx >= theme.questions.len() {
            return None;
        }
        theme.questions.remove(*idx).into()
    }

    /// Push a new [`Question`] to the end of the [`Theme`] in [`Round`]
    /// and return a reference to it.
    pub fn push_question(
        &mut self,
        idx: impl Into<ThemeIdx>,
        question: Question,
    ) -> Option<&mut Question> {
        let idx = idx.into();
        let theme = self.get_theme_mut(idx)?;
        theme.questions.push(question);
        theme.questions.last_mut().unwrap().into()
    }

    /// Insert a new [`Question`] at position and return a
    /// reference to it.
    pub fn insert_question(
        &mut self,
        idx: impl Into<QuestionIdx>,
        question: Question,
    ) -> Option<&mut Question> {
        let idx = idx.into();
        let theme = self.get_theme_mut(idx.parent())?;
        if *idx > theme.questions.len() {
            return None;
        }
        theme.questions.insert(*idx, question);
        Some(&mut theme.questions[*idx])
    }

    /// Clone a [`Question`], push it afterwards and return
    /// a reference to the new question.
    pub fn duplicate_question(&mut self, idx: impl Into<QuestionIdx>) -> Option<&mut Question> {
        let idx = idx.into();
        self.get_question(idx)
            .cloned()
            .and_then(|question| self.insert_question(idx.next(), question))
    }

    /// Create a new default [`Question`], push it to the [`Theme`] in [`Round`]
    /// and return a reference to it.
    pub fn allocate_question(&mut self, idx: impl Into<ThemeIdx>) -> Option<&mut Question> {
        let idx = idx.into();
        let price = self.get_theme(idx)?.guess_next_question_price();
        let question = Question { price, ..Default::default() };
        self.push_question(idx, question)
    }

    /// Return amount of [`Question`]s in a [`Theme`].
    pub fn count_questions(&self, idx: impl Into<ThemeIdx>) -> usize {
        self.get_theme(idx).map(|theme| theme.questions.len()).unwrap_or_default()
    }
}

/// # IO and resource methods
impl Package {
    const CONTENT_TYPE_FILE_CONTENT: &'static str = r#"<?xml version="1.0" encoding="utf-8"?><Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types"><Default Extension="xml" ContentType="si/xml" /></Types>"""#;
    const XML_VERSION_ENCODING: &'static str = r#"<?xml version="1.0" encoding="utf-8"?>"#;
    const CONTROLS_ASCII_SET: &'static AsciiSet = &CONTROLS.add(b' ');

    pub fn get_resource(&self, atom: &Atom) -> Option<&Vec<u8>> {
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
    pub fn from_zip_buffer(bytes: impl AsRef<[u8]>) -> Result<Package, Error> {
        let cursor = io::Cursor::new(bytes);
        Self::get_package_from_zip(cursor)
    }

    pub fn open_zip_file(path: impl AsRef<Path>) -> Result<Package, Error> {
        let package_file = File::open(path)?;
        Self::get_package_from_zip(package_file)
    }

    fn get_package_from_zip<T: Read + io::Seek>(source: T) -> Result<Package, Error> {
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
        package.map(|p| Package { resource: resources, ..p })
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
