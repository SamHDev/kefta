use std::fmt;
use std::fmt::Formatter;
use std::marker::PhantomData;
use proc_macro2::{Span, TokenStream};
use crate::model::error::MetaError;
use crate::model::visitor::MetaVisitor;

pub trait MetaFlavour {
    fn error_fmt(&self, f: &mut fmt::Formatter) -> fmt::Result;
}

pub trait FromMeta<F>: Sized where F: MetaFlavour {
    fn from_meta<P>(parser: P) -> Result<Self, P::Error>
        where P: MetaParser<F>;
}

pub trait MetaParser<F> where F: MetaFlavour {
    type Error: MetaError;

    fn parse<V>(self, visitor: V) -> Result<V::Output, Self::Error>
        where V: MetaVisitor<Flavour=F>;
}

pub trait MetaNested {
    type Flavour: MetaFlavour;
    type Error: MetaError;

    type List: MetaList<Flavour=Self::Flavour, Error=Self::Error>;

    fn list(self) -> Result<Self::List, Self::Error>;

    fn tokens(self) -> Result<TokenStream, Self::Error>;
}

pub trait MetaList {
    type Flavour: MetaFlavour;
    type Error: MetaError;

    fn visit_next<V>(&mut self, visitor: V) -> Result<Option<V::Output>, Self::Error>
        where V: MetaVisitor;

    fn from_next<T>(&mut self) -> Result<Option<T>, Self::Error>
        where T: FromMeta<Self::Flavour>
    {
        struct _ListVisitor<_F, _T, _V>(PhantomData<(_F, _T)>, _V);

        impl<_F, _T, _V> MetaVisitor for _ListVisitor<_F, _T, _V>
            where _F: MetaFlavour, _V: MetaVisitor<Flavour=_F, Output=_T>
        {
            type Flavour = _F;
            type Output = Option<_T>;

            fn expecting(&self, f: &mut Formatter) -> fmt::Result {
                f.write_str("?")
            }

            fn visit_marker<E>(self, span: Option<Span>) -> Result<Self::Output, E>
                where E: MetaError
            {
                self.1.visit_marker(span).map(Some)
            }

            fn visit_path<P>(self, span: Option<Span>, path: Option<&str>, child: P) -> Result<Self::Output, P::Error>
                where P: MetaParser<Self::Flavour>
            {
                self.1.visit_path(span, path, child).map(Some)
            }

            fn visit_value<E>(self, span: Option<Span>, value: Self::Flavour) -> Result<Self::Output, E> where E: MetaError {
                self.1.visit_value(span, value).map(Some)
            }
        }

        struct _ListSource<'a, _F, _E, _Self>(PhantomData<(_F, _E)>, &'a mut _Self);

        impl<'a, _F, _E, _Self> MetaParser<_F> for _ListSource<'a, _F, _E, _Self>
            where _F: MetaFlavour, _E: MetaError, _Self: MetaList<Flavour=_F, Error=_E>
        {
            type Error = _E;

            fn parse<V>(self, visitor: V) -> Result<V::Output, Self::Error> where V: MetaVisitor<Flavour=_F> {
                self.1.visit_next(_ListVisitor(PhantomData, visitor))
            }
        }

        T::from_meta(_ListSource(PhantomData, &mut self))
    }
}