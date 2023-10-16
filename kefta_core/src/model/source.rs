use std::borrow::Cow;
use std::marker::PhantomData;
use crate::model::from::{FromMeta, FromMetaCollect};
use crate::model::visitor::MetaVisitor;

pub trait MetaDomain {
    fn as_error_string(&self) -> Cow<str>;
}

pub trait MetaSource<T> where T: MetaDomain {
    type Error;

    fn visit<V>(self, visitor: V) -> Result<V::Output, Self::Error>
        where V: MetaVisitor<T>;
}

pub trait MetaAccess<T> where T: MetaDomain {
    type Error;

    fn next<R>(&mut self, receiver: R) -> Option<Result<R::Output, Self::Error>>
        where R: MetaReceiver<T>;

    fn next_from<F>(&mut self) -> Option<Result<F, Self::Error>>
        where F: FromMeta<T>
    {

        struct _Recv<_T, _F>(PhantomData<(_T, _F)>);

        impl<_T, _F> MetaReceiver<_T> for _Recv<_T, _F>
            where _T: MetaDomain, _F: FromMeta<_T>
        {
            type Output = _F;

            fn receive<S>(self, source: S) -> Result<Self::Output, S::Error> where S: MetaSource<_T> {
                _F::from_meta(source)
            }
        }

        self.next(_Recv(PhantomData))
    }

    fn next_from_collect<F>(&mut self, value: Option<F>) -> Option<Result<F, Self::Error>>
        where F: FromMetaCollect<T>
    {

        struct _Recv<_T, _F>(PhantomData<_T>, Option<_F>);

        impl<_T, _F> MetaReceiver<_T> for _Recv<_T, _F>
            where _T: MetaDomain, _F: FromMetaCollect<_T>
        {
            type Output = _F;

            fn receive<S>(self, source: S) -> Result<Self::Output, S::Error> where S: MetaSource<_T> {
                _F::from_meta_collect(self.1, source)
            }
        }

        self.next(_Recv(PhantomData, value))
    }

    fn next_visit<V>(&mut self, visitor: V) -> Option<Result<V::Output, Self::Error>>
        where V: MetaVisitor<T>
    {

        struct _Recv<_T, _V>(PhantomData<_T>, _V);

        impl<_T, _V> MetaReceiver<_T> for _Recv<_T, _V>
            where _T: MetaDomain, _V: MetaVisitor<_T>
        {
            type Output = _V::Output;

            fn receive<S>(self, source: S) -> Result<Self::Output, S::Error> where S: MetaSource<_T> {
                source.visit(self.1)
            }
        }

        self.next(_Recv(PhantomData, visitor))
    }
}

pub trait MetaReceiver<T> where T: MetaDomain {
    type Output;

    fn receive<S>(self, source: S) -> Result<Self::Output, S::Error>
        where S: MetaSource<T>;
}