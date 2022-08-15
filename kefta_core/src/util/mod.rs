//! utility types and traits.

#[cfg(feature = "util")]
pub mod case;

#[cfg(feature = "syn")]
mod syn;
#[cfg(feature = "syn")]
pub use self::syn::Syn;

//pub mod named;




