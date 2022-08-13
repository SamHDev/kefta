use std::collections::BTreeMap;
use crate::AttrNode;

pub struct MappedAttrVec {
    map: BTreeMap<String, AttrNode>
}

impl MappedAttrVec {
    pub fn new(nodes: Vec<AttrNode>) -> Self {
        let mut map = BTreeMap::new();
        for node in nodes {
            map.insert(node.ident.to_string(), node);
        }
        Self { map }
    }

    pub fn get<K: AsRef<str>>(&self, key: K) -> Option<&AttrNode> {
        self.map.get(key.as_ref())
    }
}