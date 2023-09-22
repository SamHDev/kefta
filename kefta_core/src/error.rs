use proc_macro2::{Span, TokenTree};

pub enum ParseError {
    InvalidPath {
        token: TokenTree
    },
    EmptyTree,
    UnexpectedEnd {
        last: Span
    },
    UnexpectedToken {
        position: Span,
        expected: Option<&'static str>,
        token: TokenTree
    }
}