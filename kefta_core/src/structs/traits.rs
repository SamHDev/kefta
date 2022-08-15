use crate::error::KeftaResult;
use crate::node::AttrNode;
use crate::parse::AttrValue;

/// a defined structure of attributes
pub trait AttrStruct: Sized {
    fn parse(nodes: Vec<AttrNode>) -> KeftaResult<Self>;
}

impl<T: AttrValue> AttrStruct for Vec<T> {
    fn parse(nodes: Vec<AttrNode>) -> KeftaResult<Self> {
        let mut build = Vec::with_capacity(nodes.len());

        for node in nodes {
            build.push(<T as AttrValue>::parse(node)?);
        }

        Ok(build)
    }
}