use config::AppConfig;
use eframe::egui::{Button, Color32, Response, RichText, Ui, Widget};
use entries::MenuEntry;
use std::process::Command;

pub struct EntryWidget<'a> {
    title: &'a str,
    launch_command: &'a str,
    text_size: f32,
    text_color: Color32,
    background: Color32,
    radius: u8,
    width: Option<f32>,
    height: Option<f32>,
}

impl<'a> EntryWidget<'a> {
    pub fn new(conf: &'a AppConfig, entry: &'a MenuEntry) -> EntryWidget<'a> {
        EntryWidget {
            title: &entry.title,
            launch_command: &entry.launch,
            text_size: conf.entry_text_size,
            text_color: conf.entry_text_color,
            background: conf.entry_background,
            radius: conf.entry_radius,
            width: conf.entry_width,
            height: conf.entry_height,
        }
    }
}

impl<'a> Widget for EntryWidget<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let button = Button::new(
            RichText::new(self.title)
                .size(self.text_size)
                .color(self.text_color),
        )
        .fill(self.background)
        .corner_radius(self.radius);
        let resp = if let (Some(w), Some(h)) = (self.width, self.height) {
            ui.add_sized((w, h), button)
        } else {
            ui.add(button)
        };
        if resp.clicked() {
            Command::new(self.launch_command).spawn().unwrap();
        }
        resp
    }
}
