fn main() {
    for i in 0..100 {
        if i < 99 {
            print!("{:02}, ", i);
        } else {
            println!("{:02}", i);
        }
    }
}
