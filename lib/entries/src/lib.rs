use parser::parse_file;
use std::path::PathBuf;

mod errors;
mod parse;
pub use errors::Error;
use parse::EntryBuilder;

pub struct MenuEntry {
    pub title: String,
    pub launch: String,
}

impl MenuEntry {
    pub fn from_file(path: PathBuf) -> Result<MenuEntry, Error> {
        parse_file::<EntryBuilder>(path)
    }
}
