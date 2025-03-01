#![allow(dead_code)]

pub mod node;
pub mod package_trait;
mod serde_impl;
pub mod v4;
pub mod v5;

pub mod prelude {
    pub use crate::node::*;
    pub use crate::package_trait::*;
    pub use crate::v4::{
        Infov4 as Info, Packagev4 as Package, Questionv4 as Question, Roundv4 as Round,
        Themev4 as Theme,
    };
}
