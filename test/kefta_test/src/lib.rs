use proc_macro::TokenStream;

#[proc_macro_derive(TestMacro)]
pub fn test_macro(_item: TokenStream) -> TokenStream {
    println!("{:?}", _item);
    TokenStream::new()
}

struct MyAttr {
    foo: String,
    bar: bool
}
