#[macro_export]
/// parse an array of attributes
macro_rules! parse_attr {
    ($expr:expr => $type:ty) => {
        match kefta::AttrParse::parse_attrs::<$type>({ $expr }) {
            Ok(value) => value,
            Err(error) => return error.to_compile_error().into(),
        }
    };
}