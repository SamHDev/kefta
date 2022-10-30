use proc_macro::TokenStream;
use syn::DeriveInput;
use kefta::{AttrModel, parse_attr_tokens};

#[proc_macro_derive(ExampleMacro, attributes(attr))]
pub fn derive_test(item: TokenStream) -> TokenStream {
    // parse with `syn`
    let input = syn::parse_macro_input!(item as DeriveInput);
    parse_attr_tokens!(input.attrs => ExampleModel);

    TokenStream::new()
}


#[derive(AttrModel)]
#[kefta(namespace="eg")]
struct ExampleModel {
    #[kefta(value)]
    pub value: String,

    pub description: Option<String>,

    #[kefta(default)]
    pub number: u64
}
