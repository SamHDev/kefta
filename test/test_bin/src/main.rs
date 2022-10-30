use test_macro::ExampleMacro;

fn main() {
    println!("Hello, world!");

}

#[derive(ExampleMacro)]
pub struct Target;