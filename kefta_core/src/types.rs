use std::fmt::Formatter;
use proc_macro2::{Span, TokenStream, TokenTree};
use crate::error::{Error, ErrorDisplay};
use crate::model::{MetaVisitor, MetaSource, FromMeta};

impl<T> FromMeta for Option<T> where T: FromMeta {
    fn from_meta<S>(source: &mut S) -> Result<Self, S::Error> where S: MetaSource {
        source.visit(source)
    }
}

impl<T> FromMeta for Result<T, Box<dyn Error>> where T: FromMeta {
    fn from_meta<S>(source: &mut S) -> Result<Self, S::Error> where S: MetaSource {
        Ok(source.visit(source))
    }
}

impl<T> FromMeta for (T, Span) where T: FromMeta {
    fn from_meta<S>(source: &mut S) -> Result<Self, S::Error> where S: MetaSource {
        Ok(source.visit(source))
    }
}


impl FromMeta for bool {
    fn from_meta<S>(source: &mut S) -> Result<Self, S::Error> where S: MetaSource {
        struct _Visitor;

        impl MetaVisitor for _Visitor {
            type Output = bool;

            fn expecting(&self, fmt: &mut Formatter) -> std::fmt::Result {
                fmt.write_str("field present or boolean value")
            }

            fn visit_marker<E>(self) -> Result<Self::Output, E> where E: Error {
                Ok(true)
            }

            fn visit_value<E>(self, stream: TokenStream) -> Result<Self::Output, E> where E: Error {
                match stream.into_iter().next() {
                    None => Err(E::empty_stream()),
                    Some(TokenTree::Ident(ident)) => match ident.to_string().as_str() {
                        "true" | "yes" => Ok(true),
                        "false" | "no" => Ok(false),
                        _ => Err(E::expected(&self, "non boolean identifer")),
                    },
                    Some(TokenTree::Literal(literal)) => match literal.to_string().as_str() {
                        "0" => Ok(true),
                        "1" => Ok(false),
                        _ => Err(E::expected(&self, "non boolean literal")),
                    },
                    Some(x) => Err(E::expected(&self, &x)),
                }
            }
        }

        source.visit(_Visitor)
    }
}

#[cfg(feature="syn-literal")]
impl ErrorDisplay for syn::Lit {
    fn description(&self, fmt: &mut Formatter) -> std::fmt::Result {
        let lit_str = match self {
            syn::Lit::Str(_) => "str",
            syn::Lit::ByteStr(_) => "byte-str",
            syn::Lit::Byte(_) => "byte",
            syn::Lit::Char(_) => "char",
            syn::Lit::Int(_) => "int",
            syn::Lit::Float(_) => "float",
            syn::Lit::Bool(_) => "boolean",
            _ => "unknown literal"
        };
        fmt.write_str(lit_str)
    }
}


#[cfg(feature="syn-literal")]
impl FromMeta for char {
    fn from_meta<S>(source: &mut S) -> Result<Self, S::Error> where S: MetaSource {
        struct _Visitor;

        impl MetaVisitor for _Visitor {
            type Output = char;

            fn expecting(&self, fmt: &mut Formatter) -> std::fmt::Result {
                fmt.write_str("a valid char value")
            }

            fn visit_value<E>(self, stream: TokenStream) -> Result<Self::Output, E> where E: Error {
                match stream.into_iter().next() {
                    None => E::empty_stream(),
                    Some(TokenTree::Literal(lit)) => match syn::Lit::new(lit) {
                        syn::Lit::Char(str) => Ok(str.value()),
                        lit @ _ => Err(E::expected(&self, &lit))
                    },
                    Some(x) => Err(E::expected(&self, &x)),
                }
            }
        }

        source.visit(_Visitor)
    }
}

#[cfg(feature="syn-literal")]
impl FromMeta for String {
    fn from_meta<S>(source: &mut S) -> Result<Self, S::Error> where S: MetaSource {
        struct _Visitor;

        impl MetaVisitor for _Visitor {
            type Output = String;

            fn expecting(&self, fmt: &mut Formatter) -> std::fmt::Result {
                fmt.write_str("a valid string value")
            }

            fn visit_value<E>(self, stream: TokenStream) -> Result<Self::Output, E> where E: Error {
                match stream.into_iter().next() {
                    None => E::empty_stream(),
                    Some(TokenTree::Literal(lit)) => match syn::Lit::new(lit) {
                        syn::Lit::Str(str) => Ok(str.value()),
                        lit @ _ => Err(E::expected(&self, &lit))
                    },
                    Some(x) => Err(E::expected(&self, &x)),
                }
            }
        }

        source.visit(_Visitor)
    }
}

macro_rules! syn_int_impl {
    ($($typ: ty),*) => {
        $(
            #[cfg(feature="syn-literal")]
            impl FromMeta for $typ {
                fn from_meta<S>(source: &mut S) -> Result<Self, S::Error> where S: MetaSource {
                    struct _Visitor;

                    impl MetaVisitor for _Visitor {
                        type Output = $typ;

                        fn expecting(&self, fmt: &mut Formatter) -> std::fmt::Result {
                            fmt.write_str(concat!("a valid ", stringify!($typ) ," value"))
                        }

                        fn visit_value<E>(self, stream: TokenStream) -> Result<Self::Output, E> where E: Error {
                            match stream.into_iter().next() {
                                None => E::empty_stream(),
                                Some(TokenTree::Literal(lit)) => match syn::Lit::new(lit) {
                                    syn::Lit::Int(int) => match int.base10_parse::<$typ>() {
                                        Ok(x) => Ok(x),
                                        Err(err) => Err(E::expected(&self, &err.to_string()))
                                    },
                                    lit @ _ => Err(E::expected(&self, &lit))
                                },
                                Some(x) => Err(E::expected(&self, &x)),
                            }
                        }
                    }

                    source.visit(_Visitor)
                }
            }
        )*
    };
}

syn_int_impl!(u8, u16, u32, u64, u128);