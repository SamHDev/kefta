use std::fmt::Display;
use proc_macro2::{Punct, Span, TokenStream, TokenTree};
use crate::model::{MetaError, MetaExpected};

#[derive(Debug)]
pub enum BuiltinParserError<E> where E: MetaError {
    Error(E),

    EndOfInput { span: Option<Span> },

    /// invalid token when parsing the root expression
    InvalidRootExpr { token: TokenTree },
    ///  invalid expression when parsing value
    InvalidExpr { token: TokenTree },

    /// path segment was invalid
    ExpectedTailfish { token: TokenTree, first: Punct },

    /// expected path ident
    ExpectedIdent { token: TokenTree },

    ExpectedDelimiter { token: TokenTree }
}

impl<E> MetaError for BuiltinParserError<E> where E: MetaError {
    fn into_token_stream(self) -> TokenStream {
        self.into_error().into_token_stream()
    }

    fn custom(span: Option<Span>, message: impl Display) -> Self {
        Self::Error(E::custom(span, message))
    }

    fn expecting(span: Option<Span>, expected: impl MetaExpected, found: impl Display) -> Self {
        Self::Error(E::expecting(span, expected, found))
    }

    fn invalid_value(span: Option<Span>, expected: impl MetaExpected, error: impl Display) -> Self {
        Self::Error(E::invalid_value(span, expected, error))
    }
}

impl<E> BuiltinParserError<E> where E: MetaError {
    fn into_error(self) -> E {
        match self {
            BuiltinParserError::Error(e) => e,
            BuiltinParserError::EndOfInput { span } =>
                E::custom(span, "invalid sequence: unexpected end of input"),
            BuiltinParserError::InvalidRootExpr { token } |
            BuiltinParserError::InvalidExpr { token } =>
                E::custom(
                    Some(token.span()),
                    format_args!("invalid sequence: unexpected token when parsing meta-expression ({token})")
                ),
            BuiltinParserError::ExpectedTailfish { token, first } =>
                E::custom(
                    Some(token.span().join(first.span()).unwrap_or_else(|| token.span())),
                    format_args!("invalid sequence: expected path delimiter (`tailfish ::`), found ({token})")
                ),
            BuiltinParserError::ExpectedIdent { token } =>
                E::custom(
                    Some(token.span()),
                    format_args!("invalid sequence: expected path identifier, found ({token})")
                ),
            BuiltinParserError::ExpectedDelimiter { token } =>
                E::custom(
                    Some(token.span()),
                    format_args!("invalid sequence: expected delimiter (comma `,`), found ({token})")
                ),
        }
    }
}