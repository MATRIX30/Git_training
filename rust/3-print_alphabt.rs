fn main() {
    for c in b'a'..=b'z' {
        let ch = c as char;
        if ch != 'e' && ch != 'q' {
            print!("{}", ch);
        }
    }
}
