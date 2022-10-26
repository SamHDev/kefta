use proc_macro::TokenStream;
use kefta_core::error::KeftaResult;
use kefta_core::node::{AttrNode, parse_body, ParseTokenStream};
use kefta_core::parse::{AttrMap, AttrModel};

#[proc_macro]
pub fn testing(tokens: TokenStream) -> TokenStream {

    let mut stream = ParseTokenStream::wrap(tokens.into());
    let body = parse_body(&mut stream).unwrap();

    let model = Model::parse(body).unwrap();
    println!("{:?}", model);

    TokenStream::new()
}

#[derive(Debug)]
struct Model {
    pub name: String,
    pub is_test: bool,
}

impl AttrModel for Model {
    fn parse(nodes: Vec<AttrNode>) -> KeftaResult<Self> {
        println!("{:?}", nodes);

        let mut _root = AttrMap::new(nodes);
        println!("{:?}", _root);
        let mut _ns = AttrMap::new_named(_root.get(Some("bar")))?;
        println!("{:?}", _ns);

        Ok(Self {
            name: <String as AttrModel>::parse(_ns.get(Some("name")))?,
            is_test: <bool as AttrModel>::parse(_ns.get(Some("is_test")))?
        })
    }
}