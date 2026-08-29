/// Checks whether a character is a lowercase ASCII letter.
fn islower(c: char) -> bool {
    c >= 'a' && c <= 'z'
}

fn main() {
    let test_chars = ['a', 'H', 'A', '3', 'g'];
    for &c in &test_chars {
        println!("{} is {}", c, if islower(c) { "lower" } else { "upper" });
    }
}
