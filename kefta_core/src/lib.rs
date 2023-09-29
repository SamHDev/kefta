mod model;
mod error;
mod types;

mod test {
    use std::fmt::{Formatter, Write};
    use crate::model::{FromMeta, MetaSource, MetaVisitor};

    struct Example {
        foo: bool,
        bar: Option<char>
    }

    impl FromMeta for Example {
        fn source<S>(source: &mut S) -> Result<Self, S::Error> where S: MetaSource {
            struct _0 {
                foo: Option<bool>,
                bar: Option<Option<char>>
            }

            impl _0 {
                const INIT: Self = Self {
                    foo: None,
                    bar: None
                };
            }

            // this can be abstracted behind private util
            impl MetaVisitor for _0 {
                type Output = _0;

                fn expecting(&self, fmt: &mut Formatter) -> std::fmt::Result {
                    fmt.write_str("a list of attributes")
                }

                fn visit_list<S>(mut self, mut contents: S) -> Result<Self::Output, S::Error> where S: MetaSource {
                    while contents.remaining() {
                        contents.visit(_1(&mut self))?
                    }
                    Ok(self)
                }
            }

            struct _1<'a>(&'a mut _0);

            // actual meat and gravy
            impl MetaVisitor for _1 {
                type Output = ();

                fn expecting(&self, fmt: &mut Formatter) -> std::fmt::Result {
                    fmt.write_str("path: `foo` or `bar`")
                }

                fn visit_path<S>(self, path: &str, source: &mut S) -> Result<Self::Output, S::Error> where S: MetaSource {
                    // match path
                    match path {
                        "foo" if self.0.foo.is_some() => return Err(S::Error::duplicate("foo").with_span(source.pos())),
                        "foo" => { self.0.foo = Some(bool::from_meta(source)?); },

                        "bar" if self.0.bar.is_some() => return Err(S::Error::duplicate("foo").with_span(source.pos())),
                        "bar" => { self.0.bar = Some(<Option<char>>::from_meta(source)?); },

                        _ => return Err(S::Error::unknown("foo or bar", path).with_span(source.pos()))
                    };
                    Ok(())
                }
            }

            // call
            let a = source.visit(_0::INIT)?;

            // extact
            Ok(Self {
                foo: a.foo.unwrap_or_default(),
                bar: a.bar.unwrap_or_default(),
            })
        }

    }
}