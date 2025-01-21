use clap::Parser;
use std::{fs, io, path::PathBuf};

#[derive(Debug, Parser)]
struct Cli {
    #[arg(required = true)]
    sources: Vec<PathBuf>,
    #[arg(required = true)]
    destination: PathBuf,
}

pub fn mv(args: impl Iterator<Item = String>) -> io::Result<()> {
    let cli = Cli::parse_from(args);
    let destination = cli.destination;

    if destination.exists() {
        panic!(
            "Cannot move '{}' to '{}', a file already exists at the destination",
            cli.sources[0].display(),
            destination.display()
        );
    }

    for path in &cli.sources {
        let dest = if destination.is_dir() {
            let file_name = path.file_name().expect("Failed to get file name");
            destination.join(file_name)
        } else {
            destination.clone()
        };

        fs::rename(path, &dest).expect("Failed to move file");
    }

    Ok(())
}
