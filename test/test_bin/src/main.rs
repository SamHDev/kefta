use test_macro::testing;

fn main() {
    println!("Hello, world!");

    testing! {
        bar("Hello World")
    }
}

