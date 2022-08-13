use proc_macro2::{Ident, Span, TokenTree};

pub type KeftaResult<T> = Result<T, KeftaError>;

pub enum KeftaError {
    Wrapped(syn::Error),
    Message(&'static str, Span),

    ExpectedToken,
    Expected {
        expected: KeftaExpected,
        found: TokenTree
    },

    ExpectedValue(Ident),
    ExpectedMarker(Ident),
}

pub enum KeftaExpected {
    None,
    Ident,
    Punct,
    Group,
    Literal,
    String,
    Number,
    Message(&'static str),
}

impl From<syn::Error> for KeftaError {
    fn from(x: syn::Error) -> Self {
        Self::Wrapped(x)
    }
}