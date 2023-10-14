use std::fmt;

use proc_macro2::Span;

use crate::model::MetaReceiver;

/// an error type that a source can produce
pub trait MetaError: Sized {
    /// a custom error with a message.
    fn custom(span: Option<Span>, message: &str) -> Self;

    fn expecting(span: Option<Span>, expected: impl MetaExpected, found: MetaFound) -> Self {
        Self::custom(span, &format!(
            "expected {}, found {}",
            &expected as &dyn MetaExpected,
            found
        ))
    }

    fn invalid_value(span: Option<Span>, expected: impl MetaExpected, error: &str) -> Self {
        Self::custom(span, &format!(
            "invalid value: {}, expected {}",
            error,
            &expected as &dyn MetaExpected,
        ))
    }
}

pub trait MetaExpected {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result;
}

impl MetaExpected for str {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self)
    }
}

impl<T> MetaExpected for T where T: MetaReceiver {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        MetaReceiver::expecting(self, f)
    }
}

impl<'a> fmt::Display for dyn MetaExpected + 'a {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        MetaExpected::fmt(self, f)
    }
}

pub enum MetaFound<'a> {
    Custom(&'a str),

    Path(Option<&'a str>),
    Marker,
    Value(&'a str),
    List
}

impl<'a> fmt::Display for MetaFound<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MetaFound::Custom(x) => f.write_str(x),

            MetaFound::Path(Some(x)) => {
                f.write_str("path (")?;
                f.write_str(x)?;
                f.write_str(")")
            },
            MetaFound::Path(None) => {
                f.write_str("path (prefixed colon)")
            }
            MetaFound::Marker => {
                f.write_str("non-valued marker")
            }
            MetaFound::Value(x) => {
                f.write_str("value (")?;
                f.write_str(x)?;
                f.write_str(")")
            }
            MetaFound::List => {
                f.write_str("nested list")
            }
        }
    }
}