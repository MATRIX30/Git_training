fn main() {
    for i in (0..26).rev() {
        let ch = if i % 2 == 1 {
            (b'a' + i as u8) as char
        } else {
            (b'A' + i as u8) as char
        };
        print!("{}", ch);
    }
}
