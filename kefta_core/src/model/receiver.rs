use std::fmt;

use proc_macro2::Span;

use crate::model::error::{MetaError, MetaFound};
use crate::model::source::{MetaAccess, MetaDomain, MetaSource};

/// a receiver for a meta type/value.
///
/// this type is generic over `V`, which denotes the 'value' type
pub trait MetaReceiver: Sized {
    type Domain: MetaDomain;

    /// the output/return type of the receiver.
    type Output;

    /// the expected value, implemented using `std:fmt`
    ///
    /// automagically called when a `visit_*` method is not implemented/overridden
    ///
    /// ```
    /// use std::fmt;
    /// # use kefta_core::model::MetaReceiver;
    /// # struct _Example;
    ///
    /// # impl MetaReceiver for _Example  {
    /// # type Domain = ();
    /// # type Output = ();
    ///
    /// fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
    ///     f.write_str("a string value") // expected: a string value
    /// }
    ///
    /// # }
    ///```
    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result;


    /// receive a 'path' type
    ///
    /// - `span` - an optional 'span' to the value.
    /// - `ident` - the path's identifier, `None` depicts a trailing colon.
    #[allow(unused_variables)]
    fn visit_path<S>(self, span: Option<Span>, ident: Option<String>, source: S) -> Result<Self::Output, S::Error>
        where S: MetaSource<Self::Domain>
    {
        Err(S::Error::expecting(
            span,
            self,
            MetaFound::Path(ident.as_deref())
        ))
    }

    /// receive a 'marker' type
    ///
    /// > e.g. `#[foo]`
    /// >
    /// > 1. visit_path: Some("foo")
    /// > 2. visit_marker
    ///
    /// `span` - an optional 'span' to the value.
    #[allow(unused_variables)]
    fn visit_marker<E>(self, span: Option<Span>) -> Result<Self::Output, E> where E: MetaError {
        Err(E::expecting(
            span,
            self,
            MetaFound::Marker
        ))
    }

    /// receive a 'value' type
    ///
    /// > e.g. `#[foo=10]`
    /// >
    /// > 1. visit_path: Some("foo")
    /// > 2. visit_value: T / 10
    ///
    /// - `span` - an optional 'span' to the value.
    /// - `value` - the value contents, generic over 'T'
    #[allow(unused_variables)]
    fn visit_value<E>(self, span: Option<Span>, value: Self::Domain) -> Result<Self::Output, E> where E: MetaError {
        Err(E::expecting(
            span,
            self,
            MetaFound::Value(&value.to_string())
        ))
    }

    /// receive a 'list' type
    ///
    /// > e.g. `#[foo(..)]`
    /// >
    /// > 1. visit_path: Some("foo")
    /// > 2. visit_list
    ///
    /// - `span` - an optional 'span' to the value.
    /// - `access` - a `MetaAccess` type, with the contents of the list.
    #[allow(unused_variables)]
    fn visit_list<A>(self, span: Option<Span>, access: A) -> Result<Self::Output, A::Error> where A: MetaAccess<Self::Domain> {
        Err(A::Error::expecting(
            span,
            self,
            MetaFound::List
        ))
    }
}

