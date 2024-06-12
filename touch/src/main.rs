use clap::Parser;
use std::{fs::File, path::PathBuf};

#[derive(Debug, Parser)]
struct Cli {
    paths: Vec<PathBuf>,
}

fn main() {
    let cli = Cli::parse();

    for path in cli.paths {
        File::create(&path).expect("Failed to create file");
    }
}
