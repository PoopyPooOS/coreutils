use clap::Parser;
use logger::info;
use std::{
    fs,
    io::{self, Write},
    path::{MAIN_SEPARATOR, Path},
};

#[allow(clippy::struct_excessive_bools)]
#[derive(Parser, Debug)]
struct Args {
    #[clap(short, long)]
    interactive: bool,
    #[clap(short, long)]
    force: bool,
    #[clap(short = 'r', long)]
    recursive: bool,
    #[clap(long)]
    delete_root: bool,
    #[clap(long)]
    verbose: bool,

    paths: Vec<String>,
}

pub fn rm(args: impl Iterator<Item = String>) -> io::Result<()> {
    let args = Args::parse_from(args);

    for file in &args.paths {
        remove_file(file, &args).expect("Failed to remove file");
    }

    Ok(())
}

fn remove_file(path: &str, args: &Args) -> io::Result<()> {
    let path_obj = Path::new(path);

    let metadata = if path_obj.is_symlink() {
        path_obj.symlink_metadata()?
    } else {
        path_obj.metadata()?
    };

    if metadata.is_dir() && args.recursive {
        if !args.delete_root && path_obj == Path::new(&MAIN_SEPARATOR.to_string()) {
            panic!(
                "It is dangerous to operate recursively on root. Use `--delete-root` to ignore this warning"
            );
        }

        if args.interactive {
            print!("Remove directory '{path}' and its contents? ");
            io::stdout().flush()?;
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            if !input.trim().eq_ignore_ascii_case("y") {
                return Ok(());
            }
        }

        fs::remove_dir_all(path_obj).expect("Failed to remove directory");
        if args.verbose {
            info!("Removed directory: '{path}'");
        }
    } else {
        if args.interactive {
            print!("Remove '{path}'? ");
            io::stdout().flush()?;
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            if !input.trim().eq_ignore_ascii_case("y") {
                return Ok(());
            }
        }

        fs::remove_file(path_obj).expect("Failed to remove file");
        if args.verbose {
            info!("Removed '{path}'");
        }
    }

    Ok(())
}
