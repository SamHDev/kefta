use test_macro::ExampleMacro;

fn main() {
    println!("Hello, world!");

}

#[derive(ExampleMacro)]
#[eg("Hello World!")]
/// This is a doc comment!
#[eg(locale("en", "foo"))]
pub struct Target;