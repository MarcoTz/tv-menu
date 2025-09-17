use crate::{Message, to_color};
use config::AppConfig;
use entries::MenuEntry;
use iced::{
    Border, Color, Element, Length,
    widget::{Button, Column, Container, button, container, image, text},
};
use std::path::PathBuf;

pub struct EntryWidget {
    title: String,
    launch: String,
    icon: Option<PathBuf>,
    text_size: f32,
    height: f32,
    width: f32,
    text_color: Color,
    background: Color,
    border_radius: f32,
}

impl EntryWidget {
    pub fn new(entry: &MenuEntry, conf: &AppConfig) -> EntryWidget {
        EntryWidget {
            title: entry.title.clone(),
            launch: entry.launch.clone(),
            icon: entry.icon.clone(),
            text_size: conf.entries.text_size,
            height: conf.entries.height,
            width: conf.entries.width,
            text_color: to_color(&conf.entries.text_color),
            background: to_color(&conf.entries.background),
            border_radius: conf.entries.border_radius,
        }
    }

    pub fn view<'a>(self) -> Button<'a, Message> {
        let title = text(self.title)
            .height(Length::Fixed(self.text_size))
            .width(Length::Fill)
            .center();
        let image: Element<Message> = if let Some(ref icon) = self.icon {
            image(icon).height(Length::Fill).width(Length::Fill).into()
        } else {
            text("").height(Length::Fill).width(Length::Fill).into()
        };
        let column = Column::new()
            .height(self.height)
            .width(self.width)
            .push(image)
            .push(title);
        let container = Container::new(column).style(move |_| {
            container::Style::default()
                .color(self.text_color)
                .background(self.background)
                .border(Border::default().rounded(self.border_radius))
        });
        Button::new(container)
            .on_press(Message::Launch(self.launch.to_owned()))
            .style(|_, _| button::Style::default())
    }
}
