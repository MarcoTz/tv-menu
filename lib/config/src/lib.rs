use parser::parse_file;
use std::{env::home_dir, path::PathBuf};

mod colors;
mod errors;
mod parse;
pub use colors::Color;
pub use errors::Error;
use parse::AppConfigBuilder;

/// Configuration for entries shown in the window
pub struct EntryConfig {
    /// Background color
    pub background: Color,
    /// Background color when selected
    pub background_active: Color,
    /// Text color
    pub text_color: Color,
    /// font size
    pub text_size: f32,
    /// Border radius
    pub border_radius: f32,
    /// width of an entry
    pub width: f32,
    /// height of an entry
    pub height: f32,
}

/// Configuration for the App, loaded from a file
pub struct AppConfig {
    /// Configuration for entries
    pub entries: EntryConfig,
    /// Background color
    pub background: Color,
    /// Text Color
    pub text_color: Color,
    /// Font size
    pub text_size: f32,
    /// Number of columns to show
    /// calculated based on window size if `None`
    pub columns: Option<u64>,
    /// Padding from the window sides
    pub padding: f32,
    /// Window Height
    pub height: f32,
    /// Window Width
    pub width: f32,
    /// Gap between columns
    pub column_gap: f32,
    /// Gap between rows
    pub row_gap: f32,
}

impl AppConfig {
    /// Load App Config from given locations
    /// tries to load files in order
    /// # Errors
    /// Returns an error if none of the given paths could be loaded
    /// that is for each path it either does not exist or [`Self::from_file`] returned an error
    pub fn load(config_paths: &[&str]) -> Result<Self, Error> {
        for path_name in config_paths {
            let path = expand_user(path_name)?;

            if !path.exists() {
                continue;
            }
            let conf = Self::from_file(path);
            if conf.is_ok() {
                return conf;
            }
        }
        Err(Error::NoConfigFound(
            config_paths.iter().map(|path| (*path).to_owned()).collect(),
        ))
    }

    /// Parse config from a file
    /// # Errors
    /// Returns an error if the file could not be read or if the contents could not be parsed
    pub fn from_file(path: PathBuf) -> Result<Self, Error> {
        parse_file::<AppConfigBuilder>(path)
    }
}

/// Expand the user directory `~` in a given path name
/// Only expands `~` if the path starts with it, otherwise returns `PathBuf::from(path_name)`
/// # Errors
/// Returns an error if the `~` prefix could not be stripped
/// or the home directory could not be retrieved
pub fn expand_user(path_name: &str) -> Result<PathBuf, Error> {
    if path_name.starts_with('~') {
        let mut path = PathBuf::from(path_name)
            .strip_prefix("~")
            .map_err(|err| Error::HomeDir {
                path: (*path_name).to_string(),
                msg: err.to_string(),
            })?
            .to_path_buf();
        let home_dir = home_dir().ok_or_else(|| Error::HomeDir {
            path: (*path_name).to_string(),
            msg: "Could not get home directory".to_owned(),
        })?;
        path = home_dir.join(path);
        Ok(path)
    } else {
        Ok(PathBuf::from(path_name))
    }
}
