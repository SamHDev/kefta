use kefta_test::TestMacro;

fn main() {
    println!("Hello, world!");
}

#[derive(TestMacro)]
#[test(foo = "Hello World", baz::alpha = "Test!", baz(beta))]
pub struct MyStruct {
    foo: String,
}