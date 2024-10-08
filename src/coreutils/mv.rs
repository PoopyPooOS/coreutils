use clap::Parser;
use std::{fs, io, path::PathBuf, process, sync::Arc};

#[derive(Debug, Parser)]
struct Cli {
    #[arg(required = true)]
    sources: Vec<PathBuf>,
    #[arg(required = true)]
    destination: PathBuf,
}

pub fn mv(args: impl Iterator<Item = String>) -> io::Result<()> {
    let cli = Cli::parse_from(args);
    let destination = Arc::new(cli.destination);

    if destination.exists() {
        eprintln!(
            "mv: cannot move '{}' to '{}': File exists",
            cli.sources[0].display(),
            destination.display()
        );
        process::exit(1);
    }

    for path in cli.sources.iter() {
        let dest = if destination.is_dir() {
            let file_name = path.file_name().expect("Failed to get file name");
            destination.join(file_name)
        } else {
            destination.to_path_buf()
        };

        fs::rename(path, &dest)?;
    }

    Ok(())
}
