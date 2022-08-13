use crate::error::KeftaResult;
use crate::node::AttrNode;

pub trait AttrEnum: Sized {
    fn parse(node: AttrNode) -> KeftaResult<Self>;
}

pub trait AttrStruct: Sized {
    fn parse(nodes: Vec<AttrNode>) -> KeftaResult<Self>;
}

impl<E> AttrStruct for Vec<E> where E: AttrEnum {
    fn parse(nodes: Vec<AttrNode>) -> KeftaResult<Self> {
        let mut build = Vec::with_capacity(nodes.len());

        for node in nodes {
            build.push(<E as AttrEnum>::parse(node)?);
        }

        Ok(build)
    }
}