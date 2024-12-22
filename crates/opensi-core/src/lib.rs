#![allow(dead_code)]

pub mod node;
pub mod package_trait;
mod serde_impl;
pub mod v4;
pub mod v5;

pub mod prelude {
    pub use crate::node::*;
    pub use crate::package_trait::*;
    pub use crate::v5::{
        InfoV5 as Info, PackageV5 as Package, QuestionV5 as Question, RoundV5 as Round,
        ThemeV5 as Theme,
    };
}
