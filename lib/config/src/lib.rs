use eframe::egui::Color32;
use parser::parse_file;
use std::path::PathBuf;

mod colors;
mod errors;
mod parse;
pub use errors::Error;
use parse::AppConfigBuilder;

pub struct EntryConfig {
    pub background: Color32,
    pub text_color: Color32,
    pub text_size: f32,
    pub border_radius: u8,
    pub width: Option<f32>,
    pub height: Option<f32>,
}

pub struct AppConfig {
    pub entries: EntryConfig,
    pub background: Color32,
    pub padding: i8,
}

impl AppConfig {
    pub fn from_file(path: PathBuf) -> Result<AppConfig, Error> {
        parse_file::<AppConfigBuilder>(path)
    }
}
