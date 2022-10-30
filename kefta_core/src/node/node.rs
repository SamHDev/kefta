use proc_macro::{Ident, Punct, Span, TokenStream};
use proc_macro::TokenTree;
use std::fmt::{Debug, Formatter};
use crate::node::{parse_body, parse_content, ParseError, ParseTokenStream};

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
        container_type: ContainerType,
        /// the contents of the container
        contents: AttrContents,
    }
}

pub enum AttrContents {
    Stream(TokenStream),
    Node(Box<AttrNode>)
}

impl Debug for AttrContents {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            AttrContents::Stream(x) =>
                f.debug_tuple("Stream")
                    .field(&x.to_string())
                    .finish(),
            AttrContents::Node(x) =>
                f.debug_tuple("Node")
                    .field(&x)
                    .finish(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ContainerType {
    /// a group container `ident(...)`
    Grouped,
    /// a tailfish container `ident::foo`
    Tailfish,
    /// a implicit/root container
    ///
    /// when parsing recurse until valid container
    Implicit
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
    pub fn parse(self, ident: Ident, container_type: ContainerType) -> Result<AttrNode, Result<Vec<AttrNode>, ParseError>> {
        match self {
            AttrContents::Stream(stream) => {
                let mut stream = ParseTokenStream::wrap(stream);

                if container_type == ContainerType::Implicit {
                    match parse_content(ident, &mut stream) {
                        // recurse container
                        Ok(AttrNode::Container { contents, container_type, ident, .. }) =>
                            contents.parse(ident, container_type),

                        // return
                        Ok(x) => Ok(x),
                        Err(e) => Err(Err(e))
                    }
                } else {
                    Err(parse_body(&mut stream))
                }

            },
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
            AttrNode::Container { ident, contents, container_type, .. } =>
                f.debug_tuple("Container")
                    .field(&ident.to_string())
                    .field(&container_type)
                    .field(&contents)
                    .finish(),
        }
    }
}