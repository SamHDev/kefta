use proc_macro::{Ident, TokenStream};
use crate::error::{KeftaError};
use crate::node::{AttrContents, AttrNode, ContainerType, parse_body, ParseTokenStream};

pub trait AttrSource {
    fn parse(self) -> Result<Vec<AttrNode>, KeftaError>;
}

impl AttrSource for Vec<AttrNode> {
    fn parse(self) -> Result<Vec<AttrNode>, KeftaError> {
        Ok(self)
    }
}

impl AttrSource for TokenStream {
    fn parse(self) -> Result<Vec<AttrNode>, KeftaError> {
        let mut stream = ParseTokenStream::wrap(self);
        parse_body(&mut stream).map_err(|x| x.into())
    }
}

impl AttrSource for (Ident, TokenStream) {
    fn parse(self) -> Result<Vec<AttrNode>, KeftaError> {
        if self.1.is_empty() {
            return Ok(vec![AttrNode::Marker { ident: self.0 }])
        } else {
            return Ok(vec![AttrNode::Container {
                group: self.0.span(),
                ident: self.0,
                container_type: ContainerType::Implicit,
                contents: AttrContents::Stream(self.1)
            }])
        }
    }
}

impl AttrSource for Vec<(Ident, TokenStream)> {
    fn parse(self) -> Result<Vec<AttrNode>, KeftaError> {
        let mut build = Vec::new();
        for pair in self {
            build.append(&mut pair.parse()?);
        }
        Ok(build)
    }
}

#[cfg(feature = "syn")]
fn _ident(ident: &syn::Ident) -> Ident {
    Ident::new(&ident.to_string(), ident.span().unwrap())
}

#[cfg(feature = "syn")]
impl AttrSource for syn::Attribute {
    fn parse(self) -> Result<Vec<AttrNode>, KeftaError> {
        (
            _ident(&self.path.segments.last().unwrap().ident),
            Into::<TokenStream>::into(self.tokens)
        ).parse()
    }
}

#[cfg(feature = "syn")]
impl AttrSource for Vec<syn::Attribute> {
    fn parse(self) -> Result<Vec<AttrNode>, KeftaError> {
        self.into_iter()
            .map(|x| (
                _ident(&x.path.segments.last().unwrap().ident),
                Into::<TokenStream>::into(x.tokens)
            ))
            .collect::<Vec<(Ident, TokenStream)>>()
            .parse()
    }
}