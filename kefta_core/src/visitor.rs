use std::fmt;
use proc_macro2::Span;
use crate::{MetaError, MetaExpected, MetaListParser, MetaParser};

pub trait MetaVisitor: Sized {
    type Output;
    type Type;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result;

    #[allow(unused_variables)]
    fn visit_path<P>(
        self,
        span: Option<Span>,
        path: &str,
        child: P
    ) -> Result<Self::Output, P::Error>
        where P: MetaParser<Type=Self::Type>
    {
        Err(P::Error::expected(
            span,
            self,
            format_args!("path '{path}'")
        ))
    }

    fn visit_marker<E>(
        self,
        span: Option<Span>,
    ) -> Result<Self::Output, E>
        where E: MetaError
    {
        Err(E::expected(
            span,
            self,
            format_args!("a non-valued marker")
        ))
    }

    #[allow(unused_variables)]
    fn visit_value<E>(
        self,
        span: Option<Span>,
        value: Self::Type,
    ) -> Result<Self::Output, E>
        where E: MetaError
    {
        Err(E::expected(
            span,
            self,
            "a value"
        ))
    }

    #[allow(unused_variables)]
    fn visit_list<P>(
        self,
        span: Option<Span>,
        parser: P,
    ) -> Result<Self::Output, P::Error>
        where P: MetaListParser<Type=Self::Type>
    {
        Err(P::Error::expected(
            span,
            self,
            "a nested list"
        ))
    }
}

impl<V> MetaExpected for V
    where V: MetaVisitor {
    fn expected(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.expecting(f)
    }
}