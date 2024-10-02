use ipc_init::Init;
use nix::sys::reboot::{reboot, RebootMode};
use std::{
    env,
    fmt::Display,
    io::{self, Write},
};

fn main() {
    let args = env::args().collect::<Vec<String>>();

    if args.len() > 1 && args[1] == "-f" {
        if confirm("Are you sure you want to force power off?", false) {
            reboot(RebootMode::RB_POWER_OFF).expect("Failed to power off system");
        }
    } else {
        Init::new("/tmp/ipc/init.sock").poweroff();
    }
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
