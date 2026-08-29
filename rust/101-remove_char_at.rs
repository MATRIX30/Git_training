/// Returns a new string with the character at index `n` removed.
fn remove_char_at(s: &str, n: i32) -> String {
    if n < 0 {
        return s.to_string();
    }
    let mut result = String::new();
    for (i, c) in s.chars().enumerate() {
        if i as i32 != n {
            result.push(c);
        }
    }
    result
}

fn main() {
    println!("{}", remove_char_at("Best School", 3));
    println!("{}", remove_char_at("Chicago", 2));
    println!("{}", remove_char_at("C is fun!", 0));
    println!("{}", remove_char_at("School", 10));
    println!("{}", remove_char_at("Python", -2));
}
