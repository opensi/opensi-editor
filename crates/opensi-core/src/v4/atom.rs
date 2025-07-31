use std::sync::Arc;

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
            AtomKindv4::Image => ResourceIdv4::image(resource_name),
            AtomKindv4::Video => ResourceIdv4::video(resource_name),
            AtomKindv4::Voice => ResourceIdv4::audio(resource_name),
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
    Audio(Arc<(String, String)>),
    Video(Arc<(String, String)>),
    Image(Arc<(String, String)>),
    Texts(Arc<(String, String)>),
}

impl ResourceIdv4 {
    pub fn audio(path: impl AsRef<str>) -> Self {
        Self::try_new(format!("Audio/{}", path.as_ref())).unwrap()
    }

    pub fn video(path: impl AsRef<str>) -> Self {
        Self::try_new(format!("Video/{}", path.as_ref())).unwrap()
    }

    pub fn image(path: impl AsRef<str>) -> Self {
        Self::try_new(format!("Images/{}", path.as_ref())).unwrap()
    }

    pub fn texts(path: impl AsRef<str>) -> Self {
        Self::try_new(format!("Texts/{}", path.as_ref())).unwrap()
    }

    pub fn try_new(path: impl AsRef<str>) -> Option<Self> {
        let path = path.as_ref();
        let (category, name) = path.split_once('/')?;

        let name = if name.starts_with('@') { name.to_string() } else { format!("@{name}") };
        let id = match category {
            "Audio" => Self::Audio(Arc::new((format!("{}/{}", category, name), name))),
            "Images" => Self::Image(Arc::new((format!("{}/{}", category, name), name))),
            "Video" => Self::Video(Arc::new((format!("{}/{}", category, name), name))),
            "Texts" => Self::Texts(Arc::new((format!("{}/{}", category, name), name))),
            _ => return None,
        };

        Some(id)
    }

    /// Get full resource path, e.g. "Images/@joker.png".
    pub fn path(&self) -> &str {
        match self {
            Self::Audio(data) | Self::Video(data) | Self::Image(data) | Self::Texts(data) => {
                data.0.as_str()
            },
        }
    }

    /// Get only the name part of the resource, e.g. "@joker.png".
    pub fn name(&self) -> &str {
        match self {
            Self::Audio(data) | Self::Video(data) | Self::Image(data) | Self::Texts(data) => {
                data.1.as_str()
            },
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
