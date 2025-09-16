use parser::parse_file;
use std::{path::PathBuf, process::Command};

mod errors;
pub use errors::Error;

pub struct MenuEntry {
    pub title: String,
    pub launch: String,
}

impl MenuEntry {
    pub fn from_file(path: PathBuf) -> Result<MenuEntry, Error> {
        let mut contents = parse_file(path)?;
        let title = contents.remove_key("title")?;
        let launch = contents.remove_key("launch")?;
        Ok(MenuEntry { title, launch })
    }

    pub fn launch_command(&self) -> Command {
        Command::new(&self.launch)
    }
}
