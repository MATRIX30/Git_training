/// Computes a to the power of b and returns the value.
fn power(a: i32, b: i32) -> f64 {
    (a as f64).powi(b)
}

fn main() {
    println!("{}", power(2, 2));
    println!("{}", power(98, 2));
    println!("{}", power(98, 0));
    println!("{}", power(100, -2));
    println!("{}", power(-4, 5));
}
