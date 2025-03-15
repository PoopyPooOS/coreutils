use clap::Parser;
use std::{fs, io, path::PathBuf};

#[derive(Debug, Parser)]
struct Cli {
    paths: Vec<PathBuf>,
}

pub fn mkdir(args: impl Iterator<Item = String>) -> io::Result<()> {
    let cli = Cli::parse_from(args);

    for path in cli.paths {
        fs::create_dir_all(path).expect("Failed to create directory");
    }

    Ok(())
}
