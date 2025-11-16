use crate::{
    ENTRY_PATHS, EXIT_BUTTON, EntryWidget, Error, LOCK_BUTTON, REBOOT_BUTTON, SHUTDOWN_BUTTON,
    to_color,
};
use config::AppConfig;
use entries::MenuEntry;
use iced::{
    Background, Border, Element, Length,
    alignment::Horizontal,
    keyboard::Key,
    widget::{
        Button, Column, Container, Row, Scrollable, Text, TextInput, button, image,
        scrollable::{Direction, Scrollbar},
        text_input,
    },
};

/// Messages sent to [`crate::update`]
#[derive(Debug, Clone)]
pub enum Message {
    /// Launch an app with command and arguments
    Launch(String, Vec<String>),
    /// Window was resized
    Resized { width: f32, height: f32 },
    /// A Key was pressed
    KeyPress(Key),
    /// Contents of the filter input changed
    FilterChanged(String),
    /// Exit the app
    Exit,
    /// Lock the screen
    Lock,
    /// Reboot the system
    Reboot,
    /// Shutdown the system
    Shutdown,
}

/// State of the App
pub struct MenuState {
    /// configuration
    pub config: AppConfig,
    /// current window size
    pub window_size: (f32, f32),
    /// menu entries
    pub entries: Vec<MenuEntry>,
    /// currently selected item
    pub selected_index: usize,
    /// invisible entries (used for filtering)
    pub disabled_indices: Vec<usize>,
    /// current value of the filter input
    pub filter_value: String,
}

impl MenuState {
    pub fn from_config(
        config: AppConfig,
        window_width: f32,
        window_height: f32,
    ) -> Result<Self, Error> {
        let entries = MenuEntry::load_dirs(&ENTRY_PATHS)?;
        Ok(Self {
            window_size: (window_width, window_height),
            filter_value: String::new(),
            config,
            selected_index: 0,
            disabled_indices: Vec::with_capacity(entries.len()),
            entries,
        })
    }

    pub fn widgets_per_col(&self) -> u64 {
        self.config.columns.map_or_else(
            || {
                ((self.window_size.0 - self.config.padding)
                    / (self.config.entries.width + self.config.column_gap))
                    .floor() as u64
            },
            |cols| cols,
        )
    }

    pub fn view_filter(&self) -> Container<'_, Message> {
        let filter_label = Text::new("Filter").size(self.config.text_size);
        let filter_input = TextInput::new(&self.filter_value, &self.filter_value)
            .size(self.config.text_size)
            .width(Length::Fixed(self.window_size.0 * 0.45))
            .style(|_, _| text_input::Style {
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
            .on_input(Message::FilterChanged);
        Container::new(Row::from_vec(vec![filter_label.into(), filter_input.into()]).spacing(10))
            .center_x(Length::Fill)
    }

    pub fn view_menu(&self) -> Scrollable<'_, Message> {
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

    fn view_power(&self) -> Container<'_, Message> {
        let exit_button = Button::new(image(EXIT_BUTTON))
            .height(self.config.text_size * 2.0)
            .style(|_, _| button::Style {
                background: Some(Background::Color(to_color(&self.config.entries.background))),
                text_color: to_color(&self.config.entries.text_color),
                border: Border::default().rounded(self.config.entries.border_radius),
                ..Default::default()
            })
            .on_press(Message::Exit);
        let lock_button = Button::new(image(LOCK_BUTTON))
            .height(self.config.text_size * 2.0)
            .style(|_, _| button::Style {
                background: Some(Background::Color(to_color(&self.config.entries.background))),
                text_color: to_color(&self.config.entries.text_color),
                border: Border::default().rounded(self.config.entries.border_radius),
                ..Default::default()
            })
            .on_press(Message::Lock);
        let reboot_button = Button::new(image(REBOOT_BUTTON))
            .height(self.config.text_size * 2.0)
            .style(|_, _| button::Style {
                background: Some(Background::Color(to_color(&self.config.entries.background))),
                text_color: to_color(&self.config.entries.text_color),
                border: Border::default().rounded(self.config.entries.border_radius),
                ..Default::default()
            })
            .on_press(Message::Reboot);
        let shutdown_button = Button::new(image(SHUTDOWN_BUTTON))
            .height(self.config.text_size * 2.0)
            .style(|_, _| button::Style {
                background: Some(Background::Color(to_color(&self.config.entries.background))),
                text_color: to_color(&self.config.entries.text_color),
                border: Border::default().rounded(self.config.entries.border_radius),
                ..Default::default()
            })
            .on_press(Message::Shutdown);
        Container::new(Row::from_vec(vec![
            lock_button.into(),
            shutdown_button.into(),
            reboot_button.into(),
            exit_button.into(),
        ]))
        .center_x(Length::Fill)
    }

    pub fn view(&self) -> Column<'_, Message> {
        Column::from_vec(vec![
            self.view_filter().into(),
            self.view_menu().into(),
            self.view_power().into(),
        ])
        .align_x(Horizontal::Center)
        .padding(self.config.padding)
        .width(Length::Fill)
        .height(Length::Fill)
    }
}
