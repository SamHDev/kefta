use core::fmt;
use proc_macro2::{Span, TokenStream};

pub trait MetaError: Sized {
    fn into_token_stream(self) -> TokenStream;

    fn custom(span: Option<Span>, message: impl fmt::Display) -> Self;

    fn expected(span: Option<Span>, expected: impl MetaExpected, found: impl fmt::Display) -> Self {
        let _expected = &expected as &dyn MetaExpected;
        Self::custom(span, format_args!("expected {_expected}, found {found}"))
    }
    
    fn unexpected_eoi(span: Option<Span>, expected: impl MetaExpected) -> Self {
        let _expected = &expected as &dyn MetaExpected;
        Self::custom(span, format_args!("unexpected end of input. expected {_expected}"))
    }
}

pub trait MetaExpected {
    fn expected(&self, f: &mut fmt::Formatter) -> fmt::Result;
}

impl<'a> MetaExpected for &'a str {
    fn expected(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self)
    }
}

impl<'a> MetaExpected for fmt::Arguments<'a> {
    fn expected(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_fmt(*self)
    }
}

impl<'a> fmt::Display for &'a dyn MetaExpected {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        MetaExpected::expected(*self, f)
    }
}

#[cfg(feature="default_error")]
mod default_error {
    use std::fmt::Display;
    use proc_macro2::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};
    use crate::MetaError;

    fn build_error_token_stream(span: Option<Span>, message: &str) -> TokenStream {
        let span = span.unwrap_or_else(Span::call_site);

        let mut lit = Literal::string(message);
        lit.set_span(span);

        let macro_tokens = [
            TokenTree::Ident(Ident::new("compile_error", span)),
            TokenTree::Punct(Punct::new('!', Spacing::Joint)),
            TokenTree::Group(Group::new(
                Delimiter::Parenthesis,
                TokenStream::from(TokenTree::Literal(
                    lit
                ))
            ))
        ];

        TokenStream::from_iter(macro_tokens)
    }

    impl MetaError for (String, Option<Span>) {
        fn into_token_stream(self) -> TokenStream {
            build_error_token_stream(self.1, &self.0)
        }

        fn custom(span: Option<Span>, message: impl Display) -> Self {
            (message.to_string(), span)
        }
    }

    impl MetaError for (Option<Span>, String) {
        fn into_token_stream(self) -> TokenStream {
            build_error_token_stream(self.0, &self.1)
        }

        fn custom(span: Option<Span>, message: impl Display) -> Self {
            (span, message.to_string())
        }
    }

    impl MetaError for (String, Span) {
        fn into_token_stream(self) -> TokenStream {
            build_error_token_stream(Some(self.1), &self.0)
        }

        fn custom(span: Option<Span>, message: impl Display) -> Self {
            (message.to_string(), span.unwrap_or_else(Span::call_site))
        }
    }

    impl MetaError for (Span, String) {
        fn into_token_stream(self) -> TokenStream {
            build_error_token_stream(Some(self.0), &self.1)
        }

        fn custom(span: Option<Span>, message: impl Display) -> Self {
            (span.unwrap_or_else(Span::call_site), message.to_string())
        }
    }

    impl MetaError for String {
        fn into_token_stream(self) -> TokenStream {
            build_error_token_stream(None, &self)
        }

        fn custom(_span: Option<Span>, message: impl Display) -> Self {
            message.to_string()
        }
    }
}