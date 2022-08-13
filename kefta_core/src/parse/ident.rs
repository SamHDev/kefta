use proc_macro2::Ident;
use crate::error::{KeftaError, KeftaResult};
use crate::node::AttrNode;
use crate::parse::AttrValue;

pub trait AttrIdent: Sized {
    const CHOICES: &'static [&'static str];

    fn parse(name: &str, ident: Ident) -> Option<Self>;
}

impl<T> AttrValue for T where T: AttrIdent {
    fn parse(node: AttrNode) -> KeftaResult<Self> {
        let ident = Ident::parse(node)?;

        match T::parse(&ident.to_string(), ident) {
            Some(value) => Ok(value),

            None => Err(KeftaError::Message {
                message: format!(
                    "expected one the following idents [{}]",
                    T::CHOICES.iter()
                        .map(|x| format!("`{:?}`", x))
                        .collect::<Vec<String>>()
                        .join(", ")
                ),
                span: None
            })
        }
    }
}