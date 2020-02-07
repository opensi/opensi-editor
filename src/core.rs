#![cfg(feature = "serialize")]
extern crate quick_xml;
extern crate serde; 

use PartialEq;
use serde::Deserialize;
// use quick_xml::DeError;
use quick_xml::de::{from_str, DeError};
// use quick_xml::se::to_string;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Package {
    id: String,
    name: String,
    version: String,
    date: Option<String>,
    difficulty: Option<u8>,
    language: Option<String>,
    logo: Option<String>,
    publisher: Option<String>,
    restriciton: Option<String>,
    rounds: Vec<Round>,
    tags: Vec<String>,
    info: Info
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Info {
    comments: String, 
    extension: String,
    authors: Vec<String>,
    sources: Vec<String>
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Round {
    name: String,
    variant: String, // fixme: original name "type"
    info: Info, 
    themes: Vec<Theme>
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Theme {
    name: String,
    questions: Vec<Question>,
    info: Info
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Question {
    price: usize, 
    scenario: Vec<Atom>, 
    right: Vec<String>, 
    wrong: Vec<String>, 
    variant: String, // fixme: original name "type"
    info: Info
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Atom {
    time: Option<f64>,
    variant: String // fixme: original name "type"
}
