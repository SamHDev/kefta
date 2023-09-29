use std::fmt;
use proc_macro2::{Span, TokenStream};
use crate::error::{Error, ErrorDisplay};

pub trait FromMeta {
    fn from_meta<S>(source: &mut S) -> Result<Self, S::Error> where S: MetaSource;
}

#[allow(unused_variables)]
pub trait MetaVisitor: Sized {
    type Output;

    fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result;

    fn visit_path<S>(self, path: &str, source: &mut S) -> Result<Self::Output, S::Error> where S: MetaSource {
        Err(Error::expected(&self, "path"))
    }

    fn visit_list<S>(self, contents: S) -> Result<Self::Output, S::Error> where S: MetaSource {
        Err(Error::expected(&self, "list"))
    }

    fn visit_marker<E>(self) -> Result<Self::Output, E> where E: Error {
        Err(Error::expected(&self, "marker"))
    }

    fn visit_value<E>(self, stream: TokenStream) -> Result<Self::Output, E> where E: Error {
        Err(Error::expected(&self, "stream"))
    }
}

pub trait MetaSource {
    type Error: Error;

    fn remaining(&self) -> bool;

    fn position(&self) -> Span;

    fn visit<V>(&mut self, visit: V) -> Result<V::Output, Self::Error> where V: MetaVisitor;
}


impl<'a, T> ErrorDisplay for T where T: MetaVisitor {
    fn description(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.expecting(fmt)
    }
}