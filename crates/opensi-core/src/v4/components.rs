use serde::{Deserialize, Serialize};

use crate::{
    node::{RoundIdx, ThemeIdx},
    package_trait::{QuestionBase, QuestionsContainer, RoundBase, ThemeBase, ThemesContainer},
    serde_impl,
};

use super::Atomv4;

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct Infov4 {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub comments: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub extension: String,
    #[serde(with = "serde_impl::authors", skip_serializing_if = "Vec::is_empty")]
    pub authors: Vec<String>,
    #[serde(with = "serde_impl::sources", skip_serializing_if = "Vec::is_empty")]
    pub sources: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct Roundv4 {
    #[serde(rename = "@name")]
    pub name: String,
    // TODO: Actual enum of kinds
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(rename = "@info", skip_serializing_if = "Option::is_none")]
    pub info: Option<Infov4>,
    #[serde(with = "serde_impl::themes")]
    pub themes: Vec<Themev4>,
}

impl Default for Roundv4 {
    fn default() -> Self {
        Self { name: "Новый раунд".to_string(), kind: None, info: None, themes: vec![] }
    }
}

impl RoundBase for Roundv4 {}
impl ThemesContainer for Roundv4 {
    type Theme = Themev4;

    fn get_themes(&self, _idx: impl Into<RoundIdx>) -> Option<&Vec<Self::Theme>> {
        Some(&self.themes)
    }

    fn get_themes_mut(&mut self, _idx: impl Into<RoundIdx>) -> Option<&mut Vec<Self::Theme>> {
        Some(&mut self.themes)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Themev4 {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(with = "serde_impl::questions")]
    pub questions: Vec<Questionv4>,
    #[serde(rename = "@info", skip_serializing_if = "Option::is_none")]
    pub info: Option<Infov4>,
}

impl ThemeBase for Themev4 {}
impl QuestionsContainer for Themev4 {
    type Question = Questionv4;

    fn get_questions(&self, _idx: impl Into<ThemeIdx>) -> Option<&Vec<Self::Question>> {
        Some(&self.questions)
    }

    fn get_questions_mut(&mut self, _idx: impl Into<ThemeIdx>) -> Option<&mut Vec<Self::Question>> {
        Some(&mut self.questions)
    }
}

impl Default for Themev4 {
    fn default() -> Self {
        Self {
            name: "Новая тема".to_string(),
            questions: vec![
                Questionv4 { price: 100, ..Questionv4::default() },
                Questionv4 { price: 200, ..Questionv4::default() },
                Questionv4 { price: 300, ..Questionv4::default() },
                Questionv4 { price: 400, ..Questionv4::default() },
                Questionv4 { price: 500, ..Questionv4::default() },
            ],
            info: None,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct Questionv4 {
    #[serde(rename = "@price")]
    pub price: usize,
    #[serde(rename = "type")]
    pub question_type: QuestionTypev4,
    #[serde(with = "serde_impl::atoms")]
    pub scenario: Vec<Atomv4>,
    #[serde(with = "serde_impl::answers")]
    pub right: Vec<String>,
    #[serde(with = "serde_impl::answers", skip_serializing_if = "Vec::is_empty")]
    pub wrong: Vec<String>,
    #[serde(rename = "@info", skip_serializing_if = "Option::is_none")]
    pub info: Option<Infov4>,
}

impl Default for Questionv4 {
    fn default() -> Self {
        Self {
            price: 100,
            question_type: QuestionTypev4::default(),
            scenario: vec![],
            right: vec![],
            wrong: vec![],
            info: None,
        }
    }
}

impl QuestionBase for Questionv4 {
    fn get_price(&self) -> usize {
        self.price
    }

    fn set_price(&mut self, price: usize) {
        self.price = price;
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct QuestionTypev4 {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "param", skip_serializing_if = "Option::is_none")]
    pub params: Option<Vec<Paramv4>>,
}

impl std::fmt::Display for QuestionTypev4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display = match self.name.as_str() {
            "simple" | "" => "Обычный вопрос",
            "auction" => "Вопрос со ставкой",
            "cat" => "Вопрос с секретом",
            "bagcat" => "Обобщённый Вопрос с секретом",
            "sponsored" => "Вопрос без риска",
            _ => "Неизвестно",
        };
        f.write_str(display)
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Paramv4 {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "$value", skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
}
