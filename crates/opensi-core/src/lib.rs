#![allow(dead_code)]

use quick_xml::de::from_str;
use serde::{Deserialize, Serialize};
use std::io::ErrorKind;
use std::path::Path;
use std::{fs::File, io, io::Read};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PackageNode {
    Round { index: usize },
    Theme { round_index: usize, index: usize },
    Question { round_index: usize, theme_index: usize, index: usize },
}

impl PackageNode {
    /// Get index of the node itself.
    pub fn index(&self) -> usize {
        match self {
            &PackageNode::Round { index }
            | &PackageNode::Theme { index, .. }
            | &PackageNode::Question { index, .. } => index,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Info {
    pub comments: Option<String>,
    pub extension: Option<String>,
    pub authors: Authors,
    pub sources: Option<Vec<String>>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Authors {
    #[serde(rename = "author", default)]
    pub authors: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Rounds {
    #[serde(rename = "round", default)]
    pub rounds: Vec<Round>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Round {
    pub name: String,
    #[serde(rename = "type", default)]
    pub variant: Option<String>,
    pub info: Option<Info>,
    pub themes: Themes,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Themes {
    #[serde(rename = "theme", default)]
    pub themes: Vec<Theme>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Theme {
    pub name: String,
    pub questions: Questions,
    pub info: Option<Info>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Questions {
    #[serde(rename = "question", default)]
    pub questions: Vec<Question>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
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
pub struct Variant {
    pub name: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Scenario {
    #[serde(rename = "atom", default)]
    pub atoms: Vec<Atom>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Right {
    #[serde(rename = "answer", default)]
    pub answers: Vec<Answer>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Wrong {
    #[serde(rename = "answer", default)]
    pub answers: Vec<Answer>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Answer {
    #[serde(rename = "$value")]
    pub body: Option<String>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
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
