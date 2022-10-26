use proc_macro::{TokenStream, TokenTree};
use std::iter::IntoIterator;
use std::iter::Peekable;

pub struct ParseTokenStream {
    pub inner: Peekable<<TokenStream as IntoIterator>::IntoIter>
}

impl ParseTokenStream {
    pub fn wrap(stream: TokenStream) -> Self {
        Self {
            inner: stream.into_iter().peekable()
        }
    }

    pub fn peek(&mut self) -> Option<&TokenTree> {
        self.inner.peek()
    }

    pub fn next(&mut self) -> Option<TokenTree> {
        self.inner.next()
    }

    pub fn skip(&mut self) {
        let _ = self.next();
    }
}