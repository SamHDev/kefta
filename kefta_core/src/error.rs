use proc_macro::{Span, TokenStream, TokenTree};
use syn::__private::quote::{quote, quote_spanned};
use syn::__private::TokenStream2;
use crate::node::AttrNode::Literal;
use crate::node::ParseError;


#[derive(Debug)]
pub enum KeftaError {
    ParseError(ParseError),

    Expected(KeftaErrorContext),
    Multiple(KeftaErrorContext),

    ExpectedValue(KeftaErrorContext),
    ExpectedNamed(KeftaErrorContext),
    ExpectedType {
        expected: Option<String>,
        context: KeftaErrorContext
    }
}

pub type KeftaResult<T> = Result<T, KeftaError>;

#[derive(Debug, Default)]
pub struct KeftaErrorContext {
    pub key: Option<String>,
    pub span: Option<Span>,
}

impl KeftaError {
    pub fn add_ctx_key(mut self, key: &str) -> Self {
        match &mut self {
            KeftaError::Expected(ctx) |
            KeftaError::Multiple(ctx) |
            KeftaError::ExpectedValue(ctx) |
            KeftaError::ExpectedNamed(ctx) |
            KeftaError::ExpectedType { context: ctx, .. } => {
                ctx.key = Some(key.to_string())
            }
            _ => ()
        }
        self
    }

    pub fn add_ctx_span(mut self, span: &Span) -> Self {
        match &mut self {
            KeftaError::Expected(ctx) |
            KeftaError::Multiple(ctx) |
            KeftaError::ExpectedValue(ctx) |
            KeftaError::ExpectedNamed(ctx) |
            KeftaError::ExpectedType { context: ctx, .. } => {
                ctx.span = Some(span.clone())
            }
            _ => ()
        }
        self
    }

    pub fn parts(self) -> (String, Option<Span>) {
        match self {
            KeftaError::ParseError(error) => (
                error.kind.message().to_string(),
                error.span
            ),
            KeftaError::Expected(ctx) => {
                (
                    if let Some(name) = ctx.key {
                        format!("expected attribute '{}'", name)
                    } else {
                        format!("expected attribute")
                    },
                    ctx.span
                )
            }
            KeftaError::Multiple(ctx) => (
                if let Some(name) = ctx.key {
                    format!("multiple values for attribute '{}'", name)
                } else {
                    format!("multiple values")
                },
                ctx.span
            ),
            KeftaError::ExpectedValue(ctx) => (
                if let Some(name) = ctx.key {
                    format!("expected a value for attribute '{}'", name)
                } else {
                    format!("expected a value")
                },
                ctx.span
            ),
            KeftaError::ExpectedNamed(ctx) => (
                if let Some(name) = ctx.key {
                    format!("expected a named value for attribute '{}'", name)
                } else {
                    format!("expected a named value")
                },
                ctx.span
            ),
            KeftaError::ExpectedType { context: ctx, expected } => (
                match (ctx.key, expected) {
                    (None, None) => format!("invalid type"),
                    (Some(key), None) => format!("invalid type for attribute '{}' ", key),
                    (None, Some(typ)) => format!("expected {}", typ,),
                    (Some(key), Some(typ)) => format!("expected {} for attribute '{}' ", typ, key),
                },
                ctx.span
            ),
        }
    }

    pub fn into_compile_error(self) -> TokenStream {
        let (message, span) = self.parts();
        let mut lit = proc_macro::Literal::string(&message);
        if let Some(span) = span { lit.set_span(span) };

        let lit_stream = TokenStream2::from(TokenStream::from(TokenTree::Literal(lit)));
        quote!( compile_error!( #lit_stream ) ).into()
    }
}

