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
        Infov5 as Info, Packagev5 as Package, Questionv5 as Question, Roundv5 as Round,
        Themev5 as Theme,
    };
}
