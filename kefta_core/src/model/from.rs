use std::any::type_name;
use std::fmt::{Formatter, Write};
use std::marker::PhantomData;
use std::ptr::write;
use proc_macro2::Span;
use crate::model::{MetaAccess, MetaDomain, MetaReceiver, MetaSource};

pub trait FromMeta<Domain>: Sized where Domain: MetaDomain {
    fn from_meta<S>(source: S) -> Result<Self, S::Error>
        where S: MetaSource<Domain>;
}

pub trait FromMetaCollect<Domain>: Sized where Domain: MetaDomain {
    fn from_meta_collect<S>(value: Option<Self>, source: S) -> Result<Self, S::Error>
        where S: MetaSource<Domain>;
}

impl<T, Domain> FromMeta<Domain> for T  where Domain: MetaDomain, T: FromMetaCollect<Domain> {
    fn from_meta<S>(source: S) -> Result<Self, S::Error> where S: MetaSource<Domain> {
        T::from_meta_collect(None, source)
    }
}

impl<T, Domain> FromMetaCollect<Domain> for Vec<T> where Domain: MetaDomain, T: FromMeta<Domain> {
    fn from_meta_collect<S>(value: Option<Self>, source: S) -> Result<Self, S::Error> where S: MetaSource<Domain> {
        let mut value = value.unwrap_or_else(Vec::new);

        struct _Recv<Q, R>(Vec<Q>, PhantomData<R>) where Q: FromMeta<R>, R: MetaDomain;

        impl<Q, R> MetaReceiver for _Recv<Q, R> where Q: FromMeta<R>, R: MetaDomain {
            type Domain = R;
            type Output = Vec<Q>;

            fn expecting(&self, f: &mut Formatter) -> std::fmt::Result {
                f.write_str("a list of ")?;
                f.write_str(type_name::<Q>())
            }

            fn visit_list<A>(self, span: Option<Span>, access: A) -> Result<Self::Output, A::Error> where A: MetaAccess<Self::Domain> {
                //while let Some(value) = access.next()

                Ok(self.0)
            }
        }

        source.visit(_Recv)
    }
}
