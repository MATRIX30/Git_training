fn main() {
    for i in 0..10 {
        for j in (i + 1)..10 {
            if i == 8 && j == 9 {
                println!("{}{}", i, j);
            } else {
                print!("{}{}, ", i, j);
            }
        }
    }
}
