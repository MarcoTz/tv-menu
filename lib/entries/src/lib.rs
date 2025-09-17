use parser::parse_file;
use std::{fs::read_dir, path::PathBuf};

mod errors;
mod parse;
pub use errors::Error;
use parse::EntryBuilder;

pub const ICON_DIRS: [&str; 2] = ["/usr/share/pixmaps", "/usr/share/icons"];

#[derive(Debug)]
pub struct MenuEntry {
    pub title: String,
    pub launch: String,
    pub icon: Option<PathBuf>,
}

impl MenuEntry {
    pub fn from_file(path: PathBuf) -> Result<MenuEntry, Error> {
        parse_file::<EntryBuilder>(path)
    }

    pub fn load_dir(path: PathBuf) -> Result<Vec<MenuEntry>, Error> {
        let mut entries = vec![];
        for path_entry in read_dir(&path).map_err(|err| Error::read_dir(err, &path))? {
            let path_entry = path_entry.map_err(|err| Error::read_dir(err, &path))?;
            let menu_entry = MenuEntry::from_file(path_entry.path())?;
            entries.push(menu_entry);
        }
        Ok(entries)
    }
}
