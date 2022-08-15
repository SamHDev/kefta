use proc_macro::TokenStream;
use kefta::Attr;

#[proc_macro_derive(TestMacro)]
pub fn test_macro(_item: TokenStream) -> TokenStream {
    println!("{:?}", _item);
    TokenStream::new()
}

#[derive(Attr)]
struct MyAttr {
    #[attr(required)]
    foo: String,
    bar: bool
}
