//! library error types

use std::fmt::{Debug, Formatter};
use proc_macro2::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};

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

/// an error while parsing attributes
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

/// an expected type in an error
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
            #[cfg(feature = "syn")]
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

impl KeftaError {
    pub fn build(self) -> (Span, String) {
        match self {
            #[cfg(feature = "syn")]
            KeftaError::Syn(e) => (e.span(), e.to_string()),

            KeftaError::TokenError(error) => match error {
                KeftaTokenError::ExpectedToken { span } =>
                    (span, "expected a token, found the end of the stream".to_string()),
                KeftaTokenError::Expected { expected, description, found } =>
                    (found.span(), format!(
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
                KeftaTokenError::Message(msg, span) => (span, msg.to_string())
            },

            KeftaError::ExpectedMarker { ident } =>
                (ident.span(), "expected a `marker` type attribute".to_string()),
            KeftaError::ExpectedValue { ident } =>
                (ident.span(), "expected a value".to_string()),
            KeftaError::ExpectedContainer { ident } =>
                (ident.span(), "expected a `container` attribute".to_string()),

            KeftaError::Expected { expected, span } =>
                (span, format!(
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
                (span, format!(
                    "found {} occurrences for `{}`, but only expected one",
                    count,
                    key
                )),

            KeftaError::Required { key, .. } =>
                (Span::call_site(), format!(
                    "the attribute `{}` is required", key
                )),

            KeftaError::Message { message, span } =>
                (span.unwrap_or_else(|| Span::call_site()), message),

            //this @ _ => syn::Error::new(Span::call_site(), format!("{:?}", this))
        }
    }

    pub fn to_compile_error(self) -> TokenStream {
        // from https://docs.rs/syn/latest/src/syn/error.rs.html#248
        let (span, msg) = self.build();

        // compile_error!($message)
        TokenStream::from_iter(vec![
            TokenTree::Ident(Ident::new("compile_error", span.clone())),
            TokenTree::Punct({
                let mut punct = Punct::new('!', Spacing::Alone);
                punct.set_span(span.clone());
                punct
            }),
            TokenTree::Group({
                let mut group = Group::new(Delimiter::Brace, {
                    TokenStream::from_iter(vec![TokenTree::Literal({
                        let mut string = Literal::string(&msg);
                        string.set_span(span.clone());
                        string
                    })])
                });
                group.set_span(span.clone());
                group
            }),
        ])
    }

    #[cfg(feature="syn")]
    pub fn into_syn(self) -> syn::Error {
        match self {
            KeftaError::Syn(e) => e,
            e @ _ => {
                let (span, msg) = e.build();
                syn::Error::new(span, msg)
            }
        }
    }
}

#[cfg(feature="syn")]
impl Into<syn::Error> for KeftaError {
    fn into(self) -> syn::Error {
        self.into_syn()
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