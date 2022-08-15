use proc_macro::TokenStream;
use syn::parse_macro_input;
use syn::DeriveInput;
use kefta::{Attr, AttrParse};

#[proc_macro_derive(TestMacro, attributes(test))]
pub fn test_macro(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);

    let attrs: MyAttr = match input.attrs.parse_attrs() {
        Ok(attr) => attr,
        Err(e) => {
            let e: syn::Error = e.into();
            return e.to_compile_error().into();
        },
    };

    println!("ATTR {:?}", attrs);

    TokenStream::new()
}

#[derive(Attr, Debug)]
struct MyAttr {
    #[attr(required)]
    foo: String,
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