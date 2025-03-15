use clap::{arg, Parser};
use colored::Colorize;
use std::{fs, io, path::PathBuf};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    line_numbers: bool,
    #[arg(required = true)]
    paths: Vec<PathBuf>,
}

pub fn cat(args: impl Iterator<Item = String>) -> io::Result<()> {
    let Cli {
        line_numbers,
        paths,
    } = Cli::parse_from(args);

    let multiple = paths.len() > 1;
    for (index, path) in paths.iter().enumerate() {
        let content = fs::read_to_string(path).expect("Failed to read file");

        if multiple {
            if index != 0 {
                println!();
            }

            println!(
                "{}{}",
                path.display().to_string().bold().bright_white(),
                ":".bold().bright_white()
            );
        }

        if line_numbers {
            let lines: Vec<&str> = content.lines().collect();
            let total_lines = lines.len();
            let line_numbers_width = (if line_numbers {
                total_lines.to_string().len()
            } else {
                0
            }) + 1;

            for (index, line) in content.lines().enumerate() {
                println!("{:<width$} {}", index + 1, line, width = line_numbers_width);
            }
        } else {
            println!("{}", content.trim_end_matches('\n'));
        }
    }

    Ok(())
}
