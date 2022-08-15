//! library error types

use std::fmt::{Debug, Formatter};
use proc_macro2::{Delimiter, Ident, Span, TokenTree};

/// alias for `Result<T, KeftaError>`
pub type KeftaResult<T> = Result<T, KeftaError>;

/// an error for parsing tokens into nodes
#[derive(Debug)]
pub enum KeftaTokenError {
    /// expected a token, found end of stream
    ExpectedToken { span: Span },
    /// expected a type, found otherwise
    Expected {
        /// the expected type
        expected: &'static str,
        /// further description
        description: Option<&'static str>,
        /// the tokens found
        found: TokenTree
    },
    /// a generic message
    Message(&'static str, Span)
}

pub enum KeftaError {
    /// an error while parsing tokens
    TokenError(KeftaTokenError),

    /// expected a marker, found otherwise
    ExpectedMarker { ident: Ident },
    /// expected a value, found otherwise
    ExpectedValue { ident: Ident },
    /// expected a container, found otherwise
    ExpectedContainer { ident: Ident },

    /// expected a type at the given span, found otherwise
    Expected {
        /// the expected type
        expected: KeftaExpected,
        /// the span
        span: Span,
    },

    /// found multiple nodes, but only expected one.
    Multiple {
        key: String,
        count: usize,
        span: Span,
    },

    /// the node is required, but was not found.
    Required {
        key: String,
        multiple: bool,
    },

    /// a generic message
    Message {
        message: String,
        span: Option<Span>,
    },

    /// an error for `syn` compatibility
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
        match self {
            KeftaError::Syn(e) => e,

            KeftaError::TokenError(error) => match error {
                KeftaTokenError::ExpectedToken { span } =>
                    syn::Error::new(span, "expected a token, found the end of the stream"),
                KeftaTokenError::Expected { expected, description, found } =>
                    syn::Error::new(found.span(), format!(
                        "expected {}{} but found {}",
                        expected,
                        match description {
                            None => ",".to_string(),
                            Some(desc) => format!(" [{}]", desc.to_string()),
                        },
                        match found {
                            TokenTree::Ident(ident) => format!("ident `{}`", ident.to_string()),
                            TokenTree::Punct(punct) => format!("a `{}` token", punct.as_char()),
                            TokenTree::Literal(literal) => format!("literal `{}`", literal),
                            TokenTree::Group(group) => format!("a `{}` group", delimiter_str(group.delimiter())),
                        },
                    )),
                KeftaTokenError::Message(msg, span) => syn::Error::new(span, msg)
            },

            KeftaError::ExpectedMarker { ident } =>
                syn::Error::new(ident.span(), "expected a `marker` type attribute"),
            KeftaError::ExpectedValue { ident } =>
                syn::Error::new(ident.span(), "expected a value"),
            KeftaError::ExpectedContainer { ident } =>
                syn::Error::new(ident.span(), "expected a `container` attribute"),

            KeftaError::Expected { expected, span } =>
                syn::Error::new(span, format!(
                    "expected `{}`",
                    match expected {
                        KeftaExpected::Literal => "a literal",
                        KeftaExpected::Punct => "a punct token",
                        KeftaExpected::Ident => "an identifier",
                        KeftaExpected::Group => "a group",
                        KeftaExpected::Delimiter(_delimiter) => "a group", // todo
                        KeftaExpected::CharLiteral => "a character literal (char)",
                        KeftaExpected::StringLiteral => "a string literal",
                        KeftaExpected::ByteLiteral => "a byte-string literal (b\"\")",
                        KeftaExpected::NumericLiteral => "a numeric literal",
                        KeftaExpected::BooleanLiteral => "a boolean literal (true/false)"
                    }
                )),

            KeftaError::Multiple { key, count, span } =>
                syn::Error::new(span, format!(
                    "found {} occurrences for `{}`, but only expected one",
                    count,
                    key
                )),

            KeftaError::Required { key, .. } =>
                syn::Error::new(Span::call_site(), format!(
                    "the attribute `{}` is required", key
                )),

            KeftaError::Message { message, span } =>
                syn::Error::new(
                    span.unwrap_or_else(|| Span::call_site()),
                    message
                ),

            //this @ _ => syn::Error::new(Span::call_site(), format!("{:?}", this))
        }
    }
}


fn delimiter_str(delimiter: Delimiter) -> &'static str {
    match delimiter {
        Delimiter::Parenthesis => "()",
        Delimiter::Brace => "{}",
        Delimiter::Bracket => "[]",
        Delimiter::None => "_",
    }
}