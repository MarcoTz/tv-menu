use std::{fs::read_to_string, path::PathBuf};

mod errors;
pub use errors::Error;

pub struct MenuEntry {
    pub title: String,
    pub launch: String,
}

impl MenuEntry {
    pub fn from_file(path: PathBuf) -> Result<MenuEntry, Error> {
        let file_contents = read_to_string(&path).map_err(|err| Error::read_file(err, &path))?;
        let mut title = None;
        let mut launch = None;
        for (nr, line) in file_contents.lines().enumerate() {
            let (key, val) = line.split_once("=").ok_or(Error::format(
                &path,
                nr,
                "Entries need to be in key=value format",
            ))?;
            match key.to_ascii_lowercase().trim() {
                "title" => {
                    title = Some(val.trim().to_owned());
                }
                "launch" => {
                    launch = Some(val.trim().to_owned());
                }
                _ => return Err(Error::unknown_key(&path, nr, key)),
            }
        }
        let title = title.ok_or(Error::missing_key(&path, "title"))?;
        let launch = launch.ok_or(Error::missing_key(&path, "key"))?;
        Ok(MenuEntry { title, launch })
    }
}
