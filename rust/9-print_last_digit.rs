/// Prints the last digit of a number (without a newline) and returns it.
fn print_last_digit(number: i32) -> i32 {
    let last = (number % 10).abs();
    print!("{}", last);
    last
}

fn main() {
    print_last_digit(98);
    print_last_digit(0);
    let r = print_last_digit(-1024);
    println!("{}", r);
}
