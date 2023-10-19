use proc_macro2::{Span, TokenStream};
use syn::{Expr, Meta, Path, PathSegment, Token};
use syn::parse::Parser;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use crate::model::{MetaAccess, MetaDomain, MetaError, MetaReceiver, MetaSource, MetaVisitor};

#[allow(clippy::needless_lifetimes)]
impl MetaDomain for Expr {
    type ErrorDisplay<'a> = String;

    fn as_error_display<'a>(&'a self) -> Self::ErrorDisplay<'a> {
        format!("{:?}", self)
    }
}


impl MetaSource<Expr> for Meta {
    type Error = syn::Error;

    fn visit<V>(self, visitor: V) -> Result<V::Output, Self::Error> where V: MetaVisitor<Domain=Expr> {
        match self {
            Meta::Path(path) => _SynPathIterator::new(
                _SynMarkerSource(Some(path.span())),
                path
            ).visit(visitor),
            Meta::List(list) => _SynPathIterator::new(
                _SynListSource::Unparsed(
                    list.delimiter.span().span(),
                    Some(list.tokens)
                ),
                list.path,
            ).visit(visitor),
            Meta::NameValue(value) => _SynPathIterator::new(
                value.value,
                value.path,
            ).visit(visitor)
        }
    }
}

struct _SynPathIterator<S> where S: MetaSource<Expr, Error=syn::Error>  {
    leading_colon: Option<Span>,
    segments: <Punctuated<PathSegment, Token![::]> as IntoIterator>::IntoIter,
    continue_source: S
}

impl<S> _SynPathIterator<S>  where S: MetaSource<Expr, Error=syn::Error> {
    pub fn new(continue_source: S, path: Path) -> Self {
        Self {
            leading_colon: path.leading_colon.as_ref().map(Spanned::span),
            segments: path.segments.into_iter(),
            continue_source,
        }
    }
}

impl<S> MetaSource<Expr> for _SynPathIterator<S> where S: MetaSource<Expr, Error=syn::Error> {
    type Error = syn::Error;

    fn visit<V>(mut self, visitor: V) -> Result<V::Output, Self::Error> where V: MetaVisitor<Domain=Expr> {
        if let Some(leading_colon) = self.leading_colon.take() {
            visitor.visit_path(
                Some(leading_colon.span()),
                None,
                self
            )
        } else if let Some(segment) = self.segments.next() {
            if !segment.arguments.is_empty() {
                return Err(syn::Error::custom(
                    Some(segment.arguments.span()),
                    "invalid sequence: arguments in path segment"
                ))
            }

            visitor.visit_path(
                Some(segment.ident.span()),
                Some(segment.ident.to_string().as_str()),
                self
            )
        } else {
            self.continue_source.visit(visitor)
        }
    }
}


struct _SynMarkerSource(Option<Span>);

impl MetaSource<Expr> for _SynMarkerSource {
    type Error = syn::Error;

    fn visit<V>(self, visitor: V) -> Result<V::Output, Self::Error> where V: MetaVisitor<Domain=Expr> {
       visitor.visit_marker(self.0)
    }
}

enum _SynListSource {
    Unparsed(Span, Option<TokenStream>),
    Parsed(Span, <Punctuated<Meta, Token![,]> as IntoIterator>::IntoIter),
}

impl _SynListSource {
    fn span(&self) -> Span {
        match &self {
            _SynListSource::Unparsed(x, _) |
            _SynListSource::Parsed(x, _) => x,
        }.clone()
    }
}

impl MetaSource<Expr> for _SynListSource {
    type Error = syn::Error;

    fn visit<V>(self, visitor: V) -> Result<V::Output, Self::Error> where V: MetaVisitor<Domain=Expr> {
        visitor.visit_list(
            Some(self.span()),
            self
        )
    }
}

impl MetaAccess<Expr> for _SynListSource {
    type Error = syn::Error;

    fn next<R>(&mut self, receiver: R) -> Option<Result<R::Output, Self::Error>> where R: MetaReceiver<Expr> {

        let meta = match self {
            Self::Unparsed(span, None) => return Some(Err(Self::Error::custom(
                Some(span.clone()),
                    "internal error: token source consumed while parsing list"
            ))),

            Self::Unparsed(span, tokens @ Some(_)) => {
                let tokens = tokens.take().unwrap();
                let parser = Punctuated::<Meta, Token![,]>::parse_separated_nonempty;

                let mut parsed = match parser.parse2(tokens) {
                    Ok(x) => x.into_iter(),
                    Err(e) => return Some(Err(e))
                };

                let this = parsed.next();
                *self = Self::Parsed(span.clone(), parsed);
                this
            }

            Self::Parsed(_, parsed) => parsed.next()
        };

        meta.map(|x| receiver.receive(x))
    }
}

impl MetaSource<Expr> for Expr {
    type Error = syn::Error;

    fn visit<V>(self, visitor: V) -> Result<V::Output, Self::Error> where V: MetaVisitor<Domain=Expr> {
        visitor.visit_value(Some(self.span()), self)
    }
}