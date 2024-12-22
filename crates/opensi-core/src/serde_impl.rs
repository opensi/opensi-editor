/// Implement a module with name `$module_name`, which
/// would contain `serialize` and `deserialize` functions to
/// use with `#[serde(with = "mod")]` on fields with nested containers.
macro_rules! generate_serde_mod {
    ($module_name:ident: $type:ty as $name:ident) => {
        pub mod $module_name {
            use serde::{Deserialize, Deserializer, Serialize, Serializer};

            pub fn deserialize<'de, D: Deserializer<'de>>(
                deserializer: D,
            ) -> Result<Vec<$type>, D::Error> {
                #[derive(serde::Deserialize)]
                struct List {
                    #[serde(rename = "$value")]
                    element: Vec<$type>,
                }
                Ok(List::deserialize(deserializer)?.element)
            }

            pub fn serialize<S: Serializer>(
                elements: &Vec<$type>,
                serializer: S,
            ) -> Result<S::Ok, S::Error> {
                #[derive(Serialize)]
                struct List<'a> {
                    $name: &'a Vec<$type>,
                }

                let list = List { $name: elements };
                list.serialize(serializer)
            }
        }
    };
}

generate_serde_mod!(rounds: crate::v5::RoundV5 as round);
generate_serde_mod!(themes: crate::v5::ThemeV5 as theme);
generate_serde_mod!(questions: crate::v5::QuestionV5 as question);
generate_serde_mod!(atoms: crate::v5::AtomV5 as atom);
generate_serde_mod!(answers: crate::v5::AnswerV5 as answer);
generate_serde_mod!(authors: String as author);
generate_serde_mod!(sources: String as source);
generate_serde_mod!(tags: crate::v5::Tag as tag);
