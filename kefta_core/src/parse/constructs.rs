use crate::error::{KeftaErrorKind, KeftaResult};
use crate::node::AttrNode;
use crate::parse::{AttrModel};

pub struct AttrNamed {
    pub nodes: Vec<AttrNode>
}

impl AttrModel for AttrNamed {
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
                AttrNode::Container { contents, ident, container_type, .. } => {
                    match contents.parse(ident, container_type) {
                        Ok(node) => build.push(node),
                        Err(Ok(mut nodes)) => build.append(&mut nodes),
                        Err(Err(e)) => return Err(e.into())
                    }
                }
                _ => return Err(KeftaErrorKind::ExpectedNamed.into())
            }
        }

        Ok(AttrNamed { nodes: build })
    }
}

/*pub struct AttrDefault<T>(pub T) where T: Default + AttrValue;

impl<T> AttrValue for AttrDefault<T> where T: Default + AttrValue {
    fn parse(node: Option<AttrNode>) -> KeftaResult<Self> {
        match node {
            None => Ok(Self(T::default())),
            x @ Some(_) => T::parse(x).map(Self),
        }
    }
}*/