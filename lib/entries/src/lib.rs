use config::expand_user;
use parser::parse_file;
use std::{fs::read_dir, path::PathBuf, process::Command};

mod errors;
mod parse;
pub use errors::Error;
use parse::EntryBuilder;

pub const ICON_DIRS: [&str; 2] = ["/usr/share/pixmaps", "/usr/share/icons"];

/// A Menu Entry
#[derive(Debug)]
pub struct MenuEntry {
    /// Ttile shown in the ui
    pub title: String,
    /// Command to launch when selected
    pub launch: String,
    /// Arguments for the launch command
    pub args: Vec<String>,
    /// Icon path to show in the ui
    pub icon: Option<PathBuf>,
}

impl MenuEntry {
    /// Load entry from a given file
    /// # Errors
    /// Returns an error if the file could not be read or if the contents could not be parsed
    pub fn from_file(path: PathBuf) -> Result<Self, Error> {
        parse_file::<EntryBuilder>(path)
    }

    /// Try to load menu entries from given directories
    /// # Errors
    /// Returns an error if none of the directories could be loaded
    pub fn load_dirs(dirs: &[&str]) -> Result<Vec<Self>, Error> {
        let mut errs = vec![];
        for dir in dirs {
            let dir_path = expand_user(dir)?;
            let res = Self::load_dir(&dir_path);
            match res {
                Ok(slfs) => return Ok(slfs),
                Err(err) => errs.push((dir_path, err)),
            }
        }
        Err(Error::NoEntriesFound { prev_errors: errs })
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
