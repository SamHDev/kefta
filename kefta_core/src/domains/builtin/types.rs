use std::fmt::Formatter;
use proc_macro2::{Span, TokenTree};
use crate::model::{FromMeta, MetaError, MetaSource, MetaVisitor};

impl FromMeta<TokenTree> for TokenTree {
    fn from_meta<S>(source: S) -> Result<Self, S::Error> where S: MetaSource<TokenTree> {
        struct _Visitor;

        impl MetaVisitor for _Visitor {
            type Output = TokenTree;
            type Domain = TokenTree;

            fn expecting(&self, f: &mut Formatter) -> std::fmt::Result {
                f.write_str("a value")
            }

            fn visit_value<E>(self, _span: Option<Span>, value: Self::Domain) -> Result<Self::Output, E> where E: MetaError {
                Ok(value)
            }
        }

        source.visit(_Visitor)
    }
}