mod model;
mod _syn;
mod util;

mod _test {
    use std::fmt::Formatter;
    use proc_macro2::{Ident, Span, TokenStream};
    use quote::quote;
    use syn::{Attribute, Meta, parse2};
    use syn::parse::{Parse, ParseBuffer, Parser, ParseStream};
    use crate::model;
    use crate::model::{FromMeta, MetaAccess, MetaError, MetaSource, MetaVisitor};
    use crate::util::meta_match_visitor;

    #[test]
    fn test() {

        let attr = quote!( example(foo, bar=10, baz(test)) );
        let parse = Parser::parse2( Meta::parse, attr).unwrap();

        Example::from_meta(parse).unwrap();
    }

    struct Example {
        foo: bool,
        bar: usize,
        baz: Vec<String>,
    }

    impl FromMeta for Example {
        fn from_meta<S>(source: S) -> Result<Self, S::Error> where S: MetaSource {

            #[derive(Debug)]
            struct _Data {
                example_foo: Option<bool>,
                example_bar: Option<usize>,
                example_baz: Option<Vec<String>>,
            }

            struct _Visitor0<'a>(&'a mut _Data);

            impl<'a> MetaVisitor for _Visitor0<'a> {
                type Output = ();

                fn expecting(&self, fmt: &mut Formatter) -> std::fmt::Result {
                    fmt.write_str("a meta identifier")
                }

                fn visit_path<S>(self, source: S, path: Option<&str>, _span: Option<Span>) -> Result<Self::Output, S::Error> where S: MetaSource {
                    match path {
                        Some("example") => source.visit(_Visitor1(self.0)),
                        _ => Ok(())
                    }
                }

                fn visit_list<A>(self, mut access: A, span: Option<Span>) -> Result<Self::Output, A::Error> where A: MetaAccess {
                    while access.remaining() {
                        access.visit(Self(self.0))?;
                    }
                    Ok(())
                }
            }


            struct _Visitor1<'a>(&'a mut _Data);

            impl<'a> MetaVisitor for _Visitor1<'a> {
                type Output = ();

                fn expecting(&self, fmt: &mut Formatter) -> std::fmt::Result {
                    fmt.write_str("a list of elements")
                }

                fn visit_list<A>(self, mut access: A, _span: Option<Span>) -> Result<Self::Output, A::Error> where A: MetaAccess {
                    while access.remaining() {
                        access.visit(_Visitor2(self.0))?;
                    }
                    Ok(())
                }
            }

            struct _Visitor2<'a>(&'a mut _Data);

            impl<'a> MetaVisitor for _Visitor2<'a> {
                type Output = ();

                fn expecting(&self, fmt: &mut Formatter) -> std::fmt::Result {
                    fmt.write_str("a list of elements")
                }

                fn visit_path<S>(self, source: S, path: Option<&str>, span: Option<Span>) -> Result<Self::Output, S::Error> where S: MetaSource {
                    match path {
                        Some("foo") => { self.0.example_foo = Some(<bool as FromMeta>::from_meta(source)?) },
                        _ => ()
                    };
                    Ok(())
                }
            }

            let mut _data = _Data {
                example_foo: None,
                example_bar: None,
                example_baz: None,
            };

            source.visit(_Visitor0(&mut _data))?;

            println!("{:?}", _data);

            Ok(Self {
                foo: match _data.example_foo {
                    Some(x) => x,
                    None => return Err(S::Error::custom("invalid", None)),
                },
                bar: 0,
                baz: vec![],
            })
        }
    }

    struct Example2 {
        foo: bool,
        bar: usize,
        baz: Vec<String>,
    };

    impl FromMeta for Example2 {
        fn from_meta<S>(source: S) -> Result<Self, S::Error> where S: MetaSource {
            struct _Def {
                example_foo: Option<bool>,
                example_bar: Option<usize>,
                example_baz: Option<Vec<String>>
            }

            let mut _def = _Def {
                example_foo: None,
                example_bar: None,
                example_baz: None,
            };

            meta_match_visitor! {
                _Vis0 ( _Def ) {
                    "example" => _Vis1
                }
            };

            meta_match_visitor! {
                _Vis1 ( _Def ) {
                    "example" => {  }
                }
            };


            source.visit(_Vis0(&mut _def))?;

            Ok(Self {
                foo: _def.example_foo
                    .unwrap_or_else(|| Default::default()),
                bar: _def.example_bar
                    .unwrap_or_else(|| 0),
                baz: _def.example_baz
                    .unwrap_or_else(|| Vec::new()),
            })
        }
    }
}

