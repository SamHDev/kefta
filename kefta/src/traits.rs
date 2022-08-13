use proc_macro2::{Ident, TokenTree};
use crate::{AttrNode, KeftaError, KeftaResult};

pub trait AttrValue: Sized {
    fn parse(ident: &Ident, data: Option<TokenTree>) -> KeftaResult<Self>;
}

pub trait AttrEnum: Sized {
    fn parse(node: AttrNode) -> KeftaResult<Self>;
}

pub trait AttrParse: Sized {
    fn parse(nodes: Vec<AttrNode>) -> KeftaResult<Self>;
}

impl<E: AttrEnum> AttrParse for Vec<E> {
    fn parse(nodes: Vec<AttrNode>) -> KeftaResult<Self> {
        let mut build = Vec::with_capacity(nodes.len());
        for node in nodes { build.push(E::parse(node)?); }
        Ok(build)
    }
}
