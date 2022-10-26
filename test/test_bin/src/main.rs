use test_macro::testing;

fn main() {
    println!("Hello, world!");

    testing! {
        bar(name="Hello World", is_test=yes)
    }
}

