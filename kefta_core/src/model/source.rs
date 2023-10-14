use std::marker::PhantomData;
use crate::model::error::MetaError;
use crate::model::{FromMeta, MetaReceiver};

/// a 'domain' of a source, representing it's value type
pub trait MetaDomain {
    fn to_string(&self) -> String;
}

/// a source of a meta tree
///
/// this trait is generic over 'D', representing it's value type
pub trait MetaSource<Domain> where Domain: MetaDomain {
    /// the error that this source may produce.
    type Error: MetaError;

    /// drive a `MetaVisitor` from a source.
    fn visit<V>(self, visitor: V) -> Result<V::Output, Self::Error>
        where V: MetaReceiver<Domain=Domain>;
}

/// access to a list of a meta trees
///
/// this trait is generic over 'D', representing it's value type
pub trait MetaAccess<Domain> where Domain: MetaDomain {

    /// the error that this source may produce.
    type Error: MetaError;

    /// whether the access has values remaining.
    ///
    /// this is intended to be used in a while loop
    ///
    /// ```
    /// # struct _ExampleShim;
    /// # impl _ExampleShim { fn remaining(&self) -> bool { false } }
    /// # let access = _ExampleShim;
    ///
    /// while access.remaining() {
    ///     /* ... */
    /// }
    fn remaining(&mut self) -> bool;

    /// visit the next value in the list
    ///
    /// when no values are remaining, this returns `None`
    fn visit_next<V>(&mut self, visitor: V) -> Result<V::Output, Self::Error>
        where V: MetaReceiver<Domain=Domain>;
}

impl<'a, Domain, A> MetaSource<Domain> for &'a mut A
    where A: MetaAccess<Domain>, Domain: MetaDomain
{
    type Error = A::Error;

    fn visit<V>(self, visitor: V) -> Result<V::Output, Self::Error> where V: MetaReceiver<Domain=Domain> {
        self.visit_next(visitor)
    }
}