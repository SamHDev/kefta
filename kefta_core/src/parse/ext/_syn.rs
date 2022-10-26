use proc_macro::{TokenStream, TokenTree};
use std::ops::Deref;
use syn::parse::{Parse, ParseBuffer, ParseStream};
use crate::error::KeftaResult;
use crate::node::AttrNode;
use crate::parse::AttrModel;

pub struct Syn<T>(pub T) where T: Parse;

impl<T> Deref for Syn<T> where T: Parse {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Into<T> for Syn<T> where T: Parse {
    fn into(self) -> T {
        self
    }
}

/*
impl<T> AttrModel for Syn<T> where T:Parse {
    fn parse(nodes: Vec<AttrNode>) -> KeftaResult<Self> {
        match <TokenTree as AttrModel>::parse(nodes) {
            Ok(x) => T::parse(ParseBuffer::fr)
            Err(x) => {}
        }
    }
}
*/