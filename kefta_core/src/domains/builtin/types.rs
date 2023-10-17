use crate::model::{FromMeta, MetaError, MetaSource, MetaVisitor};
use proc_macro2::{Group, Ident, Literal, Punct, Span, TokenTree};
use std::fmt::Formatter;

impl FromMeta<TokenTree> for TokenTree {
    fn from_meta<S>(source: S) -> Result<Self, S::Error>
    where
        S: MetaSource<TokenTree>,
    {
        struct _Visitor;

        impl MetaVisitor for _Visitor {
            type Output = TokenTree;
            type Domain = TokenTree;

            fn expecting(&self, f: &mut Formatter) -> std::fmt::Result {
                f.write_str("a value")
            }

            fn visit_value<E>(
                self,
                _span: Option<Span>,
                value: Self::Domain,
            ) -> Result<Self::Output, E>
            where
                E: MetaError,
            {
                Ok(value)
            }
        }

        source.visit(_Visitor)
    }
}

impl FromMeta<TokenTree> for Ident {
    fn from_meta<S>(source: S) -> Result<Self, S::Error>
    where
        S: MetaSource<TokenTree>,
    {
        struct _Visitor;

        struct _Marker<'a>(&'a str);

        impl<'a> MetaVisitor for _Marker {
            type Output = Ident;
            type Domain = TokenTree;

            fn expecting(&self, f: &mut Formatter) -> std::fmt::Result {
                f.write_str("an non-valued identifier")
            }

            fn visit_marker<E>(self, span: Option<Span>) -> Result<Self::Output, E>
            where
                E: MetaError,
            {
                Ok(Ident::new(self.0, span.unwrap_or_else(Span::call_site)))
            }
        }

        impl MetaVisitor for _Visitor {
            type Output = Ident;
            type Domain = TokenTree;

            fn expecting(&self, f: &mut Formatter) -> std::fmt::Result {
                f.write_str("an identifier")
            }

            fn visit_path<S>(
                self,
                span: Option<Span>,
                path: Option<&str>,
                source: S,
            ) -> Result<Self::Output, S::Error>
            where
                S: MetaSource<Self::Domain>,
            {
                match path {
                    None => Err(S::Error::expecting(span, self, "a leading tailfish")),
                    Some(x) => source.visit(_Marker(x)),
                }
            }

            fn visit_value<E>(
                self,
                span: Option<Span>,
                value: Self::Domain,
            ) -> Result<Self::Output, E>
            where
                E: MetaError,
            {
                match value {
                    TokenTree::Ident(ident) => Ok(ident),
                    x @ _ => Err(E::expecting(
                        span,
                        self,
                        format_args!("another token ({x})"),
                    )),
                }
            }
        }

        source.visit(_Visitor)
    }
}

impl FromMeta<TokenTree> for Literal {
    fn from_meta<S>(source: S) -> Result<Self, S::Error>
    where
        S: MetaSource<TokenTree>,
    {
        match TokenTree::from_meta(source)? {
            TokenTree::Literal(lit) => Ok(lit),
            token @ _ => Err(S::Error::expecting(
                Some(token.span()),
                "a literal value",
                format_args!("another token ({x})"),
            )),
        }
    }
}

impl FromMeta<TokenTree> for Group {
    fn from_meta<S>(source: S) -> Result<Self, S::Error>
    where
        S: MetaSource<TokenTree>,
    {
        match TokenTree::from_meta(source)? {
            TokenTree::Group(group) => Ok(group),
            token @ _ => Err(S::Error::expecting(
                Some(token.span()),
                "a group token",
                format_args!("another token ({x})"),
            )),
        }
    }
}

impl FromMeta<TokenTree> for bool {
    fn from_meta<S>(source: S) -> Result<Self, S::Error>
    where
        S: MetaSource<TokenTree>,
    {
        struct _Visitor;

        impl MetaVisitor for _Visitor {
            type Output = bool;
            type Domain = TokenTree;

            fn expecting(&self, f: &mut Formatter) -> std::fmt::Result {
                f.write_str("a marker")
            }

            fn visit_marker<E>(self, _span: Option<Span>) -> Result<Self::Output, E>
            where
                E: MetaError,
            {
                Ok(true)
            }
        }

        source.visit(_Visitor)
    }
}
