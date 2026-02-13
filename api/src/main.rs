use crate::handlers::url::plus;

mod handlers;
fn main() {
    println!("Hello, world!");
    let gg = "Kronborg";

    let number = plus(6, 7);

    println!("hello {}, {}",  gg, number);
}
