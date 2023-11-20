#[allow(unused_imports)]
use rand::Rng;

fn main() {
    let tuple = rand::random::<(f64, char)>();
    println!("{:?}", tuple);
}
