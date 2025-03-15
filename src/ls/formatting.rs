use colored::{ColoredString, Colorize};
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::{fs, io};
use terminal_size::{terminal_size, Width};

pub fn print_entries(entries: &[fs::DirEntry]) -> io::Result<()> {
    let size = terminal_size();
    let width = if let Some((Width(w), _)) = size {
        w as usize
    } else {
        80
    };

    let mut max_len = 0;
    for entry in entries {
        let len = entry.file_name().to_string_lossy().len();
        if len > max_len {
            max_len = len;
        }
    }

    let max_len = max_len + 2;
    let cols = width / max_len;

    let multiline = entries
        .iter()
        .map(|entry| entry.file_name().to_string_lossy().to_string())
        .collect::<Vec<_>>()
        .join("  ")
        .len()
        >= width;

    for (i, entry) in entries.iter().enumerate() {
        if i > 0 && i % cols == 0 {
            println!();
        }

        let colored_name = colorize_entry(entry);

        if multiline {
            print!("{colored_name:<max_len$}");
        } else {
            print!("{colored_name}  ");
        }
    }
    println!();

    Ok(())
}

pub fn colorize_entry(entry: &fs::DirEntry) -> ColoredString {
    let file_name = entry
        .file_name()
        .into_string()
        .expect("Failed to convert file name to string");

    match entry.file_type() {
        Ok(file_type) if file_type.is_dir() => file_name.blue().bold(),
        Ok(file_type) if file_type.is_symlink() => file_name.cyan().bold(),
        Ok(_) => {
            if is_executable(entry.path()) {
                file_name.bright_green().bold()
            } else {
                file_name.normal()
            }
        }
        Err(_) => file_name.red().bold(),
    }
}

fn is_executable(file_path: PathBuf) -> bool {
    if let Ok(metadata) = fs::metadata(file_path) {
        metadata.permissions().mode() & 0o111 != 0
    } else {
        false
    }
}
