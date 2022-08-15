#![allow(dead_code)]

use kefta_test::TestMacro;

fn main() {
    println!("Hello, world!");
}

#[derive(TestMacro)]
#[test(baz::alpha = "Test!", baz(beta))]
#[test(foo=69)]
pub struct MyStruct {
    foo: String,
}