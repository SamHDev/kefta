use proc_macro2::{Delimiter, Ident, Span, TokenTree};

pub type KeftaResult<T> = Result<T, KeftaError>;

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

    Expected {
        expected: KeftaExpected,
        span: Span,
    },

    Message {
        message: String,
        span: Option<Span>,
    },
}

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