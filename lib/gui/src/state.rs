use crate::{CONFIG_PATH, ENTRY_PATH, EntryWidget, Error};
use config::AppConfig;
use entries::MenuEntry;
use iced::widget::Row;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum Message {
    Launch(String),
    Resized { width: f32, height: f32 },
}
pub struct MenuState {
    pub config: AppConfig,
    pub window_size: (f32, f32),
    entries: Vec<MenuEntry>,
}

impl MenuState {
    pub fn init(window_width: f32, window_height: f32) -> Result<MenuState, Error> {
        let entry_path = PathBuf::from(ENTRY_PATH);
        let config_path = PathBuf::from(CONFIG_PATH);

        let entries = MenuEntry::load_dir(entry_path)?;
        let config = AppConfig::from_file(config_path)?;
        Ok(MenuState {
            window_size: (window_width, window_height),
            config,
            entries,
        })
    }

    pub fn from_config(
        config: AppConfig,
        window_width: f32,
        window_height: f32,
    ) -> Result<MenuState, Error> {
        let entry_path = PathBuf::from(ENTRY_PATH);
        let entries = MenuEntry::load_dir(entry_path)?;
        Ok(MenuState {
            window_size: (window_width, window_height),
            config,
            entries,
        })
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
