use proc_macro::TokenStream;
use quote::quote;
use kefta_core::node::{parse_body, ParseTokenStream};

#[proc_macro]
pub fn testing(tokens: TokenStream) -> TokenStream {

    let mut stream = ParseTokenStream::wrap(tokens.into());
    let body = parse_body(&mut stream);

    println!("{:?}", body);


    TokenStream::new()
}