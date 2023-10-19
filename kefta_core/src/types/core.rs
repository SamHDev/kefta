use std::fmt::{Formatter};
use std::marker::PhantomData;
use proc_macro2::{Span};
use crate::model::{FromMeta, FromMetaCollect, MetaAccess, MetaDomain, MetaError, MetaSource, MetaVisitor};

impl<D> FromMeta<D> for bool where D: MetaDomain {
    fn from_meta<S>(source: S) -> Result<Self, S::Error>
        where
            S: MetaSource<D>,
    {
        struct _Visitor<D1>(PhantomData<D1>);

        impl<D1> MetaVisitor for _Visitor<D1> where D1: MetaDomain {
            type Output = bool;
            type Domain = D1;

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

        source.visit(_Visitor(PhantomData))
    }
}


impl<D, T> FromMetaCollect<D> for Vec<T> where D: MetaDomain, T: FromMeta<D> {
    fn from_meta_collect<S>(value: Option<Self>, source: S) -> Result<Self, S::Error> where S: MetaSource<D> {
        let mut value = value.unwrap_or_default();

        struct _Visitor<'a, D1, T1>(&'a mut Vec<T1>, PhantomData<D1>);

        impl<'a, D1, T1> MetaVisitor for _Visitor<'a, D1, T1>
            where D1: MetaDomain, T1: FromMeta<D1>
        {
            type Output = ();
            type Domain = D1;

            fn expecting(&self, f: &mut Formatter) -> std::fmt::Result {
                f.write_fmt(format_args!("a list of {:?}", std::any::type_name::<T1>()))
            }

            fn visit_list<A>(self, _span: Option<Span>, mut access: A) -> Result<Self::Output, A::Error> where A: MetaAccess<Self::Domain> {
                while let Some(value) = access.next_from::<T1>() {
                    self.0.push(value?);
                }
                Ok(())
            }
        }

        source.visit(_Visitor(&mut value, PhantomData))?;
        Ok(value)
    }
}

impl<D, T> FromMeta<D> for Vec<T> where D: MetaDomain, T: FromMeta<D> {
    fn from_meta<S>(source: S) -> Result<Self, S::Error> where S: MetaSource<D> {
        <Vec<T> as FromMetaCollect<D>>::from_meta_collect(None, source)
    }
}