use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};
use quick_xml::de::from_str;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Display;
use std::io::{Error, ErrorKind, Write};
use std::path::Path;
use std::{fs::File, io, io::Read};
use zip::write::FileOptions;
use zip::{CompressionMethod, ZipArchive, ZipWriter};

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct Package {
    pub id: String,
    pub name: String,
    pub version: f32,
    pub date: String,
    pub difficulty: u8,
    pub language: String,
    pub logo: Option<String>,
    pub publisher: String,
    pub restriciton: String,
    pub rounds: Rounds,
    pub tags: Vec<String>,
    pub info: Info,

    // resources
    #[serde(skip)]
    pub resource: HashMap<Resource, Vec<u8>>,
}

impl Package {
    /// Get [`Round`] by index.
    pub fn get_round(&self, index: usize) -> Option<&Round> {
        self.rounds.rounds.get(index)
    }

    /// Get mutable [`Round`] by index.
    pub fn get_round_mut(&mut self, index: usize) -> Option<&mut Round> {
        self.rounds.rounds.get_mut(index)
    }

    /// Remove [`Round`] by index and return it.
    pub fn remove_round(&mut self, index: usize) -> Option<Round> {
        if index >= self.rounds.rounds.len() {
            return None;
        }
        self.rounds.rounds.remove(index).into()
    }

    /// Push a new [`Round`] to the end of the package and
    /// return a reference to it.
    pub fn push_round(&mut self, round: Round) -> &mut Round {
        self.rounds.rounds.push(round);
        self.rounds.rounds.last_mut().unwrap()
    }

    /// Create a new default [`Round`], push it and return
    /// a reference to it.
    pub fn allocate_round(&mut self) -> &mut Round {
        let round = Round { name: "Новый раунд".to_string(), ..Default::default() };
        self.push_round(round)
    }

    /// Get [`Theme`] in [`Round`] by indices.
    pub fn get_theme(&self, round_index: usize, index: usize) -> Option<&Theme> {
        self.get_round(round_index).and_then(|round| round.themes.themes.get(index))
    }

    /// Get mutable [`Theme`] in [`Round`] by indices.
    pub fn get_theme_mut(&mut self, round_index: usize, index: usize) -> Option<&mut Theme> {
        self.get_round_mut(round_index).and_then(|round| round.themes.themes.get_mut(index))
    }

    /// Remove [`Theme`] in [`Round`] by indices.
    pub fn remove_theme(&mut self, round_index: usize, index: usize) -> Option<Theme> {
        let round = self.get_round_mut(round_index)?;
        if index >= round.themes.themes.len() {
            return None;
        }
        round.themes.themes.remove(index).into()
    }

    /// Push a new [`Theme`] to the end of the [`Round`] and
    /// return a reference to it.
    pub fn push_theme(&mut self, round_index: usize, theme: Theme) -> Option<&mut Theme> {
        let round = self.get_round_mut(round_index)?;
        round.themes.themes.push(theme);
        round.themes.themes.last_mut().unwrap().into()
    }

    /// Create a new default [`Theme`], push it to the [`Round`]
    /// and return a reference to it.
    pub fn allocate_theme(&mut self, round_index: usize) -> Option<&mut Theme> {
        let theme = Theme { name: "Новая тема".to_string(), ..Default::default() };
        self.push_theme(round_index, theme)
    }

    /// Get [`Question`] in [`Theme`] in [`Round`] by indices.
    pub fn get_question(
        &self,
        round_index: usize,
        theme_index: usize,
        index: usize,
    ) -> Option<&Question> {
        self.get_theme(round_index, theme_index)
            .and_then(|theme| theme.questions.questions.get(index))
    }

    /// Get mutable [`Question`] in [`Theme`] in [`Round`] by indices.
    pub fn get_question_mut(
        &mut self,
        round_index: usize,
        theme_index: usize,
        index: usize,
    ) -> Option<&mut Question> {
        self.get_theme_mut(round_index, theme_index)
            .and_then(|theme| theme.questions.questions.get_mut(index))
    }

    /// Remove [`Question`] in [`Theme`] in [`Round`] by indices.
    pub fn remove_question(
        &mut self,
        round_index: usize,
        theme_index: usize,
        index: usize,
    ) -> Option<Question> {
        let theme = self.get_theme_mut(round_index, theme_index)?;
        if index >= theme.questions.questions.len() {
            return None;
        }
        theme.questions.questions.remove(index).into()
    }

    /// Push a new [`Question`] to the end of the [`Theme`] in [`Round`]
    /// and return a reference to it.
    pub fn push_question(
        &mut self,
        round_index: usize,
        theme_index: usize,
        question: Question,
    ) -> Option<&mut Question> {
        let theme = self.get_theme_mut(round_index, theme_index)?;
        theme.questions.questions.push(question);
        theme.questions.questions.last_mut().unwrap().into()
    }

