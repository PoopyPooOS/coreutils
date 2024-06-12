mod uid;

use clap::Parser;
use ipc_userd::{User, Userd};
use rpassword::prompt_password;
use std::{env, path::PathBuf};

#[derive(Debug, Parser)]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Parser)]
enum Command {
    /// Add a user
    Add { username: String, shell: Option<PathBuf> },
}

fn main() {
    let cli = Cli::parse();
    let mut userd = Userd::new("/tmp/init/services/userd.sock");

    match cli.command {
        Command::Add { username, shell } => {
            let uid = uid::get_new_uid(&mut userd).expect("Failed to get a new uid, please edit the /etc/users.toml file manually.");
            let password: Option<String> = if let Ok(password) = env::var("USER_ADD_PASSWORD") {
                Some(password)
            } else {
                let password = prompt_password("User password (leave empty for none): ").expect("Failed to read password");

                if password.is_empty() {
                    None
                } else {
                    Some(password)
                }
            };

            let shell = if let Some(shell) = shell {
                shell
            } else {
                PathBuf::from("/sbin/shell")
            };
            let home = PathBuf::from("/home").join(&username);

            userd.add_user(User {
                uid,
                username,
                display_name: None,
                password,
                shell,
                home,
            })
        }
    }
    .expect("Failed to run command");
}
