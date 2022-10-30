#[macro_export]
macro_rules! parse_attr {
    ($expr:expr => $typ:ident) => {
        match kefta::AttrSource::parse($expr) {
            Ok(x) => <$typ as kefta::AttrModel>::parse(x),
            Err(e) => Err(e)
        }
    };
}

#[macro_export]
macro_rules! parse_attr_tokens {
    ($expr:expr => $typ:ident) => {
        match kefta::AttrSource::parse($expr) {
            Ok(x) => match <$typ as kefta::AttrModel>::parse(x) {
                Ok(x) => x,
                Err(e) => return e.into_compile_error(),
            },
            Err(e) => return e.into_compile_error(),
        }
    };
}