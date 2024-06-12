use clap::Parser;
use std::os::unix;

#[derive(Debug, Parser)]
struct Cli {
    source: String,
    target: String,
}

fn main() {
    let cli = Cli::parse();

    unix::fs::symlink(cli.source, cli.target).expect("Failed to create symlink");
}
