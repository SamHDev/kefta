use proc_macro2::{Span, TokenTree};
use crate::error::{KeftaError, KeftaExpected, KeftaResult};
use crate::node::{AttrNode, AttrTree};
use crate::parse::AttrValue;

// ---- primitive parsing ----
impl AttrValue for () {
    fn parse(node: AttrNode) -> KeftaResult<Self> {
        match node.data {
            AttrTree::Marker => Ok(()),
            _ => Err(KeftaError::ExpectedMarker { ident: node.ident })
        }
    }
}

impl<T: AttrValue> AttrValue for Option<T> {
    fn parse(node: AttrNode) -> KeftaResult<Self> {
        T::parse(node).map(|x| Some(x))
    }
}

// ---- token tree ----

impl AttrValue for TokenTree {
    fn parse(node: AttrNode) -> KeftaResult<Self> {
        match node.data {
            AttrTree::Valued { value, .. } => Ok(value),
            _ => Err(KeftaError::ExpectedValue { ident: node.ident }),
        }
    }
}

impl AttrValue for (Span, TokenTree) {
    fn parse(node: AttrNode) -> KeftaResult<Self> {
        match node.data {
            AttrTree::Valued { value, .. } => Ok((value.span(), value)),
            _ => Err(KeftaError::ExpectedValue { ident: node.ident }),
        }
    }
}

// ---- tree parsing ----
macro_rules! attr_tree {
    ($($ident: ident),*) => {
        $(
            impl AttrValue for proc_macro2::$ident {
                fn parse(node: AttrNode) -> KeftaResult<Self> {
                    match TokenTree::parse(node)? {
                        TokenTree::$ident(x) => Ok(x),
                        token_tree @ _ => Err(KeftaError::Expected {
                            expected: KeftaExpected::$ident,
                            span: token_tree.span(),
                        })
                    }
                }
            }
        )*
    };
}
attr_tree!(Literal, Group, Punct, Ident);

// ---- literal parsing ----

#[cfg(feature="literal")]
impl AttrValue for litrs::OwnedLiteral {
    fn parse(node: AttrNode) -> KeftaResult<Self> {
        Ok(litrs::OwnedLiteral::from( proc_macro2::Literal::parse(node)? ))
    }
}

#[cfg(feature="literal")]
impl AttrValue for (Span, litrs::OwnedLiteral) {
    fn parse(node: AttrNode) -> KeftaResult<Self> {
        let literal = proc_macro2::Literal::parse(node)?;
        Ok((literal.span(), litrs::OwnedLiteral::from( literal )))
    }
}

#[cfg(feature="literal")]
impl AttrValue for char {
    fn parse(node: AttrNode) -> KeftaResult<Self> {
        match <(Span, litrs::OwnedLiteral) as AttrValue>::parse(node)? {
            (_, litrs::Literal::Char(string)) => Ok(string.value()),
            (span, _) => Err(KeftaError::Expected {
                expected: KeftaExpected::CharLiteral,
                span
            })
        }
    }
}

#[cfg(feature="literal")]
impl AttrValue for String {
    fn parse(node: AttrNode) -> KeftaResult<Self> {
        match <(Span, litrs::OwnedLiteral) as AttrValue>::parse(node)? {
            (_, litrs::Literal::String(string)) => Ok(string.value().to_string()),
            (span, _) => Err(KeftaError::Expected {
                expected: KeftaExpected::StringLiteral,
                span
            })
        }
    }
}

#[cfg(feature="literal")]
impl AttrValue for Vec<u8> {
    fn parse(node: AttrNode) -> KeftaResult<Self> {
        match <(Span, litrs::OwnedLiteral) as AttrValue>::parse(node)? {
            (_, litrs::Literal::ByteString(string)) => Ok(string.into_value().to_vec()),
            (span, _) => Err(KeftaError::Expected {
                expected: KeftaExpected::ByteLiteral,
                span
            })
        }
    }
}

#[cfg(feature="literal")]
macro_rules! attr_num {
    ( $( $type:ty ),* ) => {
        $(
            impl AttrValue for $type {
                fn parse(node: AttrNode) -> KeftaResult<Self> {
                    match <(Span, litrs::OwnedLiteral) as AttrValue>::parse(node)? {
                        (span, litrs::Literal::Integer(integer)) => match integer.value::<$type>() {
                            Some(num) => Ok(num),
                            None => Err(KeftaError::Message {
                                message: format!(
                                    "integer `{}` overflows type `{}`",
                                    integer.to_string(),
                                    std::any::type_name::<$type>()
                                ),
                                span: Some(span)
                            })
                        },
                        (span, _) => Err(KeftaError::Expected {
                            expected: KeftaExpected::NumericLiteral,
                            span
                        })
                    }
                }
            }
        )*
    };
}
#[cfg(feature="literal")]
attr_num!(usize, u8, i8, u16, i16, u32, i32, u64, i64);

impl AttrValue for bool {
    fn parse(node: AttrNode) -> KeftaResult<Self> {
        match node.data {
            AttrTree::Marker => Ok(true),

            #[cfg(feature="literal")]
            AttrTree::Valued { .. } =>
                match  <(Span, litrs::OwnedLiteral) as AttrValue>::parse(node)? {
                    (_, litrs::Literal::Bool(boolean)) => Ok(boolean.value()),

                    (span, _) => Err(KeftaError::Expected {
                        expected: KeftaExpected::BooleanLiteral,
                        span
                    })
                },

            _ => Err(KeftaError::ExpectedMarker { ident: node.ident })
        }
    }
}
