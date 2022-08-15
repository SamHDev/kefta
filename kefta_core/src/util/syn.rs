use std::ops::Deref;
use proc_macro2::{TokenStream};
use crate::error::{KeftaError, KeftaResult};
use crate::node::{AttrNode, AttrTree};
use crate::parse::AttrValue;
use syn::parse::{Parse};
use syn::parse2;

pub struct Syn<T: Parse>(pub(crate) T);

impl<T: Parse> Deref for Syn<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Parse> AsRef<T> for Syn<T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}

/*impl<T: Parse> Into<T> for Syn<T> {
    fn into(self) -> T {
        self.0
    }
}*/

impl<T: Parse> AttrValue for Syn<T> {
    fn parse(node: AttrNode) -> KeftaResult<Self> {
        match node.data {
            AttrTree::Valued { value, .. } =>
                match parse2::<T>(TokenStream::from(value)) {
                    Ok(parse) => Ok(Syn(parse)),
                    Err(e) => Err(KeftaError::Syn(e))
                },
            _ => Err(KeftaError::ExpectedValue { ident: node.ident })
        }
    }
}

macro_rules! attr_syn {
    ( $($ident: ident),* ) => {

        $(
            impl AttrValue for syn::$ident {
                fn parse(node: AttrNode) -> KeftaResult<Self> {
                    Ok(Syn::<syn::$ident>::parse(node)?.0)
                }
            }
        )*

    };
}

attr_syn!(Expr, Lit, LitStr);