use proc_macro::{Ident, Punct, Span, TokenStream};
use proc_macro::TokenTree;
use std::fmt::{Debug, Formatter};
use crate::node::{parse_body, ParseError, ParseTokenStream};

pub enum AttrNode {
    /// No parse attached
    ///
    /// `#[attr(bar)]`
    Marker {
        /// the identifier of the marker
        ident: Ident,
    },

    /// a single literal parse
    ///
    /// `#[attr("Hello World")]`
    Literal {
        /// the token tree of the parse
        value: TokenTree
    },

    /// a parse pair
    ///
    /// `#[attr(key="parse")]`
    Value {
        /// the ident of the pair
        ident: Ident,
        /// equality punct
        equal: Punct,
        /// the token tree parse
        value: TokenTree
    },

    /// a container mod
    ///
    /// `#[attr(foo(bar))]`
    /// `#[attr(foo::bar)]`
    Container {
        /// the identifier of the container
        ident: Ident,
        /// the group span
        group: Span,
        /// if the container is tailfish (`::`)
        is_tailfish: bool,
        /// the contents of the container
        contents: AttrContents,
    }
}

#[derive(Debug)]
pub enum AttrContents {
    Stream(TokenStream),
    Node(Box<AttrNode>)
}


impl AttrNode {
    pub fn ident(&self) -> Option<&Ident> {
        match self {
            AttrNode::Marker { ident } => Some(ident),
            AttrNode::Literal { .. } => None,
            AttrNode::Value { ident, .. } => Some(ident),
            AttrNode::Container { ident, .. } => Some(ident),
        }
    }

    pub fn token_tree(&self) -> Option<&TokenTree> {
        match self {
            AttrNode::Marker { .. } => None,
            AttrNode::Literal { value } => Some(value),
            AttrNode::Value { value, .. } => Some(value),
            AttrNode::Container { .. } => None,
        }
    }
}

impl AttrContents {
    pub fn parse(self) -> Result<AttrNode, Result<Vec<AttrNode>, ParseError>> {
        match self {
            AttrContents::Stream(stream) => Err({
                let mut stream = ParseTokenStream::wrap(stream);
                parse_body(&mut stream)
            }),
            AttrContents::Node(node) => Ok(*node),
        }
    }
}

impl Debug for AttrNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            AttrNode::Marker { ident } =>
                f.debug_tuple("Marker")
                    .field(&ident.to_string())
                    .finish(),
            AttrNode::Literal { value } =>
                f.debug_tuple("Literal")
                    .field(&value.to_string())
                    .finish(),
            AttrNode::Value { ident, value, .. } =>
                f.debug_tuple("Value")
                    .field(&ident.to_string())
                    .field(&value.to_string())
                    .finish(),
            AttrNode::Container { ident, contents, is_tailfish, .. } =>
                f.debug_tuple("Container")
                    .field(&ident.to_string())
                    .field(&is_tailfish)
                    .field(&contents)
                    .finish(),
        }
    }
}