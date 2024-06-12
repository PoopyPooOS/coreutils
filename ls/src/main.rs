use std::{env, fs, io};

mod formatting;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();
    let paths = if args.is_empty() {
        vec![".".to_string()]
    } else {
        args
    };

    let one_path = paths.len() == 1;

    for (index, path) in paths.iter().enumerate() {
        if !one_path {
            println!("{}:", path);
        }

        match ls(path) {
            Ok(dir_entries) => {
                let filtered_entries: Vec<fs::DirEntry> = dir_entries
                    .into_iter()
                    .filter(|entry| !entry.file_name().to_string_lossy().starts_with('.'))
                    .collect();

                if !filtered_entries.is_empty() {
                    formatting::print_entries(&filtered_entries)?;
                }
            }
            Err(e) => eprintln!("Error reading directory {}: {}", path, e),
        }

        if index != paths.len() - 1 {
            println!();
        }
    }

    Ok(())
}

fn ls(dir: &str) -> io::Result<Vec<fs::DirEntry>> {
    let paths = fs::read_dir(dir)?;
    let mut entries = Vec::new();

    for path in paths {
        match path {
            Ok(entry) => entries.push(entry),
            Err(e) => eprintln!("Error reading entry in directory {}: {}", dir, e),
        }
    }

    Ok(entries)
}
