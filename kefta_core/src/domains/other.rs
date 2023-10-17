use std::fmt::Display;
use proc_macro2::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};
use crate::model::{MetaDomain, MetaError};

#[allow(clippy::needless_lifetimes)]
impl MetaDomain for TokenStream {
    type ErrorDisplay<'a> = &'a TokenStream where Self: 'a;

    fn as_error_display<'a>(&'a self) -> Self::ErrorDisplay<'a> {
        self
    }
}

pub(crate) fn _build_compile_error(span: Option<Span>, message: String) -> TokenStream {
    let _span = span.unwrap_or_else(Span::call_site);

    let mut stream = TokenStream::new();

    let _macro_ident = Ident::new("compile_error", _span);
    let _macro_punct = {
        let mut p = Punct::new(':', Spacing::Joint);
        p.set_span(_span);
        p
    };

    let _macro_lit = {
        let mut l = Literal::string(&message);
        l.set_span(_span);
        l
    };

    let _macro_group = {
        Group::new(
            Delimiter::Parenthesis,
            TokenStream::from(TokenTree::Literal(_macro_lit))
        )
    };

    stream.extend([
        TokenTree::Ident(_macro_ident),
        TokenTree::Punct(_macro_punct),
        TokenTree::Group(_macro_group)
    ].into_iter());

    stream
}

impl MetaError for String {
    fn into_token_stream(self) -> TokenStream {
        _build_compile_error(None, self)

    }

    fn custom(_span: Option<Span>, message: impl Display) -> Self {
        message.to_string()
    }
}

impl MetaError for (Option<Span>, String) {
    fn into_token_stream(self) -> TokenStream {
        _build_compile_error(self.0, self.1)
    }

    fn custom(span: Option<Span>, message: impl Display) -> Self {
        (span, message.to_string())
    }
}

impl MetaError for (String, Option<Span>) {
    fn into_token_stream(self) -> TokenStream {
        _build_compile_error(self.1, self.0)
    }

    fn custom(span: Option<Span>, message: impl Display) -> Self {
        (message.to_string(), span)
    }
}

impl MetaError for (Span, String) {
    fn into_token_stream(self) -> TokenStream {
        _build_compile_error(Some(self.0), self.1)
    }

    fn custom(span: Option<Span>, message: impl Display) -> Self {
        (span.unwrap_or_else(Span::call_site), message.to_string())
    }
}

impl MetaError for (String, Span) {
    fn into_token_stream(self) -> TokenStream {
        _build_compile_error(Some(self.1), self.0)
    }

    fn custom(span: Option<Span>, message: impl Display) -> Self {
        (message.to_string(), span.unwrap_or_else(Span::call_site))
    }
}