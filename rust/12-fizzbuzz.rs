/// Prints numbers from 1 to 100 separated by a space following FizzBuzz rules.
fn fizzbuzz() {
    for i in 1..=100 {
        if i % 15 == 0 {
            print!("FizzBuzz ");
        } else if i % 3 == 0 {
            print!("Fizz ");
        } else if i % 5 == 0 {
            print!("Buzz ");
        } else {
            print!("{} ", i);
        }
    }
}

fn main() {
    fizzbuzz();
    println!();
}
