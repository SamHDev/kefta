use std::fmt::Formatter;
use litrs::{Literal};
use proc_macro2::{Span, TokenTree};
use crate::model::{FromMeta, MetaError, MetaSource, MetaVisitor};

impl FromMeta<TokenTree> for Literal<String> {
    fn from_meta<S>(source: S) -> Result<Self, S::Error> where S: MetaSource<TokenTree> {
        struct _Visitor;

        impl MetaVisitor for _Visitor {
            type Output = Literal<String>;
            type Domain = TokenTree;

            fn expecting(&self, f: &mut Formatter) -> std::fmt::Result {
                f.write_str("valid literal")
            }

            fn visit_value<E>(self, span: Option<Span>, value: Self::Domain) -> Result<Self::Output, E> where E: MetaError {
                Self::Output::try_from(value)
                    .map_err(|err| E::invalid_value(span, self, err))
            }
        }

        source.visit(_Visitor)
    }
}

fn _lit_type(lit: &Literal<String>) -> &'static str {
    match lit {
        Literal::Bool(_) => "bool",
        Literal::Integer(_) => "integer",
        Literal::Float(_) => "float",
        Literal::Char(_) => "char",
        Literal::String(_) => "string",
        Literal::Byte(_) => "byte",
        Literal::ByteString(_) => "byte-string"
    }
}

macro_rules! literal_impl {
    (
        $(
            for $typ: ty: $exp: literal -> $pat: ident ($ident: ident) => $expr: expr
        ),*
    ) => {
        $(impl FromMeta<TokenTree> for $typ {
            fn from_meta<S>(source: S) -> Result<Self, S::Error> where S: MetaSource<TokenTree> {
                struct _Visitor;

                impl MetaVisitor for _Visitor {
                    type Output = $typ;
                    type Domain = TokenTree;

                    fn expecting(&self, f: &mut Formatter) -> std::fmt::Result {
                        f.write_str($exp)
                    }

                    fn visit_value<E>(self, span: Option<Span>, value: Self::Domain) -> Result<Self::Output, E> where E: MetaError {
                        match Literal::<String>::try_from(value) {
                            Err(err) => Err(E::invalid_value(span, self, err)),
                            Ok(Literal::$pat($ident)) => Ok($expr),
                            Ok(x) => Err(E::invalid_value(span, self, _lit_type(&x)))
                        }
                    }
                }

                source.visit(_Visitor)
            }
        })*
    };
}

macro_rules! literal_impl_option {
    (
        $(
            for $typ: ty: $exp: literal $err: literal -> $pat: ident ($ident: ident) => $expr: expr
        ),*
    ) => {
        $(impl FromMeta<TokenTree> for $typ {
            fn from_meta<S>(source: S) -> Result<Self, S::Error> where S: MetaSource<TokenTree> {
                struct _Visitor;

                impl MetaVisitor for _Visitor {
                    type Output = $typ;
                    type Domain = TokenTree;

                    fn expecting(&self, f: &mut Formatter) -> std::fmt::Result {
                        f.write_str($exp)
                    }

                    fn visit_value<E>(self, span: Option<Span>, value: Self::Domain) -> Result<Self::Output, E> where E: MetaError {
                        match Literal::<String>::try_from(value) {
                            Err(err) => Err(E::invalid_value(span, self, err)),
                            Ok(Literal::$pat($ident)) => match $expr {
                                Some(x) => Ok(x),
                                None => Err(E::invalid_value(span, self, $err))
                            },
                            Ok(x) => Err(E::invalid_value(span, self, _lit_type(&x)))
                        }
                    }
                }

                source.visit(_Visitor)
            }
        })*
    };
}

literal_impl! {
    for String: "a string literal" -> String(x) => x.value().to_string(),
    for char: "a char literal" -> Char(x) => x.value()
}

literal_impl_option! {
    for u8: "a u8 literal" "invalid u8 value" -> Integer(x) => x.value::<u8>(),
    for u16: "a u16 literal" "invalid u16 value" -> Integer(x) => x.value::<u16>(),
    for u32: "a u32 literal" "invalid u32 value" -> Integer(x) => x.value::<u32>(),
    for u64: "a u64 literal" "invalid u64 value" -> Integer(x) => x.value::<u64>()
}
