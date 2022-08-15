//! ## `Kefta`
//! a simple attribute parser
//!
//! ### Key Features
//! - derive macros for easily parsing structures
//! - built-in "decent" error messages
//! - build-in attribute parsing
//! - optional `syn` support
//!
//! ### Feature Toggles
//! - `literal` - literal parsing
//! - `util` - pre-made types and utility traits
//! - `syn` - syn support via `Syn<impl syn::Parse>`
//! 
//! ### Examples
//! ```
//! use kefta::{Attr, parse_attr};
//!
//! // derive `Attr` onto your struct
//! #[derive(Attr)]
//! struct MyAttrs {
//!     // a marker field,
//!     alive: bool,
//!
//!     // an optional field
//!     #[attr(optional)]
//!     name: Option<String>,
//!
//!     // a required field
//!     #[attr(required)]
//!     value: i32,
//!
//!     // a default field (defaults to 0)
//!     count: u32,
//!
//!     // a renamed field
//!     #[attr(optional, name="desc")]
//!     description: Option<String>,
//!
//!     // a field with an alias
//!     #[attr(alias="color")]
//!     colour: Option<String>,
//!
//!     // a field with multiple values
//!     #[attr(multiple)]
//!     jobs: Vec<String>,
//!
//!     // an optional, aliased field, with multiple values
//!     #[attr(multiple, optional, alias="chores")]
//!     tasks: Option<Vec<String>>,
//!     /* you get the point */
//! }
//!
//! // parse in your derive-macro
//! // * this uses the syn crate and `syn` feature
//! #[proc_macro_derive(Human, attributes(human))]
//! pub fn test_macro(item: TokenStream) -> TokenStream {
//!     // parse with `syn`
//!     let input = syn::parse_macro_input!(item as DeriveInput);
//!
//!     // parse the attributes with the `parse_attr!` macro
//!     // it contains a `return TokenStream`, so you don't have to handle errors.
//!     let attrs = parse_attr!(input.attrs => MyAttrs);
//!
//!     // print out one of our fields
//!     println!("Name:  {:?}", attrs.name);
//!
//!     TokenStream::new()
//! }
//!
//! ```
//!
//! You can use attributes like so
//! ```no_compile
//! #[derive(Human)]
//! #[human(name="Jimmy", value=10, alive)]
//! #[human(jobs="foo", jobs="bar", jobs="baz")]
//! pub struct Jimmy;
//! ```


mod parse;

pub use kefta_core::error;
pub use kefta_core::token;
pub use kefta_core::parse::{AttrValue};
pub use kefta_core::node::{AttrNode, AttrTree};
pub use kefta_core::structs::{AttrMap, AttrStruct, AttrParse};
pub use kefta_core::util;

/// attribute for creating attribute structures
///
/// - fields must implement (`AttrValue`)
/// - by default, fields are parsed as single nodes using `Default` if not present.
/// - a number of attributes can be used to change this behaviour
///
/// ```text
/// #[attr(required)]               a required marker or value (errors if not present)
/// #[attr(optional)]               explicit use of an optional value (`Option<T>`)
/// #[attr(multiple)]               parse multiple nodes (`Vec<T>`)
/// #[attr(container)]              parse an inner structure (`impl AttrStruct`)
///
/// #[attr(name="name")]            rename the field
/// #[attr(alias="b", alias="b")]   add an alias for the field
///
/// #[attr(with="path_to_func")]    parse the value with a function
/// ```
///
pub use kefta_macro::Attr;