use std::marker::PhantomData;
use proc_macro2::{Delimiter, Ident, Punct, Spacing, Span, TokenStream, TokenTree};
use crate::domains::builtin::error::BuiltinParserError;
use crate::domains::builtin::source::{BuiltinAccess, BuiltinContSource};
use crate::model::{MetaError, MetaVisitor};

pub(crate) struct BuiltinParser<E>(<TokenStream as IntoIterator>::IntoIter, Option<Span>, PhantomData<E>)
    where E: MetaError;

impl<E> BuiltinParser<E> where E: MetaError {
    pub fn new(stream: TokenStream, last_span: Option<Span>) -> Self {
        Self (
            stream.into_iter(),
            last_span,
            PhantomData
        )
    }

    pub fn next(&mut self) -> Option<TokenTree> {
        match self.0.next() {
            None => None,
            Some(x) => {
                self.1 = Some(x.span());
                Some(x)
            }
        }
    }

    /// end of input error
    fn eoi(&self) -> BuiltinParserError<E> {
        BuiltinParserError::EndOfInput { span: self.1 }
    }


    /// parse a ident within a path
    fn parse_ident(&mut self) -> Result<Ident, BuiltinParserError<E>>
    {
        match self.next() {
            None => Err(self.eoi()),
            Some(TokenTree::Ident(ident)) => Ok(ident),
            Some(token) => Err(BuiltinParserError::ExpectedIdent { token })
        }
    }

    fn parse_path_delimiter(&mut self, first: Punct) -> Result<Span, BuiltinParserError<E>> {
        match self.next() {
            None => Err(self.eoi()),
            Some(TokenTree::Punct(second)) if second.as_char() == ':' =>
                Ok(first.span().join(second.span()).unwrap_or(second.span())),
            Some(token) => Err(BuiltinParserError::ExpectedTailfish { token, first })
        }
    }

    /// parse item
    pub fn parse_root_item<V>(&mut self, token: Option<TokenTree>, visitor: V) -> Result<V::Output, BuiltinParserError<E>>
        where V: MetaVisitor<Domain=TokenTree>
    {
        match token {
            None => Err(self.eoi()),

            Some(TokenTree::Punct(punct)) if _is_path_delimiter(&punct) => {
                let delimiter = self.parse_path_delimiter(punct)?;
                visitor.visit_path(
                    Some(delimiter),
                    None,
                    BuiltinContSource { parser: self }
                )
            },

            Some(TokenTree::Ident(ident)) => {
                visitor.visit_path(
                    Some(ident.span()),
                    Some(&ident.to_string()),
                    BuiltinContSource { parser: self }
                )
            }

            Some(token) => Err(BuiltinParserError::InvalidRootExpr { token })
        }
    }

    /// parse item
    pub fn parse_item<V>(&mut self, token: Option<TokenTree>, visitor: V) -> Result<V::Output, BuiltinParserError<E>>
        where V: MetaVisitor<Domain=TokenTree>
    {
        match token {
            None => Err(self.eoi()),

            Some(TokenTree::Punct(punct)) if _is_path_delimiter(&punct) => {
                let delimiter = self.parse_path_delimiter(punct)?;
                visitor.visit_path(
                    Some(delimiter),
                    None,
                    BuiltinContSource { parser: self }
                )
            },

            Some(TokenTree::Ident(ident)) => {
                visitor.visit_path(
                    Some(ident.span()),
                    Some(&ident.to_string()),
                    BuiltinContSource { parser: self }
                )
            }

            Some(token) =>
                visitor.visit_value(Some(token.span()), token)
                    .map_err(BuiltinParserError::Error)
        }

    }

    pub fn parse_item_cont<V>(&mut self, token: Option<TokenTree>, visitor: V) -> Result<V::Output, BuiltinParserError<E>>
        where V: MetaVisitor<Domain=TokenTree>
    {
        match token {
            None => visitor.visit_marker(self.1),

            Some(TokenTree::Punct(punct)) if _is_path_delimiter(&punct) => {
                let _delimiter = self.parse_path_delimiter(punct)?;
                let ident = self.parse_ident()?;
                visitor.visit_path(
                    Some(ident.span()),
                    Some(&ident.to_string()),
                    BuiltinContSource { parser: self }
                )
            },

            Some(TokenTree::Punct(punct)) if punct.as_char() == '=' => {
                let Some(value) = self.next() else { return Err(self.eoi()); };
                visitor.visit_value(Some(value.span()), value)
                    .map_err(BuiltinParserError::Error)
            }

            Some(TokenTree::Group(group)) if group.delimiter() == Delimiter::Parenthesis => {
                let mut parser = BuiltinParser::new(group.stream(), Some(group.span()));
                visitor.visit_list(Some(group.span()), BuiltinAccess { parser: &mut parser })
            }

            Some(token) => Err(BuiltinParserError::InvalidExpr { token, })
        }
    }
}

fn _is_path_delimiter(x: &Punct) -> bool{
    x.as_char() == ':' && x.spacing() == Spacing::Joint
}