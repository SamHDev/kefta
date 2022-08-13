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
