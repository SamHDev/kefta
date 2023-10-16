use crate::model::source::{MetaDomain, MetaSource};

pub trait FromMeta<T> where T: MetaDomain, Self: Sized {
    fn from_meta<S>(source: S) -> Result<Self, S::Error>
        where S: MetaSource<T>;
}

pub trait FromMetaCollect<T> where T: MetaDomain, Self: Sized {
    fn from_meta_collect<S>(value: Option<Self>, source: S) -> Result<Self, S::Error>
        where S: MetaSource<T>;
}

/*
impl<T, F> FromMeta<T> for F where T: MetaDomain, F: FromMetaCollect<T> {
    fn from_meta<S>(source: S) -> Result<Self, S::Error> where S: MetaSource<T> {
        F::from_meta_collect(None, source)
    }
}*/

