use std::fmt::{Debug, Formatter};
use proc_macro2::{Delimiter, Ident, Span, TokenTree};

pub type KeftaResult<T> = Result<T, KeftaError>;

#[derive(Debug)]
pub enum KeftaTokenError {
    ExpectedToken,
    Expected {
        expected: &'static str,
        description: Option<&'static str>,
        found: TokenTree
    },
    Message(&'static str, Span)
}

pub enum KeftaError {
    TokenError(KeftaTokenError),

    ExpectedMarker { ident: Ident },
    ExpectedValue { ident: Ident },
    ExpectedContainer { ident: Ident },

    Expected {
        expected: KeftaExpected,
        span: Span,
    },

    Multiple {
        key: String,
        count: usize,
    },

    Required {
        key: String,
        multiple: bool,
    },

    Message {
        message: String,
        span: Option<Span>,
    },

    #[cfg(feature = "syn")]
    Syn(syn::Error)
}

#[derive(Debug)]
pub enum KeftaExpected {
    Literal,
    Punct,
    Ident,
    Group,

    Delimiter(Delimiter),

    CharLiteral,
    StringLiteral,
    ByteLiteral,
    NumericLiteral,
    BooleanLiteral,
}


impl Debug for KeftaError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            KeftaError::TokenError(error) => Debug::fmt(&error, f),
            KeftaError::Syn(x) => Debug::fmt(&x, f),

            KeftaError::ExpectedMarker { ident } =>
                f.debug_tuple("ExpectedMarker")
                    .field(ident)
                    .finish(),
            KeftaError::ExpectedValue { ident } =>
                f.debug_tuple("ExpectedValue")
                    .field(ident)
                    .finish(),
            KeftaError::ExpectedContainer { ident } =>
                f.debug_tuple("ExpectedContainer")
                    .field(ident)
                    .finish(),
            KeftaError::Expected { expected, span } =>
                f.debug_tuple("ExpectedValue")
                    .field(expected)
                    .field(span)
                    .finish(),
            KeftaError::Multiple { key, .. } =>
                f.debug_tuple("ExpectedMultiple")
                    .field(key)
                    .finish(),
            KeftaError::Required { key, .. } =>
                f.debug_tuple("RequiredAttr")
                    .field(key)
                    .finish(),
            KeftaError::Message { message, .. } => Debug::fmt(&message, f)
        }
    }
}

#[cfg(feature="syn")]
impl Into<syn::Error> for KeftaError {
    fn into(self) -> syn::Error {
        syn::Error::new(Span::call_site(), format!("{:?}", self))
    }
}


