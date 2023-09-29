use std::fmt;
use std::fmt::{Display, Formatter, Write};
use proc_macro2::{Delimiter, Span, TokenTree};
use crate::model::MetaVisitor;

pub trait Error: Sized {
    fn custom(message: &dyn ErrorDisplay) -> Self;

    #[allow(unused_variables)]
    fn with_span(self, span: Span) -> Self { self }

    fn expected(expected: &dyn ErrorDisplay, found: &dyn ErrorDisplay) -> Self {
        Self::custom(&format!("expected {expected}, found {found}"))
    }

    fn empty_stream() -> Self {
        Self::custom("no tokens in stream")
    }
}

pub trait ErrorDisplay: Sized {
    fn description(&self, fmt: &mut fmt::Formatter) -> fmt::Result;
}

impl<'a, T> fmt::Display for dyn ErrorDisplay + 'a {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.description(f)
    }
}

impl<'a> ErrorDisplay for &'a str {
    fn description(&self, fmt: &mut Formatter) -> fmt::Result {
        fmt.write_str(self)
    }
}

impl ErrorDisplay for TokenTree {
    fn description(&self, fmt: &mut Formatter) -> std::fmt::Result {
        match self {
            TokenTree::Group(x) => {
                fmt.write_str("a group token `")?;
                fmt.write_str(match x.delimiter() {
                    Delimiter::Parenthesis => "()",
                    Delimiter::Brace => "{}",
                    Delimiter::Bracket => "[]",
                    Delimiter::None => "_"
                })?;
                fmt.write_str("`")?
            }
            TokenTree::Ident(x) => {
                fmt.write_str("ident (`")?;
                Display::fmt(x, fmt)?;
                fmt.write_str("`)")?;
            }
            TokenTree::Punct(x) => {
                fmt.write_str("punct (`")?;
                Display::fmt(x, fmt)?;
                fmt.write_str("`)")?;
            }
            TokenTree::Literal(x) => {
                fmt.write_str("literal (`")?;
                Display::fmt(x, fmt)?;
                fmt.write_str("`)")?;
            }
        };
        Ok(())
    }
}