use eframe::egui::Color32;
use parser::parse_file;
use std::path::PathBuf;

mod colors;
mod errors;
use colors::parse_color;
pub use errors::Error;

pub struct AppConfig {
    pub entry_background: Color32,
    pub entry_text_color: Color32,
    pub entry_radius: u8,
    pub entry_padding: i8,
    pub entry_text_size: f32,
}

impl AppConfig {
    pub fn from_file(path: PathBuf) -> Result<AppConfig, Error> {
        let mut contents = parse_file(path)?;
        let entry_background = parse_color(contents.remove_key("entry-background")?)?;
        let entry_text_color = parse_color(contents.remove_key("entry-text-color")?)?;
        let radius_str = contents
            .remove_key("entry-radius")
            .unwrap_or("0".to_owned());
        let entry_radius = radius_str
            .parse::<u8>()
            .map_err(|_| Error::InvalidNumber(radius_str))?;
        let padding_str = contents
            .remove_key("entry-padding")
            .unwrap_or("0".to_owned());
        let entry_padding = padding_str
            .parse::<i8>()
            .map_err(|_| Error::InvalidNumber(padding_str))?;
        let entry_text_size_str = contents
            .remove_key("entry-text-size")
            .unwrap_or("12.0".to_owned());
        let entry_text_size = entry_text_size_str
            .parse::<f32>()
            .map_err(|_| Error::InvalidNumber(entry_text_size_str))?;
        Ok(AppConfig {
            entry_background,
            entry_text_color,
            entry_radius,
            entry_padding,
            entry_text_size,
        })
    }
}
