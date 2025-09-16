use crate::{AppConfig, EntryConfig, Error, colors::parse_color};
use eframe::egui::Color32;
use parser::{ConfigBuilder, Key, Section};

#[derive(Default)]
pub struct AppConfigBuilder {
    entry_background: Option<Color32>,
    text_color: Option<Color32>,
    border_radius: Option<u8>,
    text_size: Option<f32>,
    width: Option<f32>,
    height: Option<f32>,
    background: Option<Color32>,
}

impl ConfigBuilder for AppConfigBuilder {
    type Output = AppConfig;
    type Error = Error;

    fn sections() -> Vec<Section> {
        vec![Section::empty(), Section::new("Entries", true)]
    }

    fn section_keys(section: &str) -> Result<Vec<Key>, Self::Error> {
        match section {
            "" => Ok(vec![Key::new("background", true)]),
            "Entries" => Ok(vec![
                Key::new("background", true),
                Key::new("text-color", true),
                Key::new("border-radius", true),
                Key::new("text-size", true),
                Key::new("width", true),
                Key::new("height", true),
            ]),
            _ => Err(Error::InvalidSection(section.to_owned())),
        }
    }

    fn parse_value(&mut self, section: &str, key: &str, value: &str) -> Result<(), Self::Error> {
        match (section, key) {
            ("", "background") => self.background = Some(parse_color(value)?),
            ("Entries", "background") => self.entry_background = Some(parse_color(value)?),
            ("Entries", "text-color") => self.text_color = Some(parse_color(value)?),
            ("Entries", "border-radius") => {
                self.border_radius = Some(
                    value
                        .parse::<u8>()
                        .map_err(|_| Error::InvalidNumber(value.to_owned()))?,
                )
            }
            ("Entries", "text-size") => {
                self.text_size = Some(
                    value
                        .parse::<f32>()
                        .map_err(|_| Error::InvalidNumber(value.to_owned()))?,
                )
            }
            ("Entries", "width") => {
                self.width = Some(
                    value
                        .parse::<f32>()
                        .map_err(|_| Error::InvalidNumber(value.to_owned()))?,
                )
            }
            ("Entries", "height") => {
                self.height = Some(
                    value
                        .parse::<f32>()
                        .map_err(|_| Error::InvalidNumber(value.to_owned()))?,
                )
            }
            _ => return Err(Error::invalid_key(section, key)),
        }
        Ok(())
    }

    fn build(self) -> Self::Output {
        AppConfig {
            background: self.background.unwrap_or(Color32::BLACK),
            entries: EntryConfig {
                background: self.entry_background.unwrap_or(Color32::TRANSPARENT),
                text_color: self.text_color.unwrap_or(Color32::BLACK),
                border_radius: self.border_radius.unwrap_or(0),
                text_size: self.text_size.unwrap_or(12.0),
                width: self.width,
                height: self.height,
            },
        }
    }
}
