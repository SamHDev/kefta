use crate::model::{MetaDomain};
use proc_macro2::{ TokenStream};

#[allow(clippy::needless_lifetimes)]
impl MetaDomain for TokenStream {
    type ErrorDisplay<'a> = &'a TokenStream where Self: 'a;

    fn as_error_display<'a>(&'a self) -> Self::ErrorDisplay<'a> {
        self
    }
}