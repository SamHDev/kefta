use proc_macro::{Span, TokenStream, TokenTree};
use syn::__private::quote::{quote};
use syn::__private::TokenStream2;
use crate::node::{AttrNode, ParseError, ParseErrorKind};

#[derive(Debug)]
pub struct KeftaError {
    pub kind: KeftaErrorKind,
    pub key: Option<String>,
    pub span: Option<Span>,
}

#[derive(Debug)]
pub enum KeftaErrorKind {
    ParseError {
        kind: ParseErrorKind
    },

    Expected,
    Multiple,

    ExpectedValue,
    ExpectedNamed,
    ExpectedType {
        expected: Option<String>,
    }
}

pub type KeftaResult<T> = Result<T, KeftaError>;

impl From<ParseError> for KeftaError {
    fn from(error: ParseError) -> Self {
        Self {
            kind: KeftaErrorKind::ParseError { kind: error.kind },
            key: None,
            span: error.span
        }
    }
}

impl From<KeftaErrorKind> for KeftaError {
    fn from(kind: KeftaErrorKind) -> Self {
        Self {
            kind,
            key: None,
            span: None
        }
    }
}


pub trait KeftaErrorContext {
    fn set_context(&mut self, node: &AttrNode);
    fn set_contexts(&mut self, nodes: &Vec<AttrNode>);

    fn set_context_key(&mut self, key: &str);

    fn context(mut self, node: &AttrNode) -> Self where Self: Sized {
        self.set_context(node);
        self
    }
    fn contexts(mut self, nodes: &Vec<AttrNode>) -> Self where Self: Sized  {
        self.set_contexts(nodes);
        self
    }

    fn context_key(mut self, key: &str) -> Self where Self: Sized  {
        self.set_context_key(key);
        self
    }
}

impl KeftaErrorContext for KeftaError {
    fn set_context(&mut self, node: &AttrNode) {
        if self.span.is_none() {

        }

        if self.key.is_none() {
            if let Some(ident) = node.ident() {
                self.key = Some(ident.to_string());
            }
        }
    }

    fn set_contexts(&mut self, nodes: &Vec<AttrNode>) {
        if let Some(first) = nodes.first() {
            self.set_context(first)
        }
    }

    fn set_context_key(&mut self, key: &str) {
        if self.key.is_none() {
            self.key = Some(key.to_string());
        }
    }
}

impl<T> KeftaErrorContext for KeftaResult<T> {
    fn set_context(&mut self, node: &AttrNode) {
        if let Err(error) = self {
            error.set_context(node)
        }
    }

    fn set_contexts(&mut self, nodes: &Vec<AttrNode>) {
        if let Err(error) = self {
            error.set_contexts(nodes)
        }
    }

    fn set_context_key(&mut self, key: &str) {
        if let Err(error) = self {
            error.set_context_key(key)
        }
    }
}

impl KeftaError {
    pub fn parts(self) -> (String, Option<Span>) {
        match self.kind {
            KeftaErrorKind::ParseError { kind } => (
                if let Some(name) = self.key {
                    format!("error while parsing attribute '{}': {}", name, kind.message().to_string())
                } else {
                    kind.message().to_string()
                },
                self.span
            ),
            KeftaErrorKind::Expected => {
                (
                    if let Some(name) = self.key {
                        format!("expected attribute '{}'", name)
                    } else {
                        format!("expected attribute")
                    },
                    self.span
                )
            }
            KeftaErrorKind::Multiple => (
                if let Some(name) = self.key {
                    format!("multiple values for attribute '{}'", name)
                } else {
                    format!("multiple values")
                },
                self.span
            ),
            KeftaErrorKind::ExpectedValue => (
                if let Some(name) = self.key {
                    format!("expected a value for attribute '{}'", name)
                } else {
                    format!("expected a value")
                },
                self.span
            ),
            KeftaErrorKind::ExpectedNamed => (
                if let Some(name) = self.key {
                    format!("expected a named value for attribute '{}'", name)
                } else {
                    format!("expected a named value")
                },
                self.span
            ),
            KeftaErrorKind::ExpectedType { expected } => (
                match (self.key, expected) {
                    (None, None) => format!("invalid type"),
                    (Some(key), None) => format!("invalid type for attribute '{}' ", key),
                    (None, Some(typ)) => format!("expected {}", typ,),
                    (Some(key), Some(typ)) => format!("expected {} for attribute '{}' ", typ, key),
                },
                self.span
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

