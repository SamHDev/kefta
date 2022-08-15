use std::iter::Peekable;
use proc_macro2::token_stream::IntoIter as TokenStreamIter;
use proc_macro2::{Span, TokenStream, TokenTree};
use crate::error::KeftaTokenError;
use crate::token::AttrTokenParse;

pub struct AttrTokenStream {
    last_span: Span,
    tokens: Peekable<TokenStreamIter>,
}


impl AttrTokenStream {
    pub fn new(stream: TokenStream) -> Self {
        Self {
            // not happy with this :(
            last_span: match stream.clone().into_iter().last() {
                None => Span::call_site(),
                Some(TokenTree::Group(x)) => x.span(),
                Some(TokenTree::Ident(x)) => x.span(),
                Some(TokenTree::Punct(x)) => x.span(),
                Some(TokenTree::Literal(x)) => x.span(),
            },
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

    pub fn stream_span(&self) -> Span {
        self.last_span.clone()
    }
}