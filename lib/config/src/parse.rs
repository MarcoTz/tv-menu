use crate::{AppConfig, Error, colors::parse_color};
use eframe::egui::Color32;
use parser::{ConfigBuilder, Key, Section};

#[derive(Default)]
pub struct AppConfigBuilder {
    background: Option<Color32>,
    text_color: Option<Color32>,
    border_radius: Option<u8>,
    text_size: Option<f32>,
    width: Option<f32>,
    height: Option<f32>,
}

impl ConfigBuilder for AppConfigBuilder {
    type Output = AppConfig;
    type Error = Error;

    fn sections() -> Vec<Section> {
        vec![Section::new("Entries", true)]
    }

    fn section_keys(section: &str) -> Result<Vec<Key>, Self::Error> {
        if section != "Entries" {
            return Err(Error::InvalidSection(section.to_owned()));
        }
        Ok(vec![
            Key::new("background", true),
            Key::new("text-color", true),
            Key::new("border-radius", true),
            Key::new("text-size", true),
            Key::new("width", true),
            Key::new("height", true),
        ])
    }

    fn parse_value(&mut self, section: &str, key: &str, value: &str) -> Result<(), Self::Error> {
        if section != "Entries" {
            return Err(Error::InvalidSection(section.to_owned()));
        }
        match key {
            "background" => self.background = Some(parse_color(value)?),
            "text-color" => self.text_color = Some(parse_color(value)?),
            "border-radius" => {
                self.border_radius = Some(
                    value
                        .parse::<u8>()
                        .map_err(|_| Error::InvalidNumber(value.to_owned()))?,
                )
            }
            "text-size" => {
                self.text_size = Some(
                    value
                        .parse::<f32>()
                        .map_err(|_| Error::InvalidNumber(value.to_owned()))?,
                )
            }
            "width" => {
                self.width = Some(
                    value
                        .parse::<f32>()
                        .map_err(|_| Error::InvalidNumber(value.to_owned()))?,
                )
            }
            "height" => {
                self.height = Some(
                    value
                        .parse::<f32>()
                        .map_err(|_| Error::InvalidNumber(value.to_owned()))?,
                )
            }
            _ => return Err(Error::invalid_key(section, key)),
        };
        Ok(())
    }

    fn build(self) -> Self::Output {
        AppConfig {
            entry_background: self.background.unwrap_or(Color32::TRANSPARENT),
            entry_text_color: self.text_color.unwrap_or(Color32::BLACK),
            entry_radius: self.border_radius.unwrap_or(0),
            entry_text_size: self.text_size.unwrap_or(12.0),
            entry_width: self.width,
            entry_height: self.height,
        }
    }
}
