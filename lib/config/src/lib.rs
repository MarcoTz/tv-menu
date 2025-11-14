use parser::parse_file;
use std::{env::home_dir, path::PathBuf};

mod colors;
mod errors;
mod parse;
pub use colors::Color;
pub use errors::Error;
use parse::AppConfigBuilder;

pub struct EntryConfig {
    pub background: Color,
    pub background_active: Color,
    pub text_color: Color,
    pub text_size: f32,
    pub border_radius: f32,
    pub width: f32,
    pub height: f32,
}

pub struct AppConfig {
    pub entries: EntryConfig,
    pub background: Color,
    pub text_color: Color,
    pub text_size: f32,
    pub columns: Option<u64>,
    pub padding: f32,
    pub height: f32,
    pub width: f32,
    pub column_gap: f32,
    pub row_gap: f32,
}

impl AppConfig {
    /// Load App Config from files
    /// possible locations
    pub fn load(config_paths: &[&str]) -> Result<AppConfig, Error> {
        for path_name in config_paths {
            let mut path = PathBuf::from(path_name);
            if path_name.starts_with("~") {
                path = path
                    .strip_prefix("~")
                    .map_err(|err| Error::HomeDir {
                        path: path_name.to_string(),
                        msg: err.to_string(),
                    })?
                    .to_path_buf();
                let home_dir = home_dir().ok_or(Error::HomeDir {
                    path: path_name.to_string(),
                    msg: "Could not get home directory".to_owned(),
                })?;
                path = home_dir.join(path);
            }
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

    pub fn from_file(path: PathBuf) -> Result<AppConfig, Error> {
        parse_file::<AppConfigBuilder>(path)
    }
}
