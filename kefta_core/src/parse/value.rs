use proc_macro::{Span, TokenStream, TokenTree};
use crate::error::{KeftaError, KeftaResult};
use crate::node::{AttrContents, AttrNode};
use crate::parse::model::AttrModel;


pub trait AttrValue: Sized {
    fn parse(node: Option<AttrNode>) -> KeftaResult<Self>;
}

// parse attr node
impl AttrValue for AttrNode {
    fn parse(node: Option<AttrNode>) -> KeftaResult<Self> {
        match node {
            None => Err(KeftaError::Expected(Default::default())),
            Some(node) => Ok(node)
        }
    }
}

// model conversion
impl<T> AttrModel for T where T: AttrValue {
    fn parse(nodes: Vec<AttrNode>) -> KeftaResult<Self> {
        match nodes.len() {
            // parse value as none
            0 => <T as AttrValue>::parse(None),

            // parse value
            // don't like this iter bs
            1 => <T as AttrValue>::parse(Some(nodes.into_iter().nth(0).unwrap())),

            // return if too many
            _ => Err(KeftaError::Multiple(Default::default()))
        }
    }
}

// token tree conversion
impl AttrValue for TokenTree {
    fn parse(node: Option<AttrNode>) -> KeftaResult<Self> {
        match <AttrNode as AttrValue>::parse(node)? {

            AttrNode::Literal { value } => Ok(value),
            AttrNode::Value { value, .. } => Ok(value),

            // parse single containers
            // e.g. [foo("Hello World")] == [foo="Hello World"]
            AttrNode::Container { contents, container_type, ident, .. } => {
                // get node or parse stream
                let node = match contents.parse(ident, container_type) {
                    Ok(x) => Some(x),
                    Err(Ok(x)) => if x.len() == 1 {
                        x.into_iter().nth(0)
                    } else {
                        return Err(KeftaError::Multiple(Default::default()))
                    },
                    Err(Err(e)) => return Err(KeftaError::ParseError(e)),
                };

                match node {
                    None => unreachable!(),
                    Some(AttrNode::Literal { value }) => Ok(value),
                    Some(AttrNode::Marker { ident }) => Ok(TokenTree::Ident(ident)),
                    _ => Err(KeftaError::ExpectedValue(Default::default()))
                }
            }

            _ => Err(KeftaError::ExpectedValue(Default::default())),
        }
    }
}

// token stream extract
impl AttrValue for TokenStream {
    fn parse(node: Option<AttrNode>) -> KeftaResult<Self> {
        match <AttrNode as AttrValue>::parse(node)? {
            AttrNode::Literal { value } | AttrNode::Value { value, ..} => {
                match value {
                    TokenTree::Group(group) => Ok(group.stream()),
                    _ => Ok(TokenStream::from(value))
                }
            }
            AttrNode::Container { contents: AttrContents::Stream(stream), .. } => Ok(stream),
            _ => Err(KeftaError::ExpectedValue(Default::default())),

        }
    }
}

// boolean flags
impl AttrValue for bool {
    fn parse(node: Option<AttrNode>) -> KeftaResult<Self> {
        match node {
            None => return Ok(false),

            // is marker - return
            Some(AttrNode::Marker { .. }) => return Ok(true),

            // parse values
            Some(AttrNode::Value { value, .. }) => match value {
                // true/false yes/no
                TokenTree::Ident(ident) => match ident.to_string().as_str() {
                    "true" | "yes" => return Ok(true),
                    "false" | "no" => return Ok(false),
                    _ => ()
                }

                // 1 or 0
                TokenTree::Literal(literal) => match literal.to_string().as_str() {
                    "0" => return Ok(false),
                    "1" => return Ok(true),
                    _ => ()
                }
                _ => ()
            },
            _ => ()
        }

        Err(KeftaError::ExpectedType {
            expected: Some("a marker or boolean value".into()),
            context: Default::default()
        })
    }
}

#[cfg(feature="spanned")]
impl AttrValue for (bool, Option<Span>) {
    fn parse(node: Option<AttrNode>) -> KeftaResult<Self> {
        match node {
            None => return Ok((false, None)),

            // is marker - return
            Some(AttrNode::Marker { ident }) => return Ok((true, Some(ident.span()))),

            // parse values
            Some(AttrNode::Value { value, .. }) => match value {
                // true/false yes/no
                TokenTree::Ident(ident) => match ident.to_string().as_str() {
                    "true" | "yes" => return Ok((true, Some(ident.span()))),
                    "false" | "no" => return Ok((false, Some(ident.span()))),
                    _ => ()
                }

                // 1 or 0
                TokenTree::Literal(literal) => match literal.to_string().as_str() {
                    "0" => return Ok((false, Some(literal.span()))),
                    "1" => return Ok((true, Some(literal.span()))),
                    _ => ()
                }
                _ => ()
            },
            _ => ()
        }

        Err(KeftaError::ExpectedType {
            expected: Some("a marker or boolean value".into()),
            context: Default::default()
        })
    }
}

macro_rules! _token_tree_impl {
    ($( $typ:ident => $err:literal );*) => {
        $( impl AttrValue for proc_macro::$typ {
            fn parse(node: Option<AttrNode>) -> KeftaResult<Self> {
                match <TokenTree as AttrValue>::parse(node)? {
                    TokenTree::$typ(x) => Ok(x),
                    _ => Err(KeftaError::ExpectedType {
                        expected: Some($err.into()),
                        context: Default::default()
                    })
                }
            }
        } )*
    };
}

_token_tree_impl! {
    Ident => "an identifier";
    Literal => "a literal value";
    Group => "a token group";
    Punct => "a punctuation character"
}