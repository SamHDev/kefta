use crate::error::{KeftaResult};
use crate::node::AttrNode;

pub trait AttrModel: Sized {
    fn parse(nodes: Vec<AttrNode>) -> KeftaResult<Self>;
}

// vec of nodes
impl AttrModel for Vec<AttrNode> {
    fn parse(nodes: Vec<AttrNode>) -> KeftaResult<Self> {
        Ok(nodes)
    }
}

// option model
impl<T> AttrModel for Option<T> where T: AttrModel {
    fn parse(nodes: Vec<AttrNode>) -> KeftaResult<Self> {
       if nodes.is_empty() {
           Ok(None)
       } else {
           T::parse(nodes).map(Some)
       }
    }
}
