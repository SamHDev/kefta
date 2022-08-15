use std::collections::BTreeMap;
use crate::error::{KeftaError, KeftaResult};
use crate::node::{AttrNode, AttrTree};
use crate::parse::AttrValue;
use crate::structs::AttrStruct;

const _EMPTY: Vec<AttrNode> = Vec::new();
const _EMPTY_REF: &Vec<AttrNode> = &_EMPTY;

/// map for parsing an array of attribute nodes
pub struct AttrMap {
    map: BTreeMap<String, Vec<AttrNode>>
}

impl AttrMap {
    /// Create a new `AttrMap` from an array of attribute nodes
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

    /// compare a list of keys/names and return the first occurring.
    /// if no match is found, return `None`
    pub fn alias<'a>(&self, names: &[&'a str]) -> Option<&'a str> {
        for name in names {
            if self.map.contains_key(*name) {
                return Some(name)
            }
        }
        None
    }

    /// compare a list of keys/names and return the first occurring.
    /// if no match is found return the first key.
    pub fn alias_otherwise<'a>(&self, names: &[&'a str]) -> &'a str {
        for name in names {
            if self.map.contains_key(*name) {
                return name
            }
        }
        names[0]
    }

    /// peek (without removing) the nodes with a given key
    pub fn peek_nodes(&self, key: &str) -> &Vec<AttrNode> {
        match self.map.get(key) {
            None => &_EMPTY_REF,
            Some(nodes) => &nodes
        }
    }

    /// get (removing from map) the nodes with a given key
    pub fn get_nodes(&mut self, key: &str) -> Option<Vec<AttrNode>> {
        match self.map.remove(key) {
            None => None,
            Some(nodes) => Some(nodes)
        }
    }

    /// peek (without removing) the first matching node with the given key
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

    /// get (removing from map) the first matching node with the given key
    ///
    /// the parameter `error` controls duplicate behaviour
    /// - when set to `true` - if multiple with the same key are found,
    ///  a `KeftaError::Multiple` error will be returned
    /// - when set to `false` - multiple nodes will be ignored.
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

    /// gather nodes (removing from map) with an array of keys
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

    /// parse a single node from an array of keys,
    /// returning `Default::default()` if not present.
    pub fn parse_one<T: AttrValue + Default>(&mut self, keys: &[&str])  -> KeftaResult<T> {
        for key in keys {
            if let Some(node) = self.get_node(key, false)? {
                return <T as AttrValue>::parse(node);
            }
        }
        Ok(<T as Default>::default())
    }

    /// parse an optional single node from an array of keys
    pub fn parse_optional<T: AttrValue>(&mut self, keys: &[&str])  -> KeftaResult<Option<T>> {
        for key in keys {
            if let Some(node) = self.get_node(key, false)? {
                return <T as AttrValue>::parse(node).map(|x| Some(x));
            }
        }
        Ok(None)
    }

    /// parse an single node from an array of keys,
    /// returning an `KeftaError::Required` error if not present.
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

    /// parse an array of nodes, from an array of keys.
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

    /// parse an array of nodes, from an array of keys.
    /// returns `None` if no matching nodes are found
    pub fn parse_array_optional<T: AttrValue>(&mut self, keys: &[&str])  -> KeftaResult<Option<Vec<T>>> {
        let array = self.parse_array(&keys)?;
        if array.is_empty() { Ok(None) } else { Ok(Some(array)) }
    }

    /// parse an array of nodes, from an array of keys.
    /// returns an `KeftaError::Required` error if no matching nodes are found
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

    /// parse a `AttrStruct` from an array of keys.
    ///
    /// this inspects the nodes and gathers the contents of `AttrTree::Container`.
    /// returns `KeftaError::ExpectedContainer` if a non-container is found
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

    /// parse an array of nodes with a given function
    pub fn parse_with<T>(&mut self, keys: &[&str], func: fn(nodes: Vec<AttrNode>) -> KeftaResult<T>) -> KeftaResult<T> {
        (func)(self.gather_nodes(&keys)?)
    }
}