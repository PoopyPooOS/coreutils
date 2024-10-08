use clap::Parser;
use ipc_init::Init;
use nix::sys::reboot::{reboot as linux_reboot, RebootMode};
use std::{
    fmt::Display,
    io::{self, Write},
};

#[derive(Debug, Parser)]
pub struct Cli {
    #[arg(short, long)]
    force: bool,
}

pub fn reboot(args: impl Iterator<Item = String>) -> io::Result<()> {
    let cli = Cli::parse_from(args);

    if cli.force {
        if confirm("Are you sure you want to force reboot?", false) {
            linux_reboot(RebootMode::RB_AUTOBOOT).expect("Failed to reboot system");
        }
    } else {
        Init::new("/tmp/ipc/init.sock").reboot();
    }

    Ok(())
}

fn confirm(prompt: impl Display, default: bool) -> bool {
    loop {
        print!("{} [{}]: ", prompt, if default { "Y/n" } else { "y/N" });
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim().to_lowercase().as_str() {
            "y" | "yes" => return true,
            "n" | "no" => return false,
            "" => return default,
            _ => println!("Please enter 'y' or 'n'."),
        }
    }
}
