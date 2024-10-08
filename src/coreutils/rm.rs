use clap::Parser;
use std::io::{self, Write};
use std::path::{Path, MAIN_SEPARATOR};
use std::{fs, process};

#[derive(Parser, Debug)]
struct Args {
    #[clap(short, long)]
    interactive: bool,
    #[clap(short, long)]
    force: bool,
    #[clap(short = 'r', long)]
    recursive: bool,
    #[clap(long)]
    no_preserve_root: bool,
    #[clap(long)]
    verbose: bool,
    paths: Vec<String>,
}

pub fn rm(args: impl Iterator<Item = String>) -> io::Result<()> {
    let args = Args::parse_from(args);

    let mut status = 0;
    for file in &args.paths {
        if let Err(e) = remove_file(file, &args) {
            eprintln!("rm: cannot remove '{}': {}", file, e);
            status = 1;
        }
    }

    process::exit(status);
}

fn remove_file(path: &str, args: &Args) -> io::Result<()> {
    let path_obj = Path::new(path);

    let metadata = if path_obj.is_symlink() {
        path_obj.symlink_metadata()?
    } else {
        path_obj.metadata()?
    };

    if metadata.is_dir() && args.recursive {
        if !args.no_preserve_root && path_obj == Path::new(&MAIN_SEPARATOR.to_string()) {
            eprintln!("rm: it is dangerous to operate recursively on '/'");
            eprintln!("rm: use --no-preserve-root to override this failsafe");
            process::exit(1);
        }

        if args.interactive {
            print!("rm: remove directory '{}' and its contents? ", path);
            io::stdout().flush()?;
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            if !input.trim().eq_ignore_ascii_case("y") {
                return Ok(());
            }
        }

        fs::remove_dir_all(path_obj)?;
        if args.verbose {
            println!("removed directory: '{}'", path);
        }
    } else {
        if args.interactive {
            print!("rm: remove '{}'? ", path);
            io::stdout().flush()?;
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            if !input.trim().eq_ignore_ascii_case("y") {
                return Ok(());
            }
        }

        fs::remove_file(path_obj)?;
        if args.verbose {
            println!("removed '{}'", path);
        }
    }

    Ok(())
}
