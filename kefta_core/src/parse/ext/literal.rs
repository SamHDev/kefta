use proc_macro::{Span, TokenTree};
use std::str::FromStr;
use crate::error::{KeftaError, KeftaResult};
use crate::node::AttrNode;
use crate::parse::AttrValue;

impl AttrValue for litrs::OwnedLiteral {
    fn parse(node: Option<AttrNode>) -> KeftaResult<Self> {
        let tree = <TokenTree as AttrValue>::parse(node)?;

        match litrs::OwnedLiteral::parse(tree.to_string()) {
            Ok(literal) => Ok(literal),
            Err(_) => Err(KeftaError::ExpectedType {
                expected: Some("a literal value".into()),
                context: Default::default()
            })
        }
    }
}

#[cfg(feature="spanned")]
impl AttrValue for (litrs::OwnedLiteral, Span) {
    fn parse(node: Option<AttrNode>) -> KeftaResult<Self> {
        let tree = <TokenTree as AttrValue>::parse(node)?;

        match litrs::OwnedLiteral::parse(tree.to_string()) {
            Ok(literal) => Ok((literal, tree.span())),
            Err(_) => Err(KeftaError::ExpectedType {
                expected: Some("a literal value".into()),
                context: Default::default()
            })
        }
    }
}

impl AttrValue for String {
    fn parse(node: Option<AttrNode>) -> KeftaResult<Self> {
        match <litrs::OwnedLiteral as AttrValue>::parse(node)? {
            litrs::OwnedLiteral::String(value) => Ok(value.into_value().to_string()),
            _ =>  Err(KeftaError::ExpectedType {
                expected: Some("a valid string literal".into()),
                context: Default::default()
            })
        }
    }
}

#[cfg(feature="spanned")]
impl AttrValue for (String, Span) {
    fn parse(node: Option<AttrNode>) -> KeftaResult<Self> {
        match <(litrs::OwnedLiteral, Span) as AttrValue>::parse(node)? {
            (litrs::OwnedLiteral::String(value), span) =>
                Ok((value.into_value().to_string(), span)),
            _ =>  Err(KeftaError::ExpectedType {
                expected: Some("a valid string literal".into()),
                context: Default::default()
            })
        }
    }
}

impl AttrValue for char {
    fn parse(node: Option<AttrNode>) -> KeftaResult<Self> {
        match <litrs::OwnedLiteral as AttrValue>::parse(node)? {
            litrs::OwnedLiteral::Char(value) => Ok(value.value()),
            _ =>  Err(KeftaError::ExpectedType {
                expected: Some("a valid char literal".into()),
                context: Default::default()
            })
        }
    }
}

#[cfg(feature="spanned")]
impl AttrValue for (char, Span) {
    fn parse(node: Option<AttrNode>) -> KeftaResult<Self> {
        match <(litrs::OwnedLiteral, Span) as AttrValue>::parse(node)? {
            (litrs::OwnedLiteral::Char(value), span) =>
                Ok((value.value(), span)),
            _ =>  Err(KeftaError::ExpectedType {
                expected: Some("a valid char literal".into()),
                context: Default::default()
            })
        }
    }
}

macro_rules! _literal_int_impl {
    ($( $typ:ident ),*) => {
        $(
            impl AttrValue for $typ {
                fn parse(node: Option<AttrNode>) -> KeftaResult<Self> {
                    match <litrs::OwnedLiteral as AttrValue>::parse(node)? {
                        litrs::OwnedLiteral::Integer(value) => match value.value::<$typ>() {
                            None => Err(KeftaError::ExpectedType {
                                expected: Some(format!("a valid {} integer", stringify!($typ))),
                                context: Default::default()
                            }),
                            Some(x) => Ok(x)
                        },
                        _ =>  Err(KeftaError::ExpectedType {
                            expected: Some("a valid integer".into()),
                            context: Default::default()
                        })
                    }
                }
            }

            #[cfg(feature="spanned")]
            impl AttrValue for ($typ, Span) {
                fn parse(node: Option<AttrNode>) -> KeftaResult<Self> {
                    match <(litrs::OwnedLiteral, Span) as AttrValue>::parse(node)? {
                        (litrs::OwnedLiteral::Integer(value), span) => match value.value::<$typ>() {
                            None => Err(KeftaError::ExpectedType {
                                expected: Some(format!("a valid {} integer", stringify!($typ))),
                                context: Default::default()
                            }),
                            Some(x) => Ok((x, span))
                        },
                        _ =>  Err(KeftaError::ExpectedType {
                            expected: Some("a valid integer".into()),
                            context: Default::default()
                        })
                    }
                }
            }
        )*
    };
}

_literal_int_impl!(u8, i8, u16, i16, u32, i32, u64, i64);

impl AttrValue for f32 {
    fn parse(node: Option<AttrNode>) -> KeftaResult<Self> {
        match <litrs::OwnedLiteral as AttrValue>::parse(node)? {
            litrs::OwnedLiteral::Float(float) =>
                match f32::from_str(float.number_part()) {
                    Ok(x) => Ok(x),
                    Err(_) => Err(KeftaError::ExpectedType {
                        expected: Some("a valid f32 number".into()),
                        context: Default::default()
                    })
                }
            _ =>  Err(KeftaError::ExpectedType {
                expected: Some("a valid string literal".into()),
                context: Default::default()
            })
        }
    }
}

impl AttrValue for f64 {
    fn parse(node: Option<AttrNode>) -> KeftaResult<Self> {
        match <litrs::OwnedLiteral as AttrValue>::parse(node)? {
            litrs::OwnedLiteral::Float(float) =>
                match f64::from_str(float.number_part()) {
                    Ok(x) => Ok(x),
                    Err(_) => Err(KeftaError::ExpectedType {
                        expected: Some("a valid f64 number".into()),
                        context: Default::default()
                    })
                }
            _ =>  Err(KeftaError::ExpectedType {
                expected: Some("a valid string literal".into()),
                context: Default::default()
            })
        }
    }
}