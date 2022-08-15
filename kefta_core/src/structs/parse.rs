use proc_macro2::TokenStream;
use crate::error::{KeftaError, KeftaResult};
use crate::node::AttrNode;
use crate::structs::AttrStruct;
use crate::token::{AttrTokenStream};

pub trait AttrParse {
    fn parse_attrs<T: AttrStruct>(self) -> KeftaResult<T>;
}

impl AttrParse for TokenStream {
    fn parse_attrs<T: AttrStruct>(self) -> KeftaResult<T> {
        let mut stream = AttrTokenStream::new(self);
        let nodes = match AttrNode::parse_root(&mut stream) {
            Ok(parse) => parse,
            Err(token) => return Err(KeftaError::TokenError(token))
        };
        T::parse(nodes)
    }
}

impl AttrParse for Vec<TokenStream> {
    fn parse_attrs<T: AttrStruct>(self) -> KeftaResult<T> {
        let mut nodes = Vec::new();

        for tokens in self {
            let mut stream = AttrTokenStream::new(tokens);
            match AttrNode::parse_root(&mut stream) {
                Ok(parse) => { nodes.extend(parse); },
                Err(token) => return Err(KeftaError::TokenError(token))
            };

        }

        T::parse(nodes)
    }
}

#[cfg(feature = "syn")]
impl AttrParse for syn::Attribute {
    fn parse_attrs<T: AttrStruct>(self) -> KeftaResult<T> {
        self.tokens.parse_attrs()
    }
}

#[cfg(feature = "syn")]
impl AttrParse for Vec<syn::Attribute> {
    fn parse_attrs<T: AttrStruct>(self) -> KeftaResult<T> {
        let mut tokens = Vec::new();

        for attr in self {
            tokens.push(attr.tokens);
        }

        tokens.parse_attrs()
    }
}
