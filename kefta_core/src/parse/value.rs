use proc_macro::TokenTree;
use crate::error::KeftaError;

pub trait ParseValue {
    fn parse_value(token: TokenTree) -> Result<Self, KeftaError>;
}

impl ParseValue for TokenTree {
    fn parse_value(token: TokenTree) -> Result<Self, KeftaError> {
        Ok(token)
    }
}
