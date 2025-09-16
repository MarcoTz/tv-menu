use eframe::egui::Color32;
use parser::parse_file;
use std::path::PathBuf;

mod colors;
mod errors;
mod parse;
pub use errors::Error;
use parse::AppConfigBuilder;

pub struct AppConfig {
    pub entry_background: Color32,
    pub entry_text_color: Color32,
    pub entry_radius: u8,
    pub entry_text_size: f32,
    pub entry_width: Option<f32>,
    pub entry_height: Option<f32>,
}

impl AppConfig {
    pub fn from_file(path: PathBuf) -> Result<AppConfig, Error> {
        parse_file::<AppConfigBuilder>(path)
    }
}
