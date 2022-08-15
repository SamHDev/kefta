use proc_macro2::Ident;
use crate::error::{KeftaError, KeftaResult};
use crate::node::{AttrNode, AttrTree};
use crate::structs::AttrStruct;

pub struct Named<T: AttrStruct>(Option<Ident>, T);

impl<T: AttrStruct> AttrStruct for Named<T> {
    fn parse(nodes: Vec<AttrNode>) -> KeftaResult<Self> {
        let mut ident = None;
        let mut inner = Vec::new();

        for node in nodes {
            if ident.is_none() {
                ident = Some(node.ident.clone());
            } else if ident.as_ref() != Some(&node.ident) {
                return Err(KeftaError::Message {
                    message: "mismatched attribute names".to_string(),
                    span: Some(node.ident.span())
                });
            }

            match node.data {
                AttrTree::Container { nodes, .. } => {
                    inner.extend(nodes);
                },

                _ => return Err(KeftaError::Message {
                    message: "expected container".to_string(),
                    span: Some(node.ident.span())
                })
            }
        }

        Ok(Named(ident, T::parse(inner)?))
    }
}

impl<T: AttrStruct> Named<T> {
    pub fn value(self) -> T {
        self.1
    }
}

/*impl<T: AttrStruct> Into<T> for Named<T> {
    fn into(self) -> T {
        self.1
    }
}*/