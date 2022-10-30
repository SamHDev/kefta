use proc_macro::{Span, TokenTree};
use kefta_core::error::KeftaResult;
use kefta_core::node::AttrNode;
use kefta_core::parse::{AttrMap, AttrModel};

#[derive(Debug)]
pub struct ModelAttr {
    pub namespace: Option<String>
}

impl AttrModel for ModelAttr {
    fn parse(nodes: Vec<AttrNode>) -> KeftaResult<Self> {
        let mut _root = AttrMap::new(nodes);
        let mut _ns = AttrMap::new_named(_root.get(Some("kefta")))?;

        Ok(Self {
            namespace: <Option<String> as AttrModel>::parse(_ns.get(Some("namespace")))?
        })
    }
}

#[derive(Debug)]
pub struct ItemAttr {
    /// rename the item
    pub rename: Option<(String, Span)>,
    /// aliases for the item
    pub alias: Vec<(String, Span)>,

    /// switch the namespace
    pub namespace: Option<String>,
    /// with the root
    pub root_namespace: bool,

    /// as root value
    pub value: (bool, Option<Span>),
    /// the rest of the values
    pub rest: (bool, Option<Span>),

    /// default
    pub default: bool,
    /// default expr
    pub default_value: Option<TokenTree>
}

impl AttrModel for ItemAttr {
    fn parse(nodes: Vec<AttrNode>) -> KeftaResult<Self> {
        let mut _root = AttrMap::new(nodes);
        let mut _ns = AttrMap::new_named(_root.get(Some("kefta")))?;

        Ok(Self {
            rename: <Option<(String, Span)> as AttrModel>::parse(_ns.gather(
                &[None, Some("rename"), Some("name")]
            ))?,
            alias: <Vec<(String, Span)> as AttrModel>::parse(_ns.get(Some("alias")))?,

            namespace: <Option<String> as AttrModel>::parse(_ns.get(Some("namespace")))?,
            root_namespace:  <bool as AttrModel>::parse(_ns.get(Some("root_namespace")))?,

            value: <(bool, Option<Span>) as AttrModel>::parse(_ns.get(Some("value")))?,
            rest: <(bool, Option<Span>) as AttrModel>::parse(_ns.get(Some("rest")))?,

            default: <bool as AttrModel>::parse(_ns.get(Some("default")))?,
            default_value: <Option<TokenTree> as AttrModel>::parse(_ns.get(Some("default_value")))?
        })
    }
}