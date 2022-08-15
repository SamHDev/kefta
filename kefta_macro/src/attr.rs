use kefta_core::error::{KeftaResult};
use kefta_core::node::AttrNode;
use kefta_core::structs::{AttrMap, AttrStruct};

#[derive(Debug)]
pub struct StructAttr {
    pub name: Option<String>,
    pub alias: Vec<String>,

    pub required: bool,
    pub optional: bool,

    pub multiple: bool,
    pub container: bool,

    pub with: Option<String>,
}

impl AttrStruct for StructAttr {
    fn parse(nodes: Vec<AttrNode>) -> KeftaResult<Self> {
        let mut map = AttrMap::new(nodes);

        Ok(Self {
            name: map.parse_optional(&["name"])?,
            alias: map.parse_array(&["alias", "names"])?,
            required: map.parse_one(&["required", "req"])?,
            optional: map.parse_one(&["optional", "opt"])?,
            multiple: map.parse_one(&["multiple", "many"])?,
            container: map.parse_one(&["container", "map"])?,
            with: map.parse_one(&["with", "parse", "call"])?
        })
    }
}

pub struct EnumAttr {
    pub name: Option<String>,
    pub alias: Vec<String>,
}

impl AttrStruct for EnumAttr {
    fn parse(nodes: Vec<AttrNode>) -> KeftaResult<Self> {
        let mut map = AttrMap::new(nodes);

        Ok(Self {
            name: map.parse_optional(&["name"])?,
            alias: map.parse_array(&["alias", "names"])?,
        })
    }
}
