## `Kefta`
simplified attribute parsing w/ proc-macros

[![crates.io badge](https://img.shields.io/crates/v/kefta.svg?style=for-the-badge)](https://crates.io/crates/kefta)
[![docs.rs badge](https://img.shields.io/docsrs/kefta.svg?style=for-the-badge&color=blue)](https://docs.rs/kefta)
[![Downloads badge](https://img.shields.io/crates/d/kefta.svg?style=for-the-badge)](https://crates.io/crates/kefta)

### Key Features
- derive macros for easily parsing structures
- built-in "decent" error messages
- build-in attribute parsing
- optional `syn` support

### Feature Toggles
- `literal` - literal parsing
- `util` - pre-made types and utility traits
- `syn` - syn support via `Syn<impl syn::Parse>`

### Examples
```rust
use kefta::{Attr, parse_attr};

// derive `Attr` onto your struct
#[derive(Attr)]
struct MyAttrs {
    // a marker field,
    alive: bool,

    // an optional field
    #[attr(optional)]
    name: Option<String>,

    // a required field
    #[attr(required)]
    value: i32,

    // a default field (defaults to 0)
    count: u32,

    // a renamed field
    #[attr(optional, name="desc")]
    description: Option<String>,

    // a field with an alias
    #[attr(alias="color")]
    colour: Option<String>,

    // a field with multiple values
    #[attr(multiple)]
    jobs: Vec<String>,

    // an optional, aliased field, with multiple values
    #[attr(multiple, optional, alias="chores")]
    tasks: Option<Vec<String>>,
    /* you get the point */
}

// parse in your derive-macro
// * this uses the syn crate and `syn` feature
#[proc_macro_derive(Human, attributes(human))]
pub fn test_macro(item: TokenStream) -> TokenStream {
    // parse with `syn`
    let input = syn::parse_macro_input!(item as DeriveInput);

    // parse the attributes with the `parse_attr!` macro
    // it contains a `return TokenStream`, so you don't have to handle errors.
    let attrs = parse_attr!(input.attrs => MyAttrs);

    // print out one of our fields
    println!("Name:  {:?}", attrs.name);

    TokenStream::new()
}

```

You can use attributes like so
```rust
#[derive(Human)]
#[human(name="Jimmy", value=10, alive)]
#[human(jobs="foo", jobs="bar", jobs="baz")]
pub struct Jimmy;
```