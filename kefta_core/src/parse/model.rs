use std::collections::btree_map::BTreeMap;
use crate::error::{KeftaResult};
use crate::node::AttrNode;
use crate::parse::AttrValue;

pub trait AttrModel: Sized {
    fn parse(nodes: Vec<AttrNode>) -> KeftaResult<Self>;
}

// vec of nodes
/*
impl AttrModel for Vec<AttrNode> {
    fn parse(nodes: Vec<AttrNode>) -> KeftaResult<Self> {
        Ok(nodes)
    }
}*/

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

// array model
impl<T> AttrModel for Vec<T> where T: AttrValue {
    fn parse(nodes: Vec<AttrNode>) -> KeftaResult<Self> {
        let mut build = Vec::new();
        for node in nodes {
            build.push(T::parse(Some(node))?);
        }
        Ok(build)
    }
}

// map model
impl<T> AttrModel for BTreeMap<Option<String>, T> where T: AttrModel {
    fn parse(nodes: Vec<AttrNode>) -> KeftaResult<Self> {
        // gather map
        let mut map = BTreeMap::<Option<String>, Vec<AttrNode>>::new();

        for node in nodes {
            let key = node.ident().map(|x| x.to_string());
            if let Some(arr) = map.get_mut(&key) {
                arr.push(node)
            } else {
                map.insert(key, vec![node]);
            }
        }

        // parse
        let mut map2 = BTreeMap::new();

        for (key, nodes) in map {
            map2.insert(key,T::parse(nodes)?);
        }

        Ok(map2)
    }
}
