#![feature(duration_constructors, iter_intersperse)]
#![allow(clippy::unnecessary_wraps)]

use std::{env, io, process};

mod coreutils;

fn main() -> io::Result<()> {
    let exe = env::args().next().expect("Failed to get executable name");
    let exe = exe.split('/').last().expect("Failed to get executable name");

    let args = env::args();

    match exe {
        "ls" => coreutils::ls(args),
        "cat" => coreutils::cat(args),
        "clear" => coreutils::clear(),
        "cp" => coreutils::cp(args),
        "echo" => coreutils::echo(args),
        "ln" => coreutils::ln(args),
        "mkdir" => coreutils::mkdir(args),
        "mount" => coreutils::mount(args),
        "mv" => coreutils::mv(args),
        "printenv" => coreutils::printenv(),
        "rm" => coreutils::rm(args),
        "sleep" => coreutils::sleep(args),
        "touch" => coreutils::touch(args),

        "coreutils" => {
            eprintln!("This binary is used through a symlink.");
            process::exit(1);
        }
        _ => {
            eprintln!("Command not found: {exe}");
            process::exit(1);
        }
    }
}
