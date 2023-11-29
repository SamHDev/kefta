use std::fmt;
use proc_macro2::{Span, TokenStream};
use crate::model::parser::MetaFlavour;
use crate::model::visitor::MetaVisitor;

pub trait MetaError: Sized {
    fn custom(span: Option<Span>, message: impl fmt::Display) -> Self;

    fn into_token_stream(self) -> TokenStream;

    fn expecting(span: Option<Span>, expected: impl fmt::Display, found: impl fmt::Display) -> Self {
        Self::custom(span, format_args!("expected: {expected}; found {found}"))
    }

    fn unknown_field(span: Option<Span>, field: impl fmt::Display, suggestions: Option<&[&str]>) -> Self {
        if let Some(suggestions) = suggestions {
            Self::custom(span, format_args!(
                "unknown field '{field}', did you mean {:?}",
                suggestions
            ))
        } else {
            Self::custom(span, format_args!("unknown field '{field}'"))
        }
    }
}

pub trait MetaExpected {
    fn expected(&self, f: &mut fmt::Formatter) -> fmt::Result;
}

impl<V> MetaExpected for V where V: MetaVisitor {
    fn expected(&self, f: &mut fmt::Formatter) -> fmt::Result {
        MetaVisitor::expecting(self, f)
    }
}

impl<'a> fmt::Display for &'a dyn MetaExpected {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        MetaExpected::expected(*self, f)
    }
}

/*pub trait MetaDisplay {
    fn display(&self, f: &mut fmt::Formatter) -> fmt::Result;
}

impl<F> MetaDisplay for F where F: MetaFlavour {
    fn display(&self, f: &mut fmt::Formatter) -> fmt::Result {
        MetaFlavour::error_fmt(self, f)
    }
}*/

impl<'a> fmt::Display for &'a dyn MetaFlavour {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        MetaFlavour::error_fmt(*self, f)
    }
}
