use clap::Parser;
use nix::mount::{self, MsFlags};
use std::{
    ffi::CString,
    io,
    path::{Path, PathBuf},
};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    source: PathBuf,
    target: PathBuf,
    #[arg(short = 't', long, value_name = "TYPE")]
    fs_type: Option<String>,
}

pub fn mount(args: impl Iterator<Item = String>) -> io::Result<()> {
    let args = Args::parse_from(args);

    mount::mount(
        Some(&args.source),
        &args.target,
        Some(CString::new(args.fs_type.unwrap_or(String::new())).unwrap().as_ref()),
        MsFlags::empty(),
        None::<&Path>,
    )
    .expect("Failed to mount filesystem");

    Ok(())
}
