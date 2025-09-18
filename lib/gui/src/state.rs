use crate::{ENTRY_PATH, EntryWidget, Error};
use config::AppConfig;
use entries::MenuEntry;
use iced::{
    Length,
    keyboard::Key,
    widget::{
        Column, Row, Scrollable,
        scrollable::{Direction, Scrollbar},
    },
};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum Message {
    Launch(String),
    Resized { width: f32, height: f32 },
    KeyPress(Key),
}
pub struct MenuState {
    pub config: AppConfig,
    pub window_size: (f32, f32),
    pub entries: Vec<MenuEntry>,
    pub selected_index: usize,
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
            selected_index: 0,
        })
    }

    pub fn widgets_per_col(&self) -> u64 {
        if let Some(cols) = self.config.columns {
            cols
        } else {
            ((self.window_size.0 - self.config.padding)
                / (self.config.entries.width + self.config.column_gap))
                .floor() as u64
        }
    }

    pub fn view(&self) -> Scrollable<'_, Message> {
        let widgets_per_col = self.widgets_per_col();
        let mut rows = vec![];
        let mut current_row = Row::new()
            .padding(self.config.padding)
            .spacing(self.config.column_gap);
        let mut num_elements = 0;
        for (ind, entry) in self.entries.iter().enumerate() {
            let button = EntryWidget::new(entry, &self.config, ind == self.selected_index).view();
            current_row = current_row.push(button);
            num_elements += 1;
            if num_elements >= widgets_per_col {
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
        Scrollable::new(
            Column::from_vec(rows)
                .padding(self.config.padding)
                .spacing(self.config.row_gap),
        )
        .direction(Direction::Both {
            vertical: Scrollbar::new(),
            horizontal: Scrollbar::new(),
        })
        .width(Length::Fill)
        .height(Length::Fill)
    }
}
