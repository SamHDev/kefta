use crate::error::{KeftaError, KeftaResult};
use crate::node::AttrNode;
use crate::parse::AttrModel;

pub struct Named {
    pub nodes: Vec<AttrNode>
}

impl AttrModel for Named {
    fn parse(nodes: Vec<AttrNode>) -> KeftaResult<Self> {
        let mut build = Vec::new();

        for node in nodes {
            match node {
                AttrNode::Marker { .. } => {
                    build.push(node);
                },
                AttrNode::Value { value, .. } => {
                    build.push(AttrNode::Literal { value });
                },
                AttrNode::Container { mut contents, .. } => {
                    build.append(&mut contents);
                }
                _ => return Err(KeftaError::ExpectedNamed)
            }
        }

        Ok(Named { nodes: build })
    }
}