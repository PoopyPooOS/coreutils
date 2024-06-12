use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let message = &args[1..].join(" ");

    println!("{}", message);
}
