mod nodes;
mod attr;
mod expr;

use proc_macro::TokenStream;
use syn::parse_macro_input;
use syn::DeriveInput;
use crate::nodes::{parse_node, parse_nodes};

#[proc_macro_derive(Attr, attributes(attr))]
pub fn test_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let nodes = parse_nodes(input.attrs).unwrap();



    TokenStream::new()
}