use std::iter::Peekable;
use std::mem;

use proc_macro2::{Span, TokenStream};
use quote::ToTokens;
use syn::{Expr, Meta, MetaList, MetaNameValue, Path, PathSegment};
use syn::parse::Parser;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::{Comma, PathSep};

use crate::model::{MetaAccess, MetaDomain, MetaError, MetaReceiver, MetaSource};

type Error = syn::Error;

impl MetaDomain for Expr {
    fn to_string(&self) -> String {
        self.to_token_stream().to_string()
    }
}

impl MetaError for syn::Error {
    fn custom(span: Option<Span>, message: &str) -> Self {
        syn::Error::new(span.unwrap_or_else(Span::call_site), message)
    }
}

impl MetaSource<Expr> for Meta {
    type Error = Error;

    fn visit<V>(self, visitor: V) -> Result<V::Output, Self::Error> where V: MetaReceiver<Domain=Expr> {
        match self {
            Meta::Path(path) =>
                _SynPathSource::new(_SynMarkerSource { span: Some(path.span()) }, path)
                    .visit(visitor),

            Meta::List(MetaList { path, delimiter, tokens }) =>
                _SynPathSource::new(
                    _SynListSource {
                        span: Some(delimiter.span().span()),
                        tokens,
                    }, path).visit(visitor),
            Meta::NameValue(MetaNameValue { path, eq_token, value }) =>
                _SynPathSource::new(
                    _SynValueSource {
                        equals: eq_token.span,
                        value,
                    }, path).visit(visitor),
        }
    }
}

struct _SynPathSource<C> where C: MetaSource<Expr, Error=Error> {
    pub leading_colon: Option<PathSep>,
    pub segments: <Punctuated<PathSegment, PathSep> as IntoIterator>::IntoIter,
    pub cont: C,
}

impl<C> _SynPathSource<C> where C: MetaSource<Expr, Error=Error> {
    pub fn new(cont: C, path: Path) -> Self {
        Self {
            leading_colon: path.leading_colon,
            segments: path.segments.into_iter(),
            cont,
        }
    }
}

impl<C> MetaSource<Expr> for _SynPathSource<C> where C: MetaSource<Expr, Error=Error> {
    type Error = Error;

    fn visit<V>(mut self, visitor: V) -> Result<V::Output, Self::Error> where V: MetaReceiver<Domain=Expr> {
        if let Some(leading) = self.leading_colon.take() {
            visitor.visit_path(Some(leading.span()), None, self)
        } else if let Some(segment) = self.segments.next() {
            visitor.visit_path(Some(segment.span()), Some(segment.ident.to_string()), self)
        } else {
            self.cont.visit(visitor)
        }
    }
}

struct _SynMarkerSource {
    span: Option<Span>,
}

impl MetaSource<Expr> for _SynMarkerSource {
    type Error = Error;

    fn visit<V>(self, visitor: V) -> Result<V::Output, Self::Error> where V: MetaReceiver<Domain=Expr> {
        visitor.visit_marker(self.span)
    }
}

struct _SynListSource {
    span: Option<Span>,
    tokens: TokenStream,
}

impl MetaSource<Expr> for _SynListSource {
    type Error = Error;

    fn visit<V>(self, visitor: V) -> Result<V::Output, Self::Error> where V: MetaReceiver<Domain=Expr> {
        visitor.visit_list(self.span, _SynListAccess::Unparsed(Some(self.tokens)))
    }
}

enum _SynListAccess {
    Unparsed(Option<TokenStream>),
    Parsed(Peekable<<Punctuated<Meta, Comma> as IntoIterator>::IntoIter>),
}

impl MetaAccess<Expr> for _SynListAccess {
    type Error = Error;

    fn remaining(&mut self) -> bool {
        match self {
            _SynListAccess::Unparsed(x) => x.is_some(),
            _SynListAccess::Parsed(x) => x.peek().is_some(),
        }
    }

    fn visit_next<V>(&mut self, visitor: V) -> Result<V::Output, Self::Error> where V: MetaReceiver<Domain=Expr> {
        let parser = Punctuated::<Meta, Comma>::parse_separated_nonempty;

        let value = match self {
            _SynListAccess::Unparsed(tokens @ Some(_)) => {
                let tokens = tokens.take().expect("valid tokens in unparsed");
                match parser.parse2(tokens) {
                    Err(err) => return Err(err),
                    Ok(parsed) => {
                        let mut iter = parsed.into_iter().peekable();
                        let first = iter.next()?;
                        let _ = mem::replace(self, Self::Parsed(iter));
                        first
                    }
                }
            }
            _SynListAccess::Unparsed(None) => return Err(self::Error),
            _SynListAccess::Parsed(iter) => iter.next()?
        };

        value.visit(visitor)
    }
}

struct _SynValueSource {
    equals: Span,
    value: Expr,
}

impl MetaSource<Expr> for _SynValueSource {
    type Error = Error;

    fn visit<V>(self, visitor: V) -> Result<V::Output, Self::Error> where V: MetaReceiver<Domain=Expr> {
        visitor.visit_value(Some(self.equals), self.value)
    }
}