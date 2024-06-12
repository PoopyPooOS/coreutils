use clap::Parser;
use std::{fs, path::PathBuf};

#[derive(Debug, Parser)]
struct Cli {
    paths: Vec<PathBuf>,
}

fn main() {
    let cli = Cli::parse();

    for path in cli.paths {
        fs::create_dir_all(path).expect("Failed to create directory");
    }
}
