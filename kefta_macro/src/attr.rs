use kefta_core::error::{KeftaResult};
use kefta_core::node::AttrNode;
use kefta_core::parse::AttrValue;
use kefta_core::structs::{AttrMap, AttrStruct};

pub struct StructAttr {
    pub name: Option<String>,
    pub alias: Vec<String>,

    pub required: bool,
    pub optional: bool,
}

impl AttrStruct for StructAttr {
    fn parse(nodes: Vec<AttrNode>) -> KeftaResult<Self> {
        let mut map = AttrMap::new(nodes);

        Ok(Self {
            name: map.node_optional("name", false)?,
            alias: map.node_multiple(&*["alias", "names"])?,
            required: map.node_default("required", false)?,
            optional: map.node_default("required", false)?
        })
    }
}