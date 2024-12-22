use serde::{Deserialize, Serialize};

use crate::{
    node::{RoundIdx, ThemeIdx},
    package_trait::{QuestionBase, QuestionsContainer, RoundBase, ThemeBase, ThemesContainer},
    serde_impl,
};

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct InfoV5 {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub comments: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub extension: String,
    #[serde(with = "serde_impl::authors", skip_serializing_if = "Vec::is_empty")]
    pub authors: Vec<String>,
    #[serde(with = "serde_impl::sources", skip_serializing_if = "Vec::is_empty")]
    pub sources: Vec<String>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct AuthorsV5 {
    #[serde(rename = "author")]
    pub authors: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct RoundV5 {
    #[serde(rename = "@name")]
    pub name: String,
    // TODO: Actual enum of kinds
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(rename = "@info", skip_serializing_if = "Option::is_none")]
    pub info: Option<InfoV5>,
    #[serde(with = "serde_impl::themes")]
    pub themes: Vec<ThemeV5>,
}

impl Default for RoundV5 {
    fn default() -> Self {
        Self { name: "Новый раунд".to_string(), kind: None, info: None, themes: vec![] }
    }
}

impl RoundBase for RoundV5 {}
impl ThemesContainer for RoundV5 {
    type Theme = ThemeV5;

    fn get_themes(&self, _idx: impl Into<RoundIdx>) -> Option<&Vec<Self::Theme>> {
        Some(&self.themes)
    }

    fn get_themes_mut(&mut self, _idx: impl Into<RoundIdx>) -> Option<&mut Vec<Self::Theme>> {
        Some(&mut self.themes)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ThemeV5 {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(with = "serde_impl::questions")]
    pub questions: Vec<QuestionV5>,
    #[serde(rename = "@info", skip_serializing_if = "Option::is_none")]
    pub info: Option<InfoV5>,
}

impl ThemeBase for ThemeV5 {}
impl QuestionsContainer for ThemeV5 {
    type Question = QuestionV5;

    fn get_questions(&self, _idx: impl Into<ThemeIdx>) -> Option<&Vec<Self::Question>> {
        Some(&self.questions)
    }

    fn get_questions_mut(&mut self, _idx: impl Into<ThemeIdx>) -> Option<&mut Vec<Self::Question>> {
        Some(&mut self.questions)
    }
}

impl Default for ThemeV5 {
    fn default() -> Self {
        Self {
            name: "Новая тема".to_string(),
            questions: vec![
                QuestionV5 { price: 100, ..QuestionV5::default() },
                QuestionV5 { price: 200, ..QuestionV5::default() },
                QuestionV5 { price: 300, ..QuestionV5::default() },
                QuestionV5 { price: 400, ..QuestionV5::default() },
                QuestionV5 { price: 500, ..QuestionV5::default() },
            ],
            info: None,
        }
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct QuestionsV5 {
    #[serde(rename = "question")]
    pub questions: Vec<QuestionV5>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct QuestionV5 {
    #[serde(rename = "@price")]
    pub price: usize,
    #[serde(rename = "type")]
    pub question_type: QuestionTypeV5,
    #[serde(with = "serde_impl::atoms")]
    pub scenario: Vec<AtomV5>,
    #[serde(with = "serde_impl::answers")]
    pub right: Vec<AnswerV5>,
    #[serde(with = "serde_impl::answers", skip_serializing_if = "Vec::is_empty")]
    pub wrong: Vec<AnswerV5>,
    #[serde(rename = "@info", skip_serializing_if = "Option::is_none")]
    pub info: Option<InfoV5>,
}

impl Default for QuestionV5 {
    fn default() -> Self {
        Self {
            price: 100,
            question_type: QuestionTypeV5::default(),
            scenario: vec![],
            right: vec![],
            wrong: vec![],
            info: None,
        }
    }
}

impl QuestionBase for QuestionV5 {
    fn get_price(&self) -> usize {
        self.price
    }

    fn set_price(&mut self, price: usize) {
        self.price = price;
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct QuestionTypeV5 {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "param", skip_serializing_if = "Option::is_none")]
    pub params: Option<Vec<ParamV5>>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct AnswerV5 {
    #[serde(rename = "$value", skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct ParamV5 {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "$value", skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct AtomV5 {
    #[serde(rename = "@time", skip_serializing_if = "Option::is_none")]
    pub time: Option<f64>,
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub variant: Option<String>,
    #[serde(rename = "$value", skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
}
