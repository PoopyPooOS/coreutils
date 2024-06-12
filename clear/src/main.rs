use std::io::{stdout, Write};

fn main() {
    print!("\x1B[2J\x1B[1;1H");
    stdout().flush().unwrap();
}