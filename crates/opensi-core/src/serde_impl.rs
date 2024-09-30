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

// WHY: crate::$type gives errors in rust-analyzer, but compiles ?
generate_serde_mod!(rounds: super::super::Round as round);
generate_serde_mod!(themes: super::super::Theme as theme);
generate_serde_mod!(questions: super::super::Question as question);
generate_serde_mod!(atoms: super::super::Atom as atom);
generate_serde_mod!(answers: super::super::Answer as answer);
generate_serde_mod!(authors: String as author);
generate_serde_mod!(sources: String as source);
