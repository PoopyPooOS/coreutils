use clap::{arg, Parser};
use colored::Colorize;
use std::{fs, path::PathBuf};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    line_numbers: bool,
    #[arg(required = true)]
    paths: Vec<PathBuf>,
}

fn main() {
    let Cli { line_numbers, paths } = Cli::parse();

    let multiple = paths.len() > 1;
    for (index, path) in paths.iter().enumerate() {
        let content = fs::read_to_string(path).expect("Failed to read file");

        if multiple {
            if index != 0 {
                println!();
            }

            println!("{}:", path.display().to_string().bold().bright_white());
        }

        if !line_numbers {
            println!("{}", content.trim_end_matches('\n'));
        } else {
            for (index, line) in content.lines().enumerate() {
                println!("{} {}", index + 1, line);
            }
        }
    }
}
