use crate::node::ParseError;

#[derive(Debug)]
pub enum KeftaError {
    ParseError(ParseError),

    Expected,
    Multiple,

    ExpectedValue,
    ExpectedNamed,
    ExpectedType {
        expected: Option<String>,
    }
}

pub type KeftaResult<T> = Result<T, KeftaError>;

