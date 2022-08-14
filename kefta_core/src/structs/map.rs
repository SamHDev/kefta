use std::collections::BTreeMap;
use crate::error::{KeftaError, KeftaResult};
use crate::node::AttrNode;
use crate::parse::AttrValue;
use crate::structs::AttrStruct;

const _EMPTY: Vec<AttrNode> = Vec::new();
const _EMPTY_REF: &Vec<AttrNode> = &_EMPTY;

pub struct AttrMap {
    map: BTreeMap<String, Vec<AttrNode>>
}

impl AttrMap {
    pub fn new(nodes: Vec<AttrNode>) -> Self {
        let mut map: BTreeMap<String, Vec<AttrNode>> = BTreeMap::new();

        for node in nodes {
            let name = node.ident.to_string();

            if let Some(array) = map.get_mut(&name) {
                array.push(node);
            } else {
                map.insert(name, vec![node]);
            }
        }

        Self { map }
    }

    pub fn alias<'a>(&self, names: &[&'a str]) -> Option<&'a str> {
        for name in names {
            if self.map.contains_key(*name) {
                return Some(name)
            }
        }
        None
    }

    pub fn alias_otherwise<'a>(&self, names: &[&'a str]) -> &'a str {
        for name in names {
            if self.map.contains_key(*name) {
                return name
            }
        }
        name[0]
    }

    pub fn peek_nodes(&self, key: &str) -> &Vec<AttrNode> {
        match self.map.get(key) {
            None => &_EMPTY_REF,
            Some(nodes) => &nodes
        }
    }

    pub fn get_nodes(&mut self, key: &str) -> Option<Vec<AttrNode>> {
        match self.map.remove(key) {
            None => None,
            Some(nodes) => Some(nodes)
        }
    }

    pub fn peek_node(&self, key: &str) -> Option<&AttrNode> {
        match self.map.get(key) {
            None => None,
            Some(nodes) => if nodes.len() == 1 {
                nodes.first()
            } else {
                None
            }
        }
    }

    pub fn get_node(&mut self, key: &str, error: bool) -> KeftaResult<Option<AttrNode>> {
        match self.map.remove(key) {
            None => Ok(None),
            Some(mut nodes) => if nodes.len() == 1 {
                Ok(Some(nodes.remove(0)))
            } else {
                if error {
                    Err(KeftaError::Multiple {
                        key: key.to_string(),
                        count: nodes.len()
                    })
                } else {
                    Ok(None)
                }
            }
        }
    }


    pub fn node_default<T: AttrValue + Default>(&mut self, key: &str, lax: bool) -> KeftaResult<T> {
        match self.get_node(key, !lax)? {
            None => Ok(Default::default()),
            Some(node) => AttrValue::parse(node),
        }
    }

    pub fn node_optional<T: AttrValue>(&mut self, key: &str, lax: bool) -> KeftaResult<Option<T>> {
        match self.get_node(key, !lax)? {
            None => Ok(None),
            Some(node) => Ok(Some(AttrValue::parse(node)?))
        }
    }

    pub fn node_required<T: AttrValue>(&mut self, key: &str, lax: bool) -> KeftaResult<T> {
        KeftaError::require(key, false, self.node_optional(key, lax)?)
    }

    pub fn node_multiple<T: AttrValue>(&mut self, keys: &[&str]) -> KeftaResult<Vec<T>> {
        let mut build = Vec::new();

        for key in keys {
            build.extend(self.get_nodes(key)?);
        }

        Ok(build)
    }

    pub fn node_multiple_optional<T: AttrValue>(&mut self, keys: &[&str]) -> KeftaResult<Option<Vec<T>>> {
        let nodes = self.node_multiple(keys)?;
        Ok(if nodes.is_empty() { None } else { Some(nodes) })
    }

    pub fn node_multiple_required<T: AttrValue>(&mut self, keys: &[&str]) -> KeftaResult<Vec<T>> {
        KeftaError::require(&keys[0], true, self.node_multiple_optional(keys)?)
    }

    pub fn node_default_alias<T: AttrValue + Default>(&mut self, keys: &[&str], lax: bool) -> KeftaResult<T> {
        self.node_default(&self.alias_otherwise(keys), lax)
    }

    pub fn node_optional_alias<T: AttrValue>(&mut self, keys: &[&str], lax: bool) -> KeftaResult<Option<T>> {
        self.node_optional(&self.alias_otherwise(keys), lax)
    }

    pub fn node_required_alias<T: AttrValue>(&mut self, keys: &[&str], lax: bool) -> KeftaResult<T> {
        KeftaError::require(
            key,
            false,
            self.node_optional(self.alias_otherwise(keys), lax)?
        )
    }
}