use std::iter::Peekable;
use proc_macro2::{TokenStream, TokenTree, token_stream::IntoIter as TokenStreamIter};
use crate::KeftaResult;
use crate::parse::AttrStreamParse;

pub struct AttrStream {
    tokens: Peekable<TokenStreamIter>,
}


impl AttrStream {
    pub fn new(stream: TokenStream) -> Self {
        AttrStream {
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

    pub fn parse<T: AttrStreamParse>(&mut self) -> KeftaResult<T> {
        T::parse(self)
    }

    pub fn has_tokens(&mut self) -> bool {
        self.tokens.peek().is_some()
    }
}