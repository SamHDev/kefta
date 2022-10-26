use std::collections::BTreeMap;
use crate::error::{KeftaResult};
use crate::node::AttrNode;
use crate::parse::AttrModel;
use crate::parse::container::Named;

type AttrMapInner = BTreeMap<String, Vec<AttrNode>>;

#[derive(Debug)]
pub struct AttrMap {
    map: AttrMapInner
}

impl AttrMap {
    pub fn new(nodes: Vec<AttrNode>) -> Self {
        let mut map: AttrMapInner = BTreeMap::new();

        for node in nodes {
            let key = node.ident().map(|x| x.to_string()).unwrap_or_default();

            if let Some(arr) = map.get_mut(&key) {
                arr.push(node);
            } else {
                map.insert(key, vec![ node ]);
            }
        }

        Self { map }
    }

    pub fn new_named(nodes: Vec<AttrNode>) -> KeftaResult<Self> {
        let named = Named::parse(nodes)?;
        Ok(Self::new(named.nodes))
    }
}

impl AttrMap {
    pub fn get(&mut self, key: Option<&str>) -> Vec<AttrNode> {
        self.map.remove(key.unwrap_or_default()).unwrap_or_default()
    }

    pub fn gather(&mut self, keys: &[Option<&str>]) -> Vec<AttrNode> {
        let mut build = Vec::new();
        for key in keys {
            build.append(&mut self.get(*key));
        }
        build
    }
}