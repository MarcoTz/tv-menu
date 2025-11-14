use std::{collections::HashMap, fs::read_to_string, path::PathBuf};

mod config_builder;
mod contents;
mod errors;
use config_builder::from_contents;
pub use config_builder::{ConfigBuilder, Key, Section};
use contents::{ConfigContents, ConfigSection};
pub use errors::Error;

/// Parse a given file with the given builder type
/// # Errors
/// Returns an error if the file cannot be read, or if the contents cannot be parsed
pub fn parse_file<Builder>(path: PathBuf) -> Result<Builder::Output, Builder::Error>
where
    Builder: ConfigBuilder,
{
    let contents = contents_from_file(path)?;
    from_contents::<Builder>(contents)
}

/// Parse file to generic [`ConfigContents`]
/// # Errors
/// Returns an error if the file could not be read or if the contents could not be parsed
fn contents_from_file(path: PathBuf) -> Result<ConfigContents, Error> {
    let path_contents = read_to_string(&path).map_err(|err| Error::read_file(&err, &path))?;
    contents_from_string(&path_contents, path)
}

/// Parse string to generic [`ConfigContents`]
/// # Errors
/// Returns an error if the contents could not be parsed
fn contents_from_string(input: &str, path: PathBuf) -> Result<ConfigContents, Error> {
    let mut sections = HashMap::new();
    let mut values = HashMap::new();
    let mut current_section = String::new();
    for (num, line) in input.lines().enumerate() {
        let line = remove_comment(line);
        if line.is_empty() {
            continue;
        }
        if line.starts_with('[') && line.ends_with(']') {
            if !values.is_empty() {
                sections.insert(current_section, ConfigSection { values });
            }
            values = HashMap::new();
            current_section = line.replace(['[', ']'], "");
            continue;
        }
        let (key, val) = line
            .split_once('=')
            .ok_or_else(|| Error::format(&path, num, "Entries need to be in key=value format"))?;
        values.insert(key.trim().to_owned(), val.trim().to_owned());
    }
    if !values.is_empty() {
        sections.insert(current_section, ConfigSection { values });
    }
    Ok(ConfigContents { sections, path })
}

/// Remove comment from a given line
fn remove_comment(line: &str) -> &str {
    match line.split_once("//") {
        Some((start, _)) => start,
        None => line,
    }
}
