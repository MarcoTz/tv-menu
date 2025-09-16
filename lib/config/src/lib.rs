use eframe::egui::Color32;
use parser::parse_file;
use std::path::PathBuf;

mod colors;
mod errors;
use colors::parse_color;
pub use errors::Error;

pub struct AppConfig {
    entry_background: Color32,
}

impl AppConfig {
    pub fn from_file(path: PathBuf) -> Result<AppConfig, Error> {
        let mut contents = parse_file(path)?;
        let entry_bg = contents.remove_key("entry-background")?;
        let entry_background = parse_color(entry_bg)?;
        Ok(AppConfig { entry_background })
    }
}
