use std::iter::Peekable;
use proc_macro2::token_stream::IntoIter as TokenStreamIter;
use proc_macro2::{TokenStream, TokenTree};
use crate::error::KeftaTokenError;
use crate::token::AttrTokenParse;

pub struct AttrTokenStream {
    tokens: Peekable<TokenStreamIter>,
}


impl AttrTokenStream {
    pub fn new(stream: TokenStream) -> Self {
        Self {
            tokens: stream.into_iter().peekable()
        }
    }

    pub fn new_tree(tree: TokenTree) -> Self {
        Self::new(TokenStream::from(tree))
    }

    pub fn next(&mut self) -> Option<TokenTree> {
        self.tokens.next()
    }

    pub fn peek(&mut self) -> Option<&TokenTree> {
        self.tokens.peek()
    }

    pub fn skip(&mut self) {
        let _ = self.tokens.next();
    }

    pub fn parse<T: AttrTokenParse>(&mut self) -> Result<T, KeftaTokenError> {
        T::parse(self)
    }

    pub fn has_tokens(&mut self) -> bool {
        self.tokens.peek().is_some()
    }
}