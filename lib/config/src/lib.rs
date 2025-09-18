use parser::parse_file;
use std::path::PathBuf;

mod colors;
mod errors;
mod parse;
pub use colors::Color;
pub use errors::Error;
use parse::AppConfigBuilder;

pub struct EntryConfig {
    pub background: Color,
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
    pub columns: Option<u64>,
    pub padding: f32,
    pub height: f32,
    pub width: f32,
    pub column_gap: f32,
    pub row_gap: f32,
}

impl AppConfig {
    pub fn from_file(path: PathBuf) -> Result<AppConfig, Error> {
        parse_file::<AppConfigBuilder>(path)
    }
}
