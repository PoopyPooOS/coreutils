use clap::Parser;
use std::{fs::File, io, path::PathBuf};

#[derive(Debug, Parser)]
struct Cli {
    paths: Vec<PathBuf>,
}

pub fn touch(args: impl Iterator<Item = String>) -> io::Result<()> {
    let cli = Cli::parse_from(args);

    for path in cli.paths {
        File::create(&path).expect("Failed to create file");
    }

    Ok(())
}
