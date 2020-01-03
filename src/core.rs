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

pub struct Info {
    comments: String, 
    extension: String,
    authors: Vec<String>,
    sources: Vec<String>
}

pub struct Round {
    name: String,
    variant: String, // original name "type"
    info: Info, 
    themes: Vec<Theme>
}

pub struct Theme {
    name: String,
    questions: Vec<Question>
    info: Info
}

pub struct Question {
    price: usize, 
    scenario: Vec<Atom>, 
    right: Vec<String>, 
    wrong: Vec<String>, 
    variant: String // original name "type"
    info: Info
}

pub enum Atom {
    time: Option<f64>,
    variant: String // original name "type"
}