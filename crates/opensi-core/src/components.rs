use serde::{Deserialize, Serialize};

use crate::serde_impl;

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct Info {
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
pub struct Authors {
    #[serde(rename = "author")]
    pub authors: Vec<String>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct Round {
    #[serde(rename = "@name")]
    pub name: String,
    // TODO: Actual enum of kinds
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(rename = "@info", skip_serializing_if = "Option::is_none")]
    pub info: Option<Info>,
    #[serde(with = "serde_impl::themes")]
    pub themes: Vec<Theme>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Theme {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(with = "serde_impl::questions")]
    pub questions: Vec<Question>,
    #[serde(rename = "@info", skip_serializing_if = "Option::is_none")]
    pub info: Option<Info>,
}

impl Theme {
    /// Try to guess price for the next question:
    /// - Either a difference between the last two question prices;
    /// - Or the last question's price plus 100;
    ///
    /// In case of no questions, the default price is 100.
    pub fn guess_next_question_price(&self) -> usize {
        let mut iter = self.questions.iter().rev();
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

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Questions {
    #[serde(rename = "question")]
    pub questions: Vec<Question>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct Question {
    #[serde(rename = "@price")]
    pub price: usize,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub question_type: Option<QuestionType>,
    #[serde(with = "serde_impl::atoms")]
    pub scenario: Vec<Atom>,
    #[serde(with = "serde_impl::answers")]
    pub right: Vec<Answer>,
    #[serde(with = "serde_impl::answers", skip_serializing_if = "Vec::is_empty")]
    pub wrong: Vec<Answer>,
    #[serde(rename = "@info", skip_serializing_if = "Option::is_none")]
    pub info: Option<Info>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct QuestionType {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "param", skip_serializing_if = "Option::is_none")]
    pub params: Option<Vec<Param>>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Answer {
    #[serde(rename = "$value", skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Param {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "$value")]
    pub body: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Atom {
    #[serde(rename = "@time", skip_serializing_if = "Option::is_none")]
    pub time: Option<f64>,
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub variant: Option<String>,
    #[serde(rename = "$value", skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
}
