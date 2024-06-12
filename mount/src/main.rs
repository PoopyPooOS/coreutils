use std::{
    ffi::CString,
    path::{Path, PathBuf},
};

use clap::Parser;
use nix::mount::{mount, MsFlags};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    source: PathBuf,
    target: PathBuf,
    #[arg(short = 't', long, value_name = "TYPE")]
    fs_type: Option<String>,
}

fn main() {
    let args = Args::parse();

    mount(
        Some(&args.source),
        &args.target,
        Some(
            CString::new(args.fs_type.unwrap_or("".to_string()))
                .unwrap()
                .as_ref(),
        ),
        MsFlags::empty(),
        None::<&Path>,
    )
    .expect("Failed to mount filesystem");
}
