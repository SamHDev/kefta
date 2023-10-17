use std::fmt;
use proc_macro2::{Span, TokenStream};
use crate::model::visitor::MetaVisitor;

pub trait MetaError: Sized {
    fn into_token_stream(self) -> TokenStream;

    fn custom(span: Option<Span>, message: impl fmt::Display) -> Self;

    fn expecting(span: Option<Span>, expected: impl MetaExpected, found: impl fmt::Display) -> Self {
        Self::custom(span, format_args!(
            "expected: {}, found {}",
            &expected as &dyn MetaExpected,
            found
        ))
    }

    fn invalid_value(span: Option<Span>, expected: impl MetaExpected, error: impl fmt::Display) -> Self {
        Self::custom(span, format_args!(
            "invalid value: {}, expected {}",
            error,
            &expected as &dyn MetaExpected,
        ))
    }
}

pub trait MetaExpected {
    fn expected(&self, f: &mut fmt::Formatter) -> fmt::Result;
}

impl<'a> fmt::Display for dyn MetaExpected + 'a {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.expected(f)
    }
}

impl<V> MetaExpected for V where V: MetaVisitor {
    fn expected(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.expecting(f)
    }
}
