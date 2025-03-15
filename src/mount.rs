use clap::Parser;
use rustix::mount::{self, MountFlags};
use std::{ffi::CString, io, path::PathBuf};

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
        &args.source,
        &args.target,
        CString::new(args.fs_type.unwrap_or(String::new()))
            .unwrap()
            .as_ref(),
        MountFlags::empty(),
        None,
    )
    .expect("Failed to mount filesystem");

    Ok(())
}
