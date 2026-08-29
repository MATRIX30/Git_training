/// Converts and prints a string in uppercase followed by a new line.
fn uppercase(s: &str) {
    for c in s.chars() {
        let upper = if c >= 'a' && c <= 'z' {
            ((c as u8) - 32) as char
        } else {
            c
        };
        print!("{}", upper);
    }
    println!();
}

fn main() {
    uppercase("best");
    uppercase("Best School 98 Battery street");
}
