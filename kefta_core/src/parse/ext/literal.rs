use proc_macro::TokenTree;
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
                expected: Some("a literal value".into())
            })
        }
    }
}

impl AttrValue for String {
    fn parse(node: Option<AttrNode>) -> KeftaResult<Self> {
        match <litrs::OwnedLiteral as AttrValue>::parse(node)? {
            litrs::OwnedLiteral::String(value) => Ok(value.into_value().to_string()),
            _ =>  Err(KeftaError::ExpectedType {
                expected: Some("a valid string literal".into())
            })
        }
    }
}

impl AttrValue for char {
    fn parse(node: Option<AttrNode>) -> KeftaResult<Self> {
        match <litrs::OwnedLiteral as AttrValue>::parse(node)? {
            litrs::OwnedLiteral::Char(value) => Ok(value.value()),
            _ =>  Err(KeftaError::ExpectedType {
                expected: Some("a valid char literal".into())
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
                                expected: Some(format!("a valid {} integer", stringify!($typ)))
                            }),
                            Some(x) => Ok(x)
                        },
                        _ =>  Err(KeftaError::ExpectedType {
                            expected: Some("a valid integer".into())
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
                        expected: Some("a valid f32 number".into())
                    })
                }
            _ =>  Err(KeftaError::ExpectedType {
                expected: Some("a valid string literal".into())
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
                        expected: Some("a valid f64 number".into())
                    })
                }
            _ =>  Err(KeftaError::ExpectedType {
                expected: Some("a valid string literal".into())
            })
        }
    }
}