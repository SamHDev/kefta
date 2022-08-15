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