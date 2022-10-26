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
                AttrNode::Container { contents, .. } => {
                    match contents.parse() {
                        Ok(node) => build.push(node),
                        Err(Ok(mut nodes)) => build.append(&mut nodes),
                        Err(Err(e)) => return Err(KeftaError::ParseError(e))
                    }
                }
                _ => return Err(KeftaError::ExpectedNamed)
            }
        }

        Ok(Named { nodes: build })
    }
}