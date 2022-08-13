use proc_macro2::{Delimiter, Ident, Punct, TokenTree};

pub struct AttrNode {
    pub ident: Ident,
    pub data: AttrTree
}

pub enum AttrTree {
    Marker,
    Valued {
        equal: Punct,
        value: TokenTree,
    },
    Container {
        group: Delimiter,
        nodes: Vec<AttrNode>,
        tailfish: bool,
    }
}