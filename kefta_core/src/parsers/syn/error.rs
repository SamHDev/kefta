use std::fmt::Display;
use proc_macro2::{Span, TokenStream};
use crate::model::MetaError;

impl MetaError for syn::Error {
    fn into_token_stream(self) -> TokenStream {
        self.into_compile_error()
    }

    fn custom(span: Option<Span>, message: impl Display) -> Self {
        syn::Error::new(span.unwrap_or_else(Span::call_site), message)
    }
}