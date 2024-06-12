use std::env;

fn main() {
    for env_var in env::vars() {
        println!("{} = {}", env_var.0, env_var.1);
    }
}
