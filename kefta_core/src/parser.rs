use std::marker::PhantomData;
use crate::{FromMeta, FromMetaSeeded, MetaError, MetaVisitor};

pub trait MetaParser {
    type Type;
    type Error: MetaError;

    fn parse_any<V>(self, visitor: V) -> Result<V::Output, Self::Error>
        where V: MetaVisitor<Type=Self::Type>;

    fn parse_path<V>(self, visitor: V) -> Result<V::Output, Self::Error>
        where V: MetaVisitor<Type=Self::Type>;
}

pub trait MetaListParser {
    type Type;
    type Error: MetaError;

    fn next_seeded<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
        where T: FromMetaSeeded<Self::Type>;

    fn next<T>(&mut self) -> Result<Option<T>, Self::Error>
        where T: FromMeta<Self::Type>
    {
        self.next_seeded(PhantomData::<T>)
    }

    fn visit_next<V>(&mut self, visitor: V) -> Result<Option<V::Output>, Self::Error>
        where V: MetaVisitor<Type=Self::Type>
    {
        struct _VisitNext<_V>(_V);

        impl<_V> FromMetaSeeded<_V::Type> for _VisitNext<_V> where _V: MetaVisitor {
            type Value = _V::Output;

            fn from_meta_seeded<P>(self, parser: P) -> Result<Self::Value, P::Error>
                where P: MetaParser<Type=_V::Type>
            {
                parser.parse_any(self.0)
            }
        }

        self.next_seeded(_VisitNext(visitor))
    }
}