use std::marker::PhantomData;
use crate::MetaParser;

pub trait FromMeta<Ty>: Sized {
    fn from_meta<P>(parser: P) -> Result<Self, P::Error>
        where P: MetaParser<Type=Ty>;
}

pub trait FromMetaSeeded<Ty>: Sized {
    type Value;

    #[allow(clippy::wrong_self_convention)]
    fn from_meta_seeded<P>(self, parser: P) -> Result<Self::Value, P::Error>
        where P: MetaParser<Type=Ty>;
}

impl<Ty, T> FromMetaSeeded<Ty> for PhantomData<T>
    where T: FromMeta<Ty>
{
    type Value = T;

    fn from_meta_seeded<P>(self, parser: P) -> Result<Self::Value, P::Error>
        where P: MetaParser<Type=Ty>
    {
        T::from_meta(parser)
    }
}

