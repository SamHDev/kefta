use std::fmt::{Debug, Display, Formatter, Pointer, Write};
use proc_macro2::{Span, TokenStream};
use kefta_core::model::{MetaFlavour, MetaParser, MetaVisitor};
use kefta_core::model::error::MetaError;

struct Value(pub &'static str);

#[derive(Debug)]
struct Error(pub String);

impl MetaError for Error {
    fn custom(_span: Option<Span>, message: impl Display) -> Self {
        Error(message.to_string())
    }

    fn into_token_stream(self) -> TokenStream {
        TokenStream::new()
    }
}

impl MetaFlavour for Value {
    fn error_fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        f.write_fmt(format_args!("value '{}'", self.0))
    }
}

enum Meta {
    Path(&'static str, Box<Meta>),
    Marker,
    Value(Value),
    List
}

impl MetaParser<Value> for Meta {
    type Error = Error;

    fn parse<V>(self, visitor: V) -> Result<V::Output, Self::Error> where V: MetaVisitor<Flavour=Value> {
        match self {
            Meta::Path(x, cont) => visitor.visit_path(None, Some(x), *cont),
            Meta::Marker => visitor.visit_marker(None),
            Meta::Value(x) => visitor.visit_value(None, x),
            Meta::List => Err(Error::custom(None, "error"))
        }
    }
}

fn main() {
    let meta = Meta::Path("foo", Box::new(Meta::Value(Value("hello world"))));

    #[derive(Debug)]
    struct _Data {
        foo: Option<&'static str>
    }

    struct _Visitor0<'a>(&'a mut _Data);

    impl<'a> MetaVisitor for _Visitor0<'a> {
        type Flavour = Value;
        type Output = ();

        fn expecting(&self, f: &mut Formatter) -> std::fmt::Result {
            f.write_str("a value or list of values")
        }

        fn visit_path<P>(self, span: Option<Span>, path: Option<&str>, child: P) -> Result<Self::Output, P::Error> where P: MetaParser<Self::Flavour> {
            match path {
                Some("foo") => child.parse(_Visitor1(self.0)),
                _ => Err(P::Error::unknown_field(span, path.unwrap_or("::"), None))
            }
        }
    }

    struct _Visitor1<'a>(&'a mut _Data);

    impl<'a> MetaVisitor for _Visitor1<'a> {
        type Flavour = Value;
        type Output = ();

        fn expecting(&self, f: &mut Formatter) -> std::fmt::Result {
            f.write_str("a string value")
        }

        fn visit_value<E>(self, _span: Option<Span>, value: Self::Flavour) -> Result<Self::Output, E> where E: MetaError {
            self.0.foo = Some(value.0);
            Ok(())
        }
    }

    let mut _data = _Data {
        foo: None
    };
    meta.parse(_Visitor0(&mut _data)).unwrap();
    println!("{:?}", _data);
}