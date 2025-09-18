use crate::{
    AppConfig, EntryConfig, Error,
    colors::{Color, parse_color},
};
use parser::{ConfigBuilder, Key, Section};

#[derive(Default)]
pub struct AppConfigBuilder {
    entry_background: Option<Color>,
    entry_text_color: Option<Color>,
    entry_border_radius: Option<f32>,
    entry_text_size: Option<f32>,
    entry_width: Option<f32>,
    entry_height: Option<f32>,
    columns: Option<u64>,
    padding: Option<f32>,
    background: Option<Color>,
    text_color: Option<Color>,
    height: Option<f32>,
    width: Option<f32>,
    column_gap: Option<f32>,
    row_gap: Option<f32>,
}

impl ConfigBuilder for AppConfigBuilder {
    type Output = AppConfig;
    type Error = Error;

    fn sections() -> Vec<Section> {
        vec![Section::empty(), Section::new("Entries", true)]
    }

    fn section_keys(section: &str) -> Result<Vec<Key>, Self::Error> {
        match section {
            "" => Ok(vec![
                Key::new("background", true),
                Key::new("padding", true),
                Key::new("text-color", true),
                Key::new("columns", true),
                Key::new("height", true),
                Key::new("width", true),
                Key::new("column-gap", true),
                Key::new("row-gap", true),
            ]),
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
            ("", "padding") => {
                self.padding = Some(
                    value
                        .parse::<f32>()
                        .map_err(|_| Error::InvalidNumber(value.to_owned()))?,
                )
            }
            ("", "text-color") => self.text_color = Some(parse_color(value)?),
            ("", "height") => {
                self.height = Some(
                    value
                        .parse::<f32>()
                        .map_err(|_| Error::InvalidNumber(value.to_owned()))?,
                )
            }
            ("", "width") => {
                self.width = Some(
                    value
                        .parse::<f32>()
                        .map_err(|_| Error::InvalidNumber(value.to_owned()))?,
                )
            }
            ("", "columns") => {
                self.columns = Some(
                    value
                        .parse::<u64>()
                        .map_err(|_| Error::InvalidNumber(value.to_owned()))?,
                )
            }
            ("", "row-gap") => {
                self.row_gap = Some(
                    value
                        .parse::<f32>()
                        .map_err(|_| Error::InvalidNumber(value.to_owned()))?,
                )
            }
            ("", "column-gap") => {
                self.column_gap = Some(
                    value
                        .parse::<f32>()
                        .map_err(|_| Error::InvalidNumber(value.to_owned()))?,
                )
            }
            ("Entries", "background") => self.entry_background = Some(parse_color(value)?),
            ("Entries", "text-color") => self.entry_text_color = Some(parse_color(value)?),
            ("Entries", "border-radius") => {
                self.entry_border_radius = Some(
                    value
                        .parse::<f32>()
                        .map_err(|_| Error::InvalidNumber(value.to_owned()))?,
                )
            }
            ("Entries", "text-size") => {
                self.entry_text_size = Some(
                    value
                        .parse::<f32>()
                        .map_err(|_| Error::InvalidNumber(value.to_owned()))?,
                )
            }
            ("Entries", "width") => {
                self.entry_width = Some(
                    value
                        .parse::<f32>()
                        .map_err(|_| Error::InvalidNumber(value.to_owned()))?,
                )
            }
            ("Entries", "height") => {
                self.entry_height = Some(
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
            background: self.background.unwrap_or(Color::BLACK),
            text_color: self.text_color.unwrap_or(Color::WHITE),
            padding: self.padding.unwrap_or(0.0),
            columns: self.columns,
            height: self.height.unwrap_or(0.0),
            width: self.width.unwrap_or(0.0),
            column_gap: self.column_gap.unwrap_or(10.0),
            row_gap: self.row_gap.unwrap_or(10.0),
            entries: EntryConfig {
                background: self.entry_background.unwrap_or(Color::TRANSPARENT),
                text_color: self.entry_text_color.unwrap_or(Color::BLACK),
                border_radius: self.entry_border_radius.unwrap_or(0.0),
                text_size: self.entry_text_size.unwrap_or(12.0),
                width: self.entry_width.unwrap_or(100.0),
                height: self.entry_height.unwrap_or(100.0),
            },
        }
    }
}
