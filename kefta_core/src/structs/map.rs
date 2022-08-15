use std::collections::BTreeMap;
use crate::error::{KeftaError, KeftaResult};
use crate::node::{AttrNode, AttrTree};
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
        names[0]
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

    pub fn gather_nodes(&mut self, keys: &[&str])  -> KeftaResult<Vec<AttrNode>> {
        let mut build = Vec::new();

        for key in keys {
            if let Some(nodes) = self.get_nodes(key) {
                for node in nodes {
                    build.push(node);
                }
            }
        }

        Ok(build)
    }


    /* parse functions */

    pub fn parse_one<T: AttrValue + Default>(&mut self, keys: &[&str])  -> KeftaResult<T> {
        for key in keys {
            if let Some(node) = self.get_node(key, false)? {
                return <T as AttrValue>::parse(node);
            }
        }
        Ok(<T as Default>::default())
    }

    pub fn parse_optional<T: AttrValue>(&mut self, keys: &[&str])  -> KeftaResult<Option<T>> {
        for key in keys {
            if let Some(node) = self.get_node(key, false)? {
                return <T as AttrValue>::parse(node).map(|x| Some(x));
            }
        }
        Ok(None)
    }

    pub fn parse_required<T: AttrValue>(&mut self, keys: &[&str])  -> KeftaResult<T> {
        for key in keys {
            if let Some(node) = self.get_node(key, false)? {
                return <T as AttrValue>::parse(node);
            }
        }
        Err(KeftaError::Required {
            key: keys[0].to_string(),
            multiple: false
        })
    }

    pub fn parse_array<T: AttrValue>(&mut self, keys: &[&str])  -> KeftaResult<Vec<T>> {
        let mut build = Vec::new();

        for key in keys {
            if let Some(nodes) = self.get_nodes(key) {
                for node in nodes {
                    build.push(T::parse(node)?);
                }
            }
        }

        Ok(build)
    }

    pub fn parse_array_optional<T: AttrValue>(&mut self, keys: &[&str])  -> KeftaResult<Option<Vec<T>>> {
        let array = self.parse_array(&keys)?;
        if array.is_empty() { Ok(None) } else { Ok(Some(array)) }
    }

    pub fn parse_array_required<T: AttrValue>(&mut self, keys: &[&str])  -> KeftaResult<Vec<T>> {
        let array = self.parse_array(&keys)?;
        if array.is_empty() {
            Err(KeftaError::Required {
                key: keys[0].to_string(),
                multiple: true
            })
        } else {
            Ok(array)
        }
    }

    pub fn parse_container<T: AttrStruct>(&mut self, keys: &[&str])  -> KeftaResult<T> {
        let mut build = Vec::new();

        for node in self.gather_nodes(&keys)? {
            match node.data {
                AttrTree::Container { nodes, .. } => build.extend(nodes),
                _ => return Err(KeftaError::ExpectedContainer { ident: node.ident })
            }
        }

        T::parse(build)
    }

    pub fn parse_with<T>(&mut self, keys: &[&str], func: fn(nodes: Vec<AttrNode>) -> KeftaResult<T>) -> KeftaResult<T> {
        (func)(self.gather_nodes(&keys)?)
    }
}