// Enum used to parse input arguments.  Ended up rolling my own arg parser instead of using an
// existing crate because I wanted `hp` commands to be more natural language-like and use dynamic
// positional commands

use crate::Hopper;
use colored::Colorize;
use std::{
    env,
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub enum Rabbit {
    Dir(String, PathBuf),
    File(String, PathBuf),
    RequestName(String),
    RequestPath(PathBuf),
}

impl Rabbit {
    pub fn from<T: AsRef<Path>>(input: T, name: Option<String>) -> Self {
        let current_name = match name {
            Some(given_name) => given_name,
            None => input
                .as_ref()
                .file_name()
                .expect("[error] Unable to disambiguate file/directory.")
                .to_str()
                .expect("[error] Unable to convert file/directory name to UTF-8.")
                .to_string(),
        };
        if input.as_ref().is_dir() {
            Rabbit::Dir(current_name, input.as_ref().to_path_buf())
        } else {
            Rabbit::File(current_name, input.as_ref().to_path_buf())
        }
    }

    pub fn request(input: String) -> Self {
        if PathBuf::from(&input).exists() {
            Rabbit::RequestPath(PathBuf::from(&input))
        } else {
            Rabbit::RequestName(input)
        }
    }
}

pub enum Cmd {
    Use(Rabbit),
    Remove(Rabbit),
    PrintMsg(String),
    SetBrb(PathBuf),
    BrbHop,
    ListHops,
    PrintHelp,
    Passthrough(String),
    LocateBunnyhop,
    LocateShortcut(String),
    Configure,
    HopDirAndEdit(String),
    EditDir(Rabbit),
    ShowHistory,
    PullHistory(Rabbit),
}

impl Cmd {
    pub fn parse() -> Self {
        let current_dir =
            env::current_dir().expect("[error] Unable to locate current working directory.");
        match env::args().nth(1) {
            Some(primary) => match primary.as_str() {
                "add" => match env::args().nth(2) {
                    Some(f_or_d) => {
                        let mut f_or_d_path = PathBuf::from(&current_dir);
                        f_or_d_path.push(&f_or_d);
                        if f_or_d_path.is_file() {
                            Cmd::Use(Rabbit::from(f_or_d_path, env::args().nth(3)))
                        } else {
                            Cmd::Use(Rabbit::from(&current_dir, Some(f_or_d)))
                        }
                    }
                    None => Cmd::Use(Rabbit::from(
                        env::current_dir()
                            .expect("[error] Unable to locate current working directory."),
                        None,
                    )),
                },
                "rm" | "remove" => match env::args().nth(2) {
                    Some(name) => Cmd::Remove(Rabbit::request(name)),
                    None => Cmd::Remove(Rabbit::request(current_dir.display().to_string())),
                },
                "ls" | "list" => Cmd::Passthrough("_ls".to_string()),
                "_ls" => Cmd::ListHops,
                "version" | "v" => Cmd::Passthrough("_version".to_string()),
                "_version" => Cmd::PrintMsg(format!(
                    "{} 🐇 {}{}",
                    "BunnyHop".cyan().bold(),
                    "v.".bold(),
                    env!("CARGO_PKG_VERSION").bright_white().bold()
                )),
                "brb" => Cmd::SetBrb(current_dir),
                "back" => Cmd::BrbHop,
                "help" => Cmd::Passthrough("_help".to_string()),
                "_help" => Cmd::PrintHelp,
                "config" | "configure" => Cmd::Configure,
                "edit" => match env::args().nth(2) {
                    Some(name) => Cmd::HopDirAndEdit(name),
                    None => Cmd::EditDir(Rabbit::from(current_dir, None)),
                },
                "locate" => match env::args().nth(2) {
                    Some(name) => Cmd::LocateShortcut(name),
                    None => Cmd::Passthrough("_list_all_history_hops".to_string()),
                },
                "_list_all_history_hops" => Cmd::LocateBunnyhop,
                "history" => match env::args().nth(2) {
                    Some(arg) => Cmd::Passthrough(format!("_history {}", arg)),
                    None => Cmd::Passthrough("_history".to_string()),
                },
                "_history" => match env::args().nth(2) {
                    Some(name) => Cmd::PullHistory(Rabbit::request(name)),
                    None => Cmd::ShowHistory,
                },
                whatevs => Cmd::Use(Rabbit::RequestName(whatevs.to_string())),
            },
            None => Cmd::PrintMsg("[error] Unable to parse current arguments.".to_string()),
        }
    }
}

impl Hopper {
    pub fn execute(&mut self, cmd: Cmd) -> anyhow::Result<()> {
        match cmd {
            Cmd::Passthrough(cmd) => self.runner(cmd),
            Cmd::Use(bunny) => self.just_do_it(bunny),
            Cmd::SetBrb(loc) => self.brb(loc),
            Cmd::BrbHop => self.use_hop("back".to_string()),
            Cmd::ListHops => self.list_hops(),
            Cmd::PrintHelp => Self::print_help(),
            Cmd::Remove(bunny) => self.remove_hop(bunny),
            Cmd::Configure => self.configure(),
            Cmd::LocateBunnyhop => self.show_locations(),
            Cmd::LocateShortcut(name) => self.print_hop(name),
            Cmd::HopDirAndEdit(name) => self.hop_to_and_open_dir(name),
            Cmd::EditDir(bunny) => self.edit_dir(bunny),
            Cmd::ShowHistory => self.show_history(),
            Cmd::PullHistory(bunny) => self.search_history(bunny),
            Cmd::PrintMsg(msg) => {
                println!("{}", msg);
                Ok(())
            }
        }
    }
}
