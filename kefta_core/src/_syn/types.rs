use std::fmt::Formatter;
use proc_macro2::Span;
use crate::model::{FromMeta, MetaError, MetaSource, MetaVisitor};

impl FromMeta for bool {
    fn from_meta<S>(source: S) -> Result<Self, S::Error> where S: MetaSource {
        struct _Visit;

        impl MetaVisitor for _Visit {
            type Output = bool;

            fn expecting(&self, fmt: &mut Formatter) -> std::fmt::Result {
                fmt.write_str("a word")
            }

            fn visit_word<E>(self, _span: Option<Span>) -> Result<Self::Output, E> where E: MetaError {
                Ok(true)
            }
        }

        source.visit(_Visit)
    }
}
