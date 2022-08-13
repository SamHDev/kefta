use crate::error::KeftaResult;
use crate::node::AttrNode;

pub trait AttrValue: Sized {
    fn parse(node: AttrNode) -> KeftaResult<Self>;
}