use crate::parsers::builtin::error::BuiltinParserError;
use crate::parsers::builtin::parser::BuiltinParser;
use crate::model::{MetaAccess, MetaDomain, MetaError, MetaReceiver, MetaSource, MetaVisitor};
use proc_macro2::{Span, TokenStream, TokenTree};

pub type _Error = (Option<Span>, String);

#[allow(clippy::needless_lifetimes)]
impl MetaDomain for TokenTree {
    type ErrorDisplay<'a> = &'a TokenTree where Self: 'a;

    fn as_error_display<'a>(&'a self) -> Self::ErrorDisplay<'a> {
        self
    }
}

impl MetaSource<TokenTree> for TokenStream {
    type Error = BuiltinParserError<_Error>;

    fn visit<V>(self, visitor: V) -> Result<V::Output, Self::Error>
    where
        V: MetaVisitor<Domain = TokenTree>,
    {
        let mut parser = BuiltinParser::new(self, None);

        let token = parser.next();
        parser.parse_root_item(token, visitor)
    }
}

pub(crate) struct BuiltinSource<'a, E>
where
    E: MetaError,
{
    parser: &'a mut BuiltinParser<E>,
}

impl<'a, E> MetaSource<TokenTree> for BuiltinSource<'a, E>
where
    E: MetaError,
{
    type Error = BuiltinParserError<E>;

    fn visit<V>(self, visitor: V) -> Result<V::Output, Self::Error>
    where
        V: MetaVisitor<Domain = TokenTree>,
    {
        let token = self.parser.next();
        self.parser.parse_item(token, visitor)
    }
}

pub(crate) struct BuiltinContSource<'a, E>
where
    E: MetaError,
{
    pub(crate) parser: &'a mut BuiltinParser<E>,
}

impl<'a, E> MetaSource<TokenTree> for BuiltinContSource<'a, E>
where
    E: MetaError,
{
    type Error = BuiltinParserError<E>;

    fn visit<V>(self, visitor: V) -> Result<V::Output, Self::Error>
    where
        V: MetaVisitor<Domain = TokenTree>,
    {
        let token = self.parser.next();
        self.parser.parse_item_cont(token, visitor)
    }
}

pub(crate) struct BuiltinAccess<'a, E>
where
    E: MetaError,
{
    pub(crate) parser: &'a mut BuiltinParser<E>,
}

impl<'a, E> MetaAccess<TokenTree> for BuiltinAccess<'a, E>
where
    E: MetaError,
{
    type Error = BuiltinParserError<E>;

    fn next<R>(&mut self, receiver: R) -> Option<Result<R::Output, Self::Error>>
    where
        R: MetaReceiver<TokenTree>,
    {
        match self.parser.next() {
            None => None,

            Some(TokenTree::Punct(punct)) if punct.as_char() == ',' => {
                Some(receiver.receive(BuiltinSource {
                    parser: self.parser,
                }))
            }

            Some(token) => Some(Err(BuiltinParserError::ExpectedDelimiter { token })),
        }
    }
}
