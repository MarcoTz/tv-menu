use std::{collections::HashMap, fs::read_to_string, path::PathBuf};

mod errors;
pub use errors::Error;

pub struct ConfigContents {
    values: HashMap<String, String>,
    path: PathBuf,
}

impl ConfigContents {
    pub fn remove_key(&mut self, key: &str) -> Result<String, Error> {
        self.values
            .remove(key)
            .ok_or(Error::missing_key(&self.path, key))
    }
}

pub fn parse_file(path: PathBuf) -> Result<ConfigContents, Error> {
    let path_contents = read_to_string(&path).map_err(|err| Error::read_file(err, &path))?;
    parse_string(path_contents, path)
}

pub fn parse_string(input: String, path: PathBuf) -> Result<ConfigContents, Error> {
    let mut contents = HashMap::new();
    for (num, line) in input.lines().enumerate() {
        let (key, val) = line.split_once("=").ok_or(Error::format(
            &path,
            num,
            "Entries need to be in key=value format",
        ))?;
        contents.insert(key.trim().to_owned(), val.trim().to_owned());
    }
    Ok(ConfigContents {
        values: contents,
        path,
    })
}
