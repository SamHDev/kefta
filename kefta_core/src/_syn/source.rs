use std::collections::VecDeque;
use std::fmt::Display;
use std::iter::Peekable;
use std::mem;
use std::vec::IntoIter;
use proc_macro2::{Span, TokenStream};
use quote::ToTokens;
use syn::{Attribute, Meta, Path};
use syn::parse::Parser;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::Comma;
use crate::model::{MetaAccess, MetaError, MetaErrorFmt, MetaSource, MetaVisitor};


impl MetaError for syn::Error {
    fn custom(message: impl MetaErrorFmt, at: Option<Span>) -> Self {
        syn::Error::new(
            at.unwrap_or(Span::call_site()),
            message.as_fmt()
        )
    }
}

impl MetaSource for Attribute {
    type Error = syn::Error;

    fn visit<V: MetaVisitor>(self, visitor: V) -> Result<V::Output, Self::Error> {
        self.meta.visit(visitor)
    }
}

impl MetaSource for Meta {
    type Error = syn::Error;

    fn visit<V: MetaVisitor>(mut self, visitor: V) -> Result<V::Output, Self::Error> {
        match self {
            Meta::Path(x) => {
                let span1 = x.segments.last().map(|x| x.span());
                let span2 = x.leading_colon.as_ref().map(|x| x.span());
                let span = span1.or(span2);

                SynPathVisitor::new(
                    x,
                    SynWordSource(span)
                ).visit(visitor)
            },

            Meta::List(x) => SynPathVisitor::new(
                x.path,
                SynListAccess(
                    x.tokens.span(),
                    Parser::parse2(
                        Punctuated::<Meta, Comma>::parse_separated_nonempty,
                        x.tokens
                    )?.into_iter().peekable()
                )
            ).visit(visitor),

            Meta::NameValue(x) => SynPathVisitor::new(
                x.path,
                SynValueSource(ToTokens::to_token_stream(&x.value))
            ).visit(visitor)
        }
    }
}

pub struct SynPathVisitor<T> where T: MetaSource {
    leading: Option<Span>,
    segments: VecDeque<syn::Ident>,
    rest: T
}

impl<T> SynPathVisitor<T> where T: MetaSource {
    pub fn new(path: Path, rest: T) -> Self {
        Self {
            leading: path.leading_colon.map(|x| x.span()),
            segments: path.segments.into_iter().map(|x| x.ident).collect(),
            rest
        }
    }
}

impl<T> MetaSource for SynPathVisitor<T> where T: MetaSource<Error=syn::Error> {
    type Error = syn::Error;


    fn visit<V: MetaVisitor>(mut self, visitor: V) -> Result<V::Output, Self::Error> {
        if let Some(span) = mem::take(&mut self.leading) {
            visitor.visit_path(self, None, Some(span))
        } else if let Some(ident) = self.segments.pop_front() {
            visitor.visit_path(self, Some(&ident.to_string()), Some(ident.span()))
        } else {
            self.rest.visit(visitor)
        }
    }
}

pub struct SynWordSource(Option<Span>);

impl MetaSource for SynWordSource {
    type Error = syn::Error;

    fn visit<V: MetaVisitor>(self, visitor: V) -> Result<V::Output, Self::Error> {
        visitor.visit_word(self.0)
    }
}

pub struct SynValueSource(TokenStream);

impl MetaSource for SynValueSource {
    type Error = syn::Error;

    fn visit<V: MetaVisitor>(self, visitor: V) -> Result<V::Output, Self::Error> {
        let span = self.0.span();
        visitor.visit_value(self.0, Some(span))
    }
}

pub struct SynListAccess(Span, Peekable<<Punctuated<Meta, Comma> as IntoIterator>::IntoIter>);

impl MetaSource for SynListAccess {
    type Error = syn::Error;

    fn visit<V: MetaVisitor>(self, visitor: V) -> Result<V::Output, Self::Error> {
        let span = self.0.clone();
        visitor.visit_list(self, Some(span))
    }
}

impl MetaAccess for SynListAccess {
    type Error = syn::Error;

    fn remaining(&mut self) -> bool {
        self.1.peek().is_some()
    }

    fn visit<V: MetaVisitor>(&mut self, visitor: V) -> Result<V::Output, Self::Error> {
        if let Some(meta) = self.1.next() {
            meta.visit(visitor)
        } else {
            panic!("meta access has no remaining");
        }
    }
}