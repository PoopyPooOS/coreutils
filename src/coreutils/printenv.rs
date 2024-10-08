use std::{env, io};

pub fn printenv() -> io::Result<()> {
    for env_var in env::vars() {
        println!("{}={}", env_var.0, env_var.1);
    }

    Ok(())
}
