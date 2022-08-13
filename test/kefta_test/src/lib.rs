use proc_macro::TokenStream;
use kefta::{AttrData, AttrNode, AttrParse, AttrValue, KeftaError, KeftaResult, MappedAttrVec};

#[proc_macro_derive(TestMacro)]
pub fn test_macro(_item: TokenStream) -> TokenStream {
    TokenStream::new()
}

pub struct MyAttr {
    foo: String,
    bar: bool
}

impl AttrParse for MyAttr {
    fn parse(nodes: Vec<AttrNode>) -> KeftaResult<Self> {
        let mapped = MappedAttrVec::new(nodes);

        Ok(Self {
            foo: match mapped.get("foo").cloned() {
                Some(AttrData::Marker { .. }) => AttrValue::parse(&node.ident, None),
                Some(AttrData::Valued { value, .. }) => AttrValue::parse(&node.ident, Some(value)),
                Some(AttrData::Container { nodes }) => A
            }
        })
    }
}