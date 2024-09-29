use serde::{Deserialize, Deserializer, Serialize, Serializer};
use crate::{Answer, Atom, Question, Round, Theme};

// Generic function to deserialize List structures
pub fn unwrap_list<'de, T, D>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de> + Default,
{
    #[derive(Deserialize)]
    struct List<T> {
        #[serde(rename = "$value")]
        element: Vec<T>,
    }
    Ok(List::deserialize(deserializer)?.element)
}

pub fn unwrap_option_list<'de, T, D>(deserializer: D) -> Result<Option<Vec<T>>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de> + Default,
{
    #[derive(Deserialize)]
    struct List<T> {
        #[serde(rename = "$value")]
        element: Option<Vec<T>>,
    }

    Ok(List::deserialize(deserializer)?.element)
}

pub fn wrap_round_list<S>(rounds: &Vec<Round>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    #[derive(Serialize)]
    struct List<'a> {
        round: &'a Vec<Round>,
    }

    let list = List { round: rounds };
    list.serialize(serializer)
}

pub fn wrap_theme_list<S>(themes: &Vec<Theme>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    #[derive(Serialize)]
    struct List<'a> {
        theme: &'a Vec<Theme>,
    }

    let list = List { theme: themes };
    list.serialize(serializer)
}

pub fn wrap_question_list<S>(questions: &Vec<Question>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    #[derive(Serialize)]
    struct List<'a> {
        question: &'a Vec<Question>,
    }

    let list = List { question: questions };
    list.serialize(serializer)
}

pub fn wrap_atom_list<S>(atoms: &Vec<Atom>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    #[derive(Serialize)]
    struct List<'a> {
        atom: &'a Vec<Atom>,
    }

    let list = List { atom: atoms };
    list.serialize(serializer)
}

pub fn wrap_answer_list<S>(answers: &Vec<Answer>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    #[derive(Serialize)]
    struct List<'a> {
        answer: &'a Vec<Answer>,
    }

    let list = List { answer: answers };
    list.serialize(serializer)
}

pub fn wrap_option_answer_list<S>(answers: &Option<Vec<Answer>>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    #[derive(Serialize)]
    struct List<'a> {
        answer: &'a Option<Vec<Answer>>,
    }

    let list = List { answer: answers };
    list.serialize(serializer)
}