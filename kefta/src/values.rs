use litrs::{Literal, OwnedLiteral};
use proc_macro2::{Ident, TokenTree};
use syn::spanned::Spanned;
use crate::{AttrNode, AttrValue, KeftaError, KeftaResult};

impl AttrValue for () {
    fn parse(ident: &Ident, data: Option<TokenTree>) -> KeftaResult<Self> {
        match data {
            Some(_) => Err(KeftaError::ExpectedMarker(ident.clone())),
            None => Ok(()),
        }
    }
}

impl AttrValue for TokenTree {
    fn parse(ident: &Ident, data: Option<TokenTree>) -> KeftaResult<Self> {
        match data {
            None => Err(KeftaError::ExpectedValue(ident.clone())),
            Some(value) => Ok(value)
        }
    }
}

impl AttrValue for proc_macro2::Literal {
    fn parse(ident: &Ident, data: Option<TokenTree>) -> KeftaResult<Self> {
        match TokenTree::parse(ident, data)? {
            TokenTree::Literal(literal) => Ok(literal),
            x @ _ => Err(KeftaError::Message("expected literal", x.span()))
        }
    }
}

impl AttrValue for OwnedLiteral {
    fn parse(ident: &Ident, data: Option<TokenTree>) -> KeftaResult<Self> {
        Ok(Literal::from(proc_macro2::Literal::parse(ident, data)?))
    }
}

impl AttrValue for Vec<u8> {
    fn parse(ident: &Ident, data: Option<TokenTree>) -> KeftaResult<Self> {
        match <OwnedLiteral as AttrValue>::parse(ident, data)? {
            Literal::ByteString(s) => Ok(s.value().to_vec()),

            // todo: fix literal
            _ => Err(KeftaError::Message("expected string literal", ident.span()))
        }
    }
}

impl AttrValue for String {
    fn parse(ident: &Ident, data: Option<TokenTree>) -> KeftaResult<Self> {
        match <OwnedLiteral as AttrValue>::parse(ident, data)? {
            Literal::String(s) => Ok(s.value().to_string()),

            // todo: fix literal
            _ => Err(KeftaError::Message("expected string literal", ident.span()))
        }
    }
}

impl AttrValue for char {
    fn parse(ident: &Ident, data: Option<TokenTree>) -> KeftaResult<Self> {
        match <OwnedLiteral as AttrValue>::parse(ident, data)? {
            Literal::Char(s) => Ok(s.value()),
            // todo: fix literal
            _ => Err(KeftaError::Message("expected char literal", ident.span()))
        }
    }
}

macro_rules! int_value {
    ( $($type: ty),* ) => {
        $(
            impl AttrValue for $type {
                fn parse(ident: &Ident, data: Option<TokenTree>) -> KeftaResult<Self> {
                    match <OwnedLiteral as AttrValue>::parse(ident, data)? {
                        Literal::Integer(int) => match int.value::<$type>() {
                            Some(x) => Ok(x),
                            None => Err(KeftaError::Message("expected valid numerical value", ident.span()))
                        },
                        // todo: fix literal
                        _ => Err(KeftaError::Message("expected numerical literal", ident.span()))
                    }
                }
            }
        )*
    };
}

int_value!(usize, u8, u16, u32, u64);

impl AttrValue for bool {
    fn parse(ident: &Ident, data: Option<TokenTree>) -> KeftaResult<Self> {
        if data.is_none() {
            return Ok(true);
        } else {
            match <OwnedLiteral as AttrValue>::parse(ident, data)? {
                Literal::Bool(s) => Ok(s.value()),
                // todo: fix literal
                _ => Err(KeftaError::Message("expected boolean literal", ident.span()))
            }
        }

    }
}