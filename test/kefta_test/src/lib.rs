#![allow(dead_code)]

use proc_macro::TokenStream;
use syn::parse_macro_input;
use syn::DeriveInput;
use kefta::{Attr, parse_attr};

#[proc_macro_derive(TestMacro, attributes(test))]
pub fn test_macro(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);

    let attrs = parse_attr!(input.attrs => MyAttr);
    println!("ATTR {:?}", attrs);

    TokenStream::new()
}

#[derive(Attr, Debug)]
struct MyAttr {
    #[attr(required)]
    foo: u8,
    bar: bool,
    #[attr(container)]
    baz: MyAttrBaz
}

#[derive(Attr, Debug)]
struct MyAttrBaz {
    #[attr(optional)]
    alpha: Option<String>,
    beta: bool
}