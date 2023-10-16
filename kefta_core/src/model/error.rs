use std::fmt;
use proc_macro2::Span;
use crate::model::source::MetaDomain;
use crate::model::visitor::MetaVisitor;

pub trait MetaError: Sized {
    fn custom(span: Option<Span>, message: impl fmt::Display) -> Self;

    fn expecting(span: Option<Span>, expected: impl MetaExpected, found: impl fmt::Display) -> Self {
        Self::custom(span, format_args!("expected: {expected}, found {found}", ))
    }
}

pub trait MetaExpected {
    fn expected(&self, f: &mut fmt::Formatter) -> fmt::Result;
}

impl<T, V> MetaExpected for V where T: MetaDomain, V: MetaVisitor<T> {
    fn expected(&self, &mut f: fmt::Formatter) -> fmt::Result {
        self.expecting(f)
    }
}