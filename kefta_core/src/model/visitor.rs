use core::fmt;
use proc_macro2::Span;
use crate::model::MetaError;
use crate::model::source::{MetaAccess, MetaDomain, MetaSource};

pub trait MetaVisitor: Sized {
    /// the return value for this visitor
    type Output;

    /// the domain for this visitor
    type Domain: MetaDomain;

    /// the type of value this visitor was expecting.
    ///
    /// - in the format `"expecting: <FMT>"`
    /// - automagically used in non-implemented `visit_*` methods.
    ///
    /// ```
    /// use std::fmt;
    ///
    /// # struct _Shim;
    /// # impl _Shim {
    /// fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
    ///     f.write_str("a valid value") // expected: a valid value
    /// }
    /// # }
    /// ```
    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result;


    /// visit a path segment
    ///
    /// contains:
    /// - the (optional) span of segment
    /// - the identifier of the segment, None denotes a leading colon.
    /// - the source value contained within this segment.
    fn visit_path<S>(
        self,
        span: Option<Span>,
        path: Option<&str>,
        source: S
    ) -> Result<Self::Output, S::Error>
        where S: MetaSource<Self::Domain> 
    {
        Err(S::Error::expecting(span, self, match path {
            None => format_args!("a leading colon"),
            Some(x) => format_args!("path ({x})"),
        }))
    }

    /// visit a marker segment
    ///
    /// denotes the end of an path with no value.
    ///
    /// contains:
    /// - the (optional) span of segment
    /// - an error type (generic `E`)
    fn visit_marker<E>(
        self,
        span: Option<Span>,
    ) -> Result<Self::Output, E>
        where E: MetaError 
    {
        Err(E::expecting(span, self, "a marker"))
    }

    /// visit a value segment
    ///
    /// denotes the end of an path with a value.
    ///
    /// contains:
    /// - the (optional) span of segment
    /// - the value of the segment (as the domain)
    /// - an error type (generic `E`)
    fn visit_value<E>(
        self,
        span: Option<Span>,
        value: Self::Domain,
    ) -> Result<Self::Output, E>
        where E: MetaError 
    {
        Err(E::expecting(span, self, format_args!(
            "value ({})",
            value.as_error_display()
        )))
    }

    /// visit a list segment
    ///
    /// denotes the end of an path with a list.
    ///
    /// contains:
    /// - the (optional) span of segment
    /// - the value of the segment (as the domain)
    /// - an error type (generic `E`)
    fn visit_list<A>(
        self,
        span: Option<Span>,
        access: A
    ) -> Result<Self::Output, A::Error>
        where A: MetaAccess<Self::Domain> 
    {
        Err(A::Error::expecting(span, self, format_args!("a list")))
    }
}