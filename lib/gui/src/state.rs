use crate::{CONFIG_PATH, ENTRY_PATH, EntryWidget, Error};
use config::AppConfig;
use entries::MenuEntry;
use iced::widget::{Column, Row};
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

    pub fn view(&self) -> Column<'_, Message> {
        let widgets_per_col = if let Some(cols) = self.config.columns {
            cols as f32
        } else {
            (self.window_size.0 - self.config.padding)
                / (self.config.entries.width + self.config.column_gap)
        };
        let mut rows = vec![];
        let mut current_row = Row::new()
            .padding(self.config.padding)
            .spacing(self.config.column_gap);
        let mut num_elements = 0;
        for entry in self.entries.iter() {
            let button = EntryWidget::new(entry, &self.config).view();
            current_row = current_row.push(button);
            num_elements += 1;
            if num_elements >= widgets_per_col.floor() as u64 {
                rows.push(current_row.into());
                num_elements = 0;
                current_row = Row::new()
                    .padding(self.config.padding)
                    .spacing(self.config.column_gap);
            }
        }
        if num_elements != 0 {
            rows.push(current_row.into());
        }
        Column::from_vec(rows)
            .padding(self.config.padding)
            .spacing(self.config.row_gap)
    }
}
