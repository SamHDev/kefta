use proc_macro::TokenStream;
use syn::DeriveInput;
use kefta::{AttrModel, parse_attr_tokens};

#[proc_macro_derive(ExampleMacro, attributes(eg))]
pub fn derive_test(item: TokenStream) -> TokenStream {
    // parse with `syn`
    let input = syn::parse_macro_input!(item as DeriveInput);
    println!("{:?}", input.attrs);

    // parse attr tokens
    let tokens = parse_attr_tokens!(input.attrs => ExampleModel);
    println!("{:?}", tokens);

    // return no new tokens
    TokenStream::new()
}


#[derive(AttrModel, Debug)]
#[kefta(namespace="eg")]
struct ExampleModel {
    #[kefta(value)]
    pub value: String,

    pub description: Option<String>,

    #[kefta(default)]
    pub number: u64,

    #[kefta(namespace="doc", name="doc")]
    pub comments: Vec<String>,
}


