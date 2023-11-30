use proc_macro2::{Span, TokenStream, TokenTree};
use kefta_core::{MetaError, MetaParser, MetaVisitor};

pub struct BuiltinParser {
    last_span: Option<Span>,
    iter: <TokenStream as IntoIterator>::IntoIter
}

impl BuiltinParser {
    fn next(&mut self) ->
}

impl MetaParser for BuiltinParser {
    type Type = TokenTree;
    type Error = String;

    fn parse_any<V>(self, visitor: V) -> Result<V::Output, Self::Error> where V: MetaVisitor<Type=Self::Type> {
        match self.iter.next() {
            None => Self::Error::unexpected_eoi(self.last_span, "a valid meta expression"),

            Some(TokenTree::Punct(x)) if x.as_char() == ':' => {
                self.last_span = None
            }

            Some(TokenTree::Ident(ident)) => {

            }
        }
    }

    fn parse_path<V>(self, visitor: V) -> Result<V::Output, Self::Error> where V: MetaVisitor<Type=Self::Type> {
        todo!()
    }
}