use proc_macro::Span;

#[derive(Debug)]
pub struct ParseError {
    pub span: Option<Span>,
    pub kind: ParseErrorKind
}

impl From<ParseErrorKind> for ParseError {
    fn from(kind: ParseErrorKind) -> Self {
        Self { span: None, kind }
    }
}

impl From<(ParseErrorKind, Span)> for ParseError {
    fn from((kind, span): (ParseErrorKind, Span)) -> Self {
        Self { span: Some(span), kind }
    }
}

#[derive(Debug)]
pub enum ParseErrorKind {
    ExpectedDelimiter,
    ExpectedAttribute,

    ExpectedValue,

    ExpectedTailfishIdent,
    ExpectedTailfishPunct,

    InvalidContainerGroup,

    InvalidContent,
    UnexpectedEnd,
}

impl ParseErrorKind {
    pub fn message(&self) -> &'static str {
        match &self {
            ParseErrorKind::ExpectedDelimiter =>
                "expected a delimiter (`,`) after token",
            ParseErrorKind::ExpectedAttribute =>
                "expected a literal or identifier",

            ParseErrorKind::ExpectedValue =>
                "expected a parse after `=`",

            ParseErrorKind::ExpectedTailfishIdent =>
                "expected identifier after inline container",
            ParseErrorKind::ExpectedTailfishPunct =>
                "expected tailfish (`::`)",

            ParseErrorKind::InvalidContainerGroup =>
                "invalid container group, expected parenthesis `()`",

            ParseErrorKind::InvalidContent =>
                "expected `(...)`, `::` or `=`",
            ParseErrorKind::UnexpectedEnd =>
                "unexpected end of tokens"
        }
    }
}