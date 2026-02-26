#[flux_rs::no_panic_if(true)]
fn foo() {
    println!("Hello");
}

fn main() {
    foo();
}
