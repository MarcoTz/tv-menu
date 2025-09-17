use config::AppConfig;
use entries::MenuEntry;
use iced::{Color, widget::Row};
use std::path::PathBuf;

mod errors;
mod menu_widget;
pub use errors::Error;
use menu_widget::EntryWidget;

pub const ENTRY_PATH: &str = "entries";
pub const CONFIG_PATH: &str = "tvmenu.conf";

#[derive(Debug, Clone)]
pub enum Message {
    Launch(String),
}
pub struct MenuState {
    pub config: AppConfig,
    entries: Vec<MenuEntry>,
}

impl MenuState {
    pub fn init() -> Result<MenuState, Error> {
        let entry_path = PathBuf::from(ENTRY_PATH);
        let config_path = PathBuf::from(CONFIG_PATH);

        let entries = MenuEntry::load_dir(entry_path)?;
        let config = AppConfig::from_file(config_path)?;
        Ok(MenuState { config, entries })
    }

    pub fn view(&self) -> Row<'_, Message> {
        let mut entry_elements = vec![];
        for entry in self.entries.iter() {
            let widget = EntryWidget::new(entry, &self.config);
            let button = widget.view();
            entry_elements.push(button.into());
        }
        Row::from_vec(entry_elements).padding(self.config.padding)
    }
}

pub fn to_color(color: &config::Color) -> Color {
    Color::from_rgba(
        color.red as f32 / 255.0,
        color.green as f32 / 255.0,
        color.blue as f32 / 255.0,
        color.alpha as f32 / 255.0,
    )
}
