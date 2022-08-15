use proc_macro2::{Delimiter, Ident, Punct, TokenTree};

/// an attribute node
///
/// contains the `ident` and `data` of an attribute node
pub struct AttrNode {
    pub ident: Ident,
    pub data: AttrTree
}

/// data of an attribute node
pub enum AttrTree {
    /// a non-valued attribute
    ///
    /// e.g. `#[attr(foo)]`
    Marker,
    /// a valued attribute
    ///
    /// has a given value as a `TokenTree`
    ///
    /// e.g. `#[attr(foo=10)]` or `#[attr(foo=(10 + 10))]`
    Valued {
        /// equal token
        equal: Punct,
        /// value token(s)
        value: TokenTree,
    },
    /// a container attribute
    ///
    /// has multiple descendants
    ///
    /// e.g. `#[attr(foo(bar))]` or `#[attr(foo::bar)]`
    Container {
        /// group delimiter
        group: Delimiter,
        /// descendant nodes
        nodes: Vec<AttrNode>,
        /// is a tailfish container (e.g. foo::bar)
        tailfish: bool,
    }
}