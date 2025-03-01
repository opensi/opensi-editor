use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Atomv4 {
    #[serde(rename = "@time", skip_serializing_if = "Option::is_none", default)]
    pub time: Option<f64>,
    #[serde(rename = "@type", skip_serializing_if = "AtomKindv4::is_text", default)]
    pub kind: AtomKindv4,
    #[serde(rename = "$value", skip_serializing_if = "String::is_empty", default)]
    pub body: String,
}

impl Atomv4 {
    const CONTROLS_ASCII_SET: &'static percent_encoding::AsciiSet =
        &percent_encoding::CONTROLS.add(b' ');

    pub fn resource(&self) -> Option<ResourceIdv4> {
        if self.kind.is_text() {
            return None;
        }

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

        let resource_name =
            percent_encoding::utf8_percent_encode(&self.body, Self::CONTROLS_ASCII_SET).to_string();

        let resource = match self.kind {
            AtomKindv4::Image => ResourceIdv4::Audio(format!("Images/{}", resource_name)),
            AtomKindv4::Video => ResourceIdv4::Image(format!("Video/{}", resource_name)),
            AtomKindv4::Voice => ResourceIdv4::Video(format!("Audio/{}", resource_name)),
            _ => return None,
        };

        Some(resource)
    }
}

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum AtomKindv4 {
    Image,
    Voice,
    Video,
    #[serde(other)]
    #[default]
    Text,
}

impl AtomKindv4 {
    pub fn is_text(&self) -> bool {
        matches!(self, Self::Text)
    }
}

/// Typed resource handle for [`Atomv4`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ResourceIdv4 {
    Audio(String),
    Video(String),
    Image(String),
    Texts(String),
}

impl ResourceIdv4 {
    pub fn try_new(path: impl AsRef<str>) -> Option<Self> {
        let path = path.as_ref();
        let (category, path) = path.split_once('/')?;

        let path = if path.starts_with('@') { path.to_string() } else { format!("@{path}") };
        let id = match category {
            "Audio" => Self::Audio(format!("{}/{}", category, path)),
            "Images" => Self::Image(format!("{}/{}", category, path)),
            "Video" => Self::Video(format!("{}/{}", category, path)),
            "Texts" => Self::Texts(format!("{}/{}", category, path)),
            _ => return None,
        };

        Some(id)
    }

    pub fn path(&self) -> &str {
        match self {
            Self::Audio(path) | Self::Video(path) | Self::Image(path) | Self::Texts(path) => path,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn regular_atom() {
        let text_atom =
            Atomv4 { time: None, kind: AtomKindv4::default(), body: "text atom body".to_string() };

        assert_eq!(
            quick_xml::se::to_string_with_root("atom", &text_atom).unwrap(),
            "<atom>text atom body</atom>"
        );
        assert_eq!(
            quick_xml::de::from_str::<Atomv4>("<atom>text atom body</atom>").unwrap(),
            text_atom
        );
        assert_eq!(
            quick_xml::de::from_str::<Atomv4>("<atom type=\"text\">text atom body</atom>").unwrap(),
            text_atom
        );
    }

    #[test]
    fn resource_atom() {
        let image_atom = Atomv4 { time: None, kind: AtomKindv4::Image, body: "@1.jpg".to_string() };

        assert_eq!(
            quick_xml::se::to_string_with_root("atom", &image_atom).unwrap(),
            "<atom type=\"image\">@1.jpg</atom>"
        );
        assert_eq!(
            quick_xml::de::from_str::<Atomv4>("<atom type=\"image\">@1.jpg</atom>").unwrap(),
            image_atom
        );
    }
}
