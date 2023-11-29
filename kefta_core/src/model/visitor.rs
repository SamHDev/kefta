use std::fmt;
use std::fmt::Formatter;
use std::task::ready;
use proc_macro2::Span;
use crate::model::error::{MetaError, MetaExpected};
use crate::model::parser::{MetaFlavour, MetaParser};

pub trait MetaVisitor: Sized {
    type Flavour: MetaFlavour;
    type Output;

    fn expecting(&self, f: &mut Formatter) -> fmt::Result;

    #[allow(unused_variables)]
    fn visit_marker<E>(
        self,
        span: Option<Span>
    ) -> Result<Self::Output, E>
        where E: MetaError
    {
        Err(E::expecting(
            span,
            &self as &dyn MetaExpected,
            "marker value"
        ))
    }

    #[allow(unused_variables)]
    fn visit_path<P>(
        self,
        span: Option<Span>,
        path: Option<&str>,
        child: P
    ) -> Result<Self::Output, P::Error>
        where P: MetaParser<Self::Flavour>
    {
        Err(P::Error::expecting(
            span,
            &self as &dyn MetaExpected,
            "marker value"
        ))
    }

    fn visit_value<E>(
        self,
        span: Option<Span>,
        value: Self::Flavour,
    ) -> Result<Self::Output, E>
        where E: MetaError
    {
        Err(E::expecting(
            span,
            &self as &dyn MetaExpected,
            &value as &dyn MetaFlavour
        ))
    }

}