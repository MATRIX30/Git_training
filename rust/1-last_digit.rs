use std::fs::File;
use std::io::Read;

/// Generates a pseudo-random integer between -10000 and 10000.
fn random_int_gen() -> i32 {
    let mut buffer = [0u8; 4];
    if let Ok(mut file) = File::open("/dev/urandom") {
        if file.read_exact(&mut buffer).is_ok() {
            let val = i32::from_ne_bytes(buffer);
            return val % 10001;
        }
    }
    // Fallback based on UNIX timestamp nanoseconds
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .subsec_nanos() as i32;
    (nanos % 20001) - 10000
}

fn main() {
    let number = random_int_gen();
    let last_digit = number % 10;

    print!("Last digit of {} is {} and is ", number, last_digit);

    if last_digit > 5 {
        println!("greater than 5");
    } else if last_digit == 0 {
        println!("0");
    } else {
        println!("less than 6 and not 0");
    }
}
