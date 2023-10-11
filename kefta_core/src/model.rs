use std::fmt;
use std::iter::Peekable;
use proc_macro2::{ Span, TokenStream};


pub trait FromMeta: Sized {
    fn from_meta<S>(source: S) -> Result<Self, S::Error> where S: MetaSource;
}

pub trait MetaVisitor: Sized {
    type Output;

    fn expecting(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result;

    fn visit_path<S>(self, source: S, path: Option<&str>, span: Option<Span>) -> Result<Self::Output, S::Error>
        where S: MetaSource
    {
        let _ = source;

        Err(S::Error::expected(
            &self,
            format!("path ({})", path.unwrap_or("::")).as_str(),
            span
        ))
    }

    fn visit_word<E>(self, span: Option<Span>) -> Result<Self::Output, E>
        where E: MetaError
    {
        Err(E::expected(
            &self,
            "word",
            span
        ))
    }

    fn visit_value<E>(self, tokens: TokenStream, span: Option<Span>) -> Result<Self::Output, E>
        where E: MetaError
    {
        Err(E::expected(
            &self,
            format!("value ({})", tokens).as_str(),
            span
        ))
    }

    fn visit_list<A>(self, access: A, span: Option<Span>) -> Result<Self::Output, A::Error>
        where A: MetaAccess
    {
        Err(A::Error::expected(
            &self,
            "list of meta",
            None
        ))
    }
}

pub trait MetaSource {
    type Error: MetaError;

    fn visit<V: MetaVisitor>(self, visitor: V) -> Result<V::Output, Self::Error>;
}

pub trait MetaAccess {
    type Error: MetaError;

    fn remaining(&mut self) -> bool;

    fn visit<V: MetaVisitor>(&mut self, visitor: V) -> Result<V::Output, Self::Error>;
}


impl<T> MetaSource for Vec<T> where T: MetaSource {
    type Error = T::Error;

    fn visit<V: MetaVisitor>(self, visitor: V) -> Result<V::Output, Self::Error> {
        struct _Access<U>(Peekable<<Vec<U> as IntoIterator>::IntoIter>);

        impl<U> MetaAccess for _Access<U> where U: MetaSource {
            type Error = U::Error;

            fn remaining(&mut self) -> bool {
                self.0.peek().is_some()
            }

            fn visit<V: MetaVisitor>(&mut self, visitor: V) -> Result<V::Output, Self::Error> {
                if let Some(x) = self.0.next() {
                    x.visit(visitor)
                } else {
                    unreachable!()
                }
            }
        }

        visitor.visit_list(_Access(self.into_iter().peekable()), None)
    }
}

pub trait MetaError: Sized {
    fn custom(message: impl MetaErrorFmt, at: Option<Span>) -> Self;

    fn expected(expected: impl MetaErrorFmt, found: impl MetaErrorFmt, at: Option<Span>) -> Self {
        Self::custom(format!("expected `{}`, found `{}`", expected.as_fmt(), found.as_fmt()).as_str(), at)
    }
}

pub struct MetaErrorFormatter<'a, T: MetaErrorFmt>(&'a T);

impl<'a, T: MetaErrorFmt> fmt::Display for MetaErrorFormatter<'a, T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.format(f)
    }
}

impl<'a, T: MetaErrorFmt> MetaErrorFmt for MetaErrorFormatter<'a, T> {
    #[inline]
    fn format(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.0.format(fmt)
    }

}

pub trait MetaErrorFmt {
    fn format(&self, fmt: &mut fmt::Formatter) -> fmt::Result;

    #[inline]
    fn as_fmt(&self) -> MetaErrorFormatter<Self> where Self: Sized {
        MetaErrorFormatter(self)
    }
}

impl<'a> MetaErrorFmt for &'a str {
    fn format(&self, fmt: &mut  fmt::Formatter) -> fmt::Result {
        fmt.write_str(&self)
    }
}


impl<T> MetaErrorFmt for &T where T: MetaVisitor {
    #[inline]
    fn format(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.expecting(fmt)
    }
}