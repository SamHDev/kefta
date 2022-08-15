use crate::error::KeftaResult;
use crate::node::AttrNode;

/// parse a value from a single attribute node
pub trait AttrValue: Sized {
    fn parse(node: AttrNode) -> KeftaResult<Self>;
}