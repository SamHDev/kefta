mod attr;
mod attr_struct;

use proc_macro::TokenStream;
use syn::{Data, parse_macro_input};
use syn::DeriveInput;
use syn::spanned::Spanned;

#[proc_macro_derive(Attr, attributes(attr))]
pub fn attr_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let out = match input.data {
        Data::Struct(ref data) => attr_struct::attr_struct(input),
        Data::Enum(_) => Err(syn::Error::new(input.span(), "unions are not supported")),
        Data::Union(_) => Err(syn::Error::new(input.span(), "unions are not supported")),
    };

    match out {
        Ok(x) => x,
        Err(e) => e.into_compile_error()
    }.into()
}

