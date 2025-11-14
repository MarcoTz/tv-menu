use parser::parse_file;
use std::{fs::read_dir, path::PathBuf, process::Command};

mod errors;
mod parse;
pub use errors::Error;
use parse::EntryBuilder;

pub const ICON_DIRS: [&str; 2] = ["/usr/share/pixmaps", "/usr/share/icons"];

#[derive(Debug)]
pub struct MenuEntry {
    pub title: String,
    pub launch: String,
    pub args: Vec<String>,
    pub icon: Option<PathBuf>,
}

impl MenuEntry {
    /// Load entry from a given file
    /// # Errors
    /// Returns an error if the file could not be read or if the contents could not be parsed
    pub fn from_file(path: PathBuf) -> Result<Self, Error> {
        parse_file::<EntryBuilder>(path)
    }

    /// Load entries from a given directory
    /// # Errors
    /// Returns an error if files could not be read or if file contents could not be parsed
    pub fn load_dir(path: &PathBuf) -> Result<Vec<Self>, Error> {
        let mut entries = vec![];
        for path_entry in read_dir(path).map_err(|err| Error::read_dir(&err, path))? {
            let path_entry = path_entry.map_err(|err| Error::read_dir(&err, path))?;
            let menu_entry = Self::from_file(path_entry.path())?;
            entries.push(menu_entry);
        }
        Ok(entries)
    }
}

#[must_use]
pub fn launch_command(cmd: &str, args: &[String]) -> Command {
    let mut cmd = Command::new(cmd);
    for arg in args {
        cmd.arg(arg);
    }
    cmd
}
