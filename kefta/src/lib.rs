mod parse;

pub use kefta_core::{
    parse::{
        AttrModel,
        AttrValue,
        AttrMap,
    },
    node::{
        AttrNode,
        AttrContents,
        ContainerType,
        AttrSource
    }
};

pub mod error {
    pub use kefta_core::error::*;
    pub use kefta_core::node::{ParseError, ParseErrorKind};
}

#[cfg(feature="derive")]
pub use kefta_macro::AttrModel;

#[cfg(feature="syn")]
pub use kefta_core::parse::Syn;