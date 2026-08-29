use std::fs::File;
use std::io::Read;

/// Generates a pseudo-random integer between -9999 and 9999.
/// Reads from /dev/urandom on Unix systems, with a time-based fallback.
fn random_int_gen() -> i32 {
    let mut buffer = [0u8; 4];
    if let Ok(mut file) = File::open("/dev/urandom") {
        if file.read_exact(&mut buffer).is_ok() {
            let val = i32::from_ne_bytes(buffer);
            return val % 10000;
        }
    }
    // Fallback based on UNIX timestamp nanoseconds
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .subsec_nanos() as i32;
    (nanos % 19999) - 9999
}

fn main() {
    let number = random_int_gen();

    if number > 0 {
        println!("{} is positive", number);
    } else if number == 0 {
        println!("{} is zero", number);
    } else {
        println!("{} is negative", number);
    }
}
