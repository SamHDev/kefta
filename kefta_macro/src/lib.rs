use proc_macro::TokenStream;
use syn::{Data, DeriveInput, Error};
use syn::spanned::Spanned;
use kefta_core::node::AttrSource;
use kefta_core::parse::AttrModel;
use crate::attrs::ModelAttr;
use crate::proc::process;

mod attrs;
mod proc;

#[proc_macro_derive(AttrModel, attributes(kefta))]
pub fn attr_model(item: TokenStream) -> TokenStream {
    // parse with `syn`
    let input = syn::parse_macro_input!(item as DeriveInput);
    let span = input.span();

    let ident = input.ident;
    let generics = input.generics;

    let nodes = AttrSource::parse(input.attrs).unwrap();
    let attrs = ModelAttr::parse(nodes).unwrap();

    return if let Data::Struct(data) = input.data {
        match process(attrs, data, ident, generics) {
            Ok(x) => {
                //println!("{}", x.to_string());
                x.into()
            },
            Err(e) => e.to_compile_error().into()
        }
    } else {
        Error::new(
            span,
            "`AttrModel` macro can only be used on structs"
        ).to_compile_error().into()
    }
}
