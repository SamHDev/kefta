use proc_macro2::{Group, Ident, Punct, TokenStream, TokenTree};

#[derive(Debug)]
pub struct AttrIdent {
    pub prefix: Option<[Punct; 2]>,
    pub path: Vec<(Ident, [Punct; 2])>,
    pub ident: Ident,
}

#[derive(Debug)]
pub struct AttrNode {
    pub ident: AttrIdent,
    pub data: AttrData
}

#[derive(Debug)]
pub enum AttrData {
    None,
    Valued {
        equals: Punct,
        value: TokenTree,
    },
    List {
        group: Group,
        contents: TokenStream
    }
}