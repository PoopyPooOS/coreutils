use clap::Parser;
use std::{fs, io, path::PathBuf};

#[derive(Debug, Parser)]
struct Cli {
    #[arg(required = true)]
    sources: Vec<PathBuf>,
    destination: PathBuf,
}

pub fn cp(args: impl Iterator<Item = String>) -> io::Result<()> {
    let cli = Cli::parse_from(args);

    let destination_is_dir = cli.destination.is_dir();

    for path in &cli.sources {
        let dest = if destination_is_dir {
            let file_name = path.file_name().unwrap();
            cli.destination.join(file_name)
        } else {
            cli.destination.clone()
        };

        match fs::copy(path, &dest) {
            Ok(_) => (),
            Err(err) => eprintln!("Error: {err}"),
        }
    }

    Ok(())
}
