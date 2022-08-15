use proc_macro2::{Group, Ident, Literal, Punct, TokenTree};
use crate::error::KeftaTokenError;
use crate::token::AttrTokenStream;

pub trait AttrTokenParse: Sized {
    fn parse(stream: &mut AttrTokenStream) -> Result<Self, KeftaTokenError>;
}

impl AttrTokenParse for Option<TokenTree> {
    fn parse(stream: &mut AttrTokenStream) -> Result<Self, KeftaTokenError> {
        Ok(stream.next())
    }
}

impl AttrTokenParse for TokenTree {
    fn parse(stream: &mut AttrTokenStream) -> Result<Self, KeftaTokenError> {
        match stream.next() {
            Some(token_tree) => Ok(token_tree),
            None => Err(KeftaTokenError::ExpectedToken { span: stream.stream_span() }),
        }
    }
}

impl AttrTokenParse for Ident {
    fn parse(stream: &mut AttrTokenStream) -> Result<Self, KeftaTokenError> {
        match stream.parse::<TokenTree>()? {
            TokenTree::Ident(ident) => Ok(ident),
            token_tree @ _ => Err(KeftaTokenError::Expected {
                expected: "ident",
                description: None,
                found: token_tree
            })
        }
    }
}

impl AttrTokenParse for Punct {
    fn parse(stream: &mut AttrTokenStream) -> Result<Self, KeftaTokenError> {
        match stream.parse::<TokenTree>()? {
            TokenTree::Punct(punct) => Ok(punct),
            token_tree @ _ => Err(KeftaTokenError::Expected {
                expected: "punct",
                description: None,
                found: token_tree
            })
        }
    }
}

impl AttrTokenParse for Literal {
    fn parse(stream: &mut AttrTokenStream) -> Result<Self, KeftaTokenError> {
        match stream.parse::<TokenTree>()? {
            TokenTree::Literal(literal) => Ok(literal),
            token_tree @ _ => Err(KeftaTokenError::Expected {
                expected: "literal",
                description: None,
                found: token_tree
            })
        }
    }
}

impl AttrTokenParse for Group {
    fn parse(stream: &mut AttrTokenStream) -> Result<Self, KeftaTokenError> {
        match stream.parse::<TokenTree>()? {
            TokenTree::Group(group) => Ok(group),
            token_tree @ _ => Err(KeftaTokenError::Expected {
                expected: "group",
                description: None,
                found: token_tree
            })
        }
    }
}