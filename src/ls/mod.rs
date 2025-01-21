use clap::Parser;
use std::{fs, io, path::PathBuf};

mod formatting;

#[derive(Debug, Parser)]
struct Cli {
    #[arg(short, long)]
    all: bool,

    paths: Vec<PathBuf>,
}

pub fn ls(args: impl Iterator<Item = String>) -> io::Result<()> {
    let mut cli = Cli::parse_from(args);
    cli.paths = if cli.paths.is_empty() {
        vec![PathBuf::from(".")]
    } else {
        cli.paths
    };

    for (index, path) in cli.paths.iter().enumerate() {
        let mut entries = fs::read_dir(path)?
            .filter_map(Result::ok)
            .filter(|entry| {
                if cli.all {
                    true
                } else {
                    !entry
                        .file_name()
                        .to_string_lossy()
                        .to_string()
                        .starts_with('.')
                }
            })
            .collect::<Vec<_>>();

        if entries.is_empty() {
            continue;
        }

        entries.sort_by(|a, b| {
            a.file_name()
                .to_string_lossy()
                .to_string()
                .cmp(&b.file_name().to_string_lossy().to_string())
        });

        if cli.paths.len() > 1 {
            println!("{}:", path.display());
        }

        formatting::print_entries(&entries)?;

        if index != cli.paths.len() - 1 {
            println!(); // Separate output with newlines for multiple paths
        }
    }

    Ok(())
}
