use crate::{ENTRY_PATH, EntryWidget, Error, to_color};
use config::AppConfig;
use entries::MenuEntry;
use iced::{
    Background, Border, Element, Length,
    alignment::Horizontal,
    keyboard::Key,
    widget::{
        Column, Container, Row, Scrollable, Text, TextInput,
        scrollable::{Direction, Scrollbar},
        text_input::Style,
    },
};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum Message {
    Launch(String),
    Resized { width: f32, height: f32 },
    KeyPress(Key),
    FilterChanged(String),
}
pub struct MenuState {
    pub config: AppConfig,
    pub window_size: (f32, f32),
    pub entries: Vec<MenuEntry>,
    pub selected_index: usize,
    pub disabled_indices: Vec<usize>,
    pub filter_value: String,
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
            filter_value: "".to_owned(),
            config,
            selected_index: 0,
            disabled_indices: Vec::with_capacity(entries.len()),
            entries,
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

    pub fn view(&self) -> Column<'_, Message> {
        let filter_label = Text::new("Filter").size(self.config.text_size);
        let filter_input = TextInput::new(&self.filter_value, &self.filter_value)
            .size(self.config.text_size)
            .width(Length::Fixed(self.window_size.0 * 0.45))
            .style(|_, _| Style {
                background: Background::Color(to_color(&self.config.background)),
                border: Border {
                    color: to_color(&self.config.text_color),
                    width: 2.0,
                    radius: 5.0.into(),
                },
                icon: to_color(&self.config.text_color),
                placeholder: to_color(&self.config.text_color),
                value: to_color(&self.config.text_color),
                selection: to_color(&self.config.text_color),
            })
            .on_input(|val| Message::FilterChanged(val));
        let widgets_per_col = self.widgets_per_col();
        let mut rows: Vec<Element<Message>> = vec![];
        let mut current_row = Row::new().spacing(self.config.column_gap);
        let mut num_elements = 0;
        for (ind, entry) in self.entries.iter().enumerate() {
            if self.disabled_indices.contains(&ind) {
                continue;
            }
            let button = EntryWidget::new(entry, &self.config, ind == self.selected_index).view();
            current_row = current_row.push(button);
            num_elements += 1;
            if num_elements >= widgets_per_col {
                rows.push(current_row.into());
                num_elements = 0;
                current_row = Row::new().spacing(self.config.column_gap);
            }
        }
        if num_elements != 0 {
            rows.push(current_row.into());
        }
        Column::from_vec(vec![
            Container::new(
                Row::from_vec(vec![filter_label.into(), filter_input.into()]).spacing(10),
            )
            .center_x(Length::Fill)
            .into(),
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
            .into(),
        ])
        .align_x(Horizontal::Center)
        .padding(self.config.padding)
        .width(Length::Fill)
        .height(Length::Fill)
    }
}