    /// Create a new default [`Question`], push it to the [`Theme`] in [`Round`]
    /// and return a reference to it.
    pub fn allocate_question(
        &mut self,
        round_index: usize,
        theme_index: usize,
    ) -> Option<&mut Question> {
        let question = Question {
            price: self.guess_next_question_price(round_index, theme_index),
            ..Default::default()
        };
        self.push_question(round_index, theme_index, question)
    }
}

impl Package {
    /// Try to guess a price for the next question:
    /// - Either a difference between the last two question prices;
    /// - Or the last question's price plus 100;
    ///
    /// In case of no questions, the default price is 100.
    pub fn guess_next_question_price(&self, round_index: usize, theme_index: usize) -> usize {
        let Some(theme) = self.get_theme(round_index, theme_index) else {
            return 100;
        };
        let questions = &theme.questions.questions;
        let mut iter = questions.iter().rev();
        match (iter.next(), iter.next()) {
            (Some(last), Some(prev)) => {
                let diff = last.price.abs_diff(prev.price);
                last.price + diff
            },
            (Some(last), None) => last.price + 100,
            _ => 100,
        }
    }
}

/// Package tree node which operates on indices and is easy to copy.
#[derive(
    serde::Deserialize, serde::Serialize, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
pub enum PackageNode {
    Round { index: usize },
    Theme { round_index: usize, index: usize },
    Question { round_index: usize, theme_index: usize, index: usize },
}

impl PackageNode {
    /// Get parent of the node, unless it's a round node..
    pub fn get_parent(&self) -> Option<PackageNode> {
        match *self {
            PackageNode::Round { .. } => None,
            PackageNode::Theme { round_index, .. } => {
                PackageNode::Round { index: round_index }.into()
            },
            PackageNode::Question { round_index, theme_index, .. } => {
                PackageNode::Theme { round_index, index: theme_index }.into()
            },
        }
    }

    /// Get index of the node itself.
    pub fn index(&self) -> usize {
        match *self {
            PackageNode::Round { index }
            | PackageNode::Theme { index, .. }
            | PackageNode::Question { index, .. } => index,
        }
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct Info {
    pub comments: Option<String>,
    pub extension: Option<String>,
    pub authors: Authors,
    pub sources: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct Authors {
    #[serde(rename = "author", default)]
    pub authors: Vec<String>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct Rounds {
    #[serde(rename = "round", default)]
    pub rounds: Vec<Round>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct Round {
    pub name: String,
    #[serde(rename = "type", default)]
    pub variant: Option<String>,
    pub info: Option<Info>,
    pub themes: Themes,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct Themes {
    #[serde(rename = "theme", default)]
    pub themes: Vec<Theme>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct Theme {
    pub name: String,
    pub questions: Questions,
    pub info: Option<Info>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct Questions {
    #[serde(rename = "question", default)]
    pub questions: Vec<Question>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct Question {
    pub price: usize,
    pub scenario: Scenario,
    pub right: Right,
    pub wrong: Option<Wrong>,
    #[serde(rename = "type", default)]
    pub variant: Option<Variant>,
    pub info: Option<Info>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct Variant {
    pub name: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct Scenario {
    #[serde(rename = "atom", default)]
    pub atoms: Vec<Atom>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct Right {
    #[serde(rename = "answer", default)]
    pub answers: Vec<Answer>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct Wrong {
    #[serde(rename = "answer", default)]
    pub answers: Vec<Answer>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct Answer {
    #[serde(rename = "$value")]
    pub body: Option<String>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct Atom {
    pub time: Option<f64>,
    #[serde(rename = "type", default)]
    pub variant: Option<String>,
    #[serde(rename = "$value")]
    pub body: Option<String>,
}

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
            Resource::Audio(key) |
            Resource::Video(key) |
            Resource::Image(key) |
            Resource::Texts(key) => key
        }
    }
}

impl Package {
    const CONTENT_TYPE_FILE_CONTENT: &'static str = r#"<?xml version="1.0" encoding="utf-8"?><Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types"><Default Extension="xml" ContentType="si/xml" /></Types>"""#;
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

    fn to_bytes(self) -> Result<Vec<u8>, Error> {
        let buffer = Vec::new();
        let cursor = io::Cursor::new(buffer);
        let mut zip = ZipWriter::new(cursor);

        // Define file options (e.g., compression method)
        let options = FileOptions::default().compression_method(CompressionMethod::Deflated);
        let xml = quick_xml::se::to_string(&self)
            .map_err(|e| io::Error::new(ErrorKind::InvalidData, e))?;
        zip.start_file("content.xml", options)?;
        zip.write_all(&xml.into_bytes())?;

        zip.start_file("[Content_Types].xml", options)?;
        zip.write_all(Self::CONTENT_TYPE_FILE_CONTENT.as_ref())?;

        for (key, value) in self.resource.into_iter() {
            zip.start_file(key.extract_key(), options)?;
            zip.write_all(&value)?
        }

        let result = zip.finish()?;

        Ok(result.into_inner())
    }
}
