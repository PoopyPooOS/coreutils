use clap::Parser;
use std::{io, os::unix};

#[derive(Debug, Parser)]
struct Cli {
    source: String,
    target: String,
}

pub fn ln(args: impl Iterator<Item = String>) -> io::Result<()> {
    let cli = Cli::parse_from(args);
    unix::fs::symlink(cli.source, cli.target)
}
