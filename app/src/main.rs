use config::AppConfig;
use entries::MenuEntry;
use iced::{
    Border, Color, Element, Length, Task, application,
    application::Appearance,
    widget::{Button, Column, Container, Row, button, container, image, text},
};
use std::{path::PathBuf, process::Command, process::exit};

mod errors;
use errors::Error;

pub const ENTRY_PATH: &str = "entries";
pub const CONFIG_PATH: &str = "tvmenu.conf";

fn color_to_color(color: &config::Color) -> Color {
    Color::from_rgba(
        color.red as f32 / 255.0,
        color.green as f32 / 255.0,
        color.blue as f32 / 255.0,
        color.alpha as f32 / 255.0,
    )
}
pub fn launch_command(cmd: &str) -> Command {
    let mut parts = cmd.split(" ");
    let mut cmd = Command::new(parts.next().unwrap());
    for arg in parts {
        cmd.arg(arg);
    }
    cmd
}

struct MenuState {
    config: AppConfig,
    entries: Vec<MenuEntry>,
}

#[derive(Debug, Clone)]
enum Message {
    Launch(String),
}

fn main() -> Result<(), Error> {
    application("TV Menu", update, view)
        .style(|state, _| Appearance {
            background_color: color_to_color(&state.config.background),
            text_color: color_to_color(&state.config.text_color),
        })
        .centered()
        .run_with(setup_app)?;

    {}

    Ok(())
}

fn report_err<T, E>(res: Result<T, E>) -> T
where
    E: std::error::Error,
{
    match res {
        Ok(t) => t,
        Err(err) => {
            eprintln!("App encountered an error:\n{err}");
            exit(1)
        }
    }
}

fn setup_app<'a>() -> (MenuState, Task<Message>) {
    let entry_path = PathBuf::from(ENTRY_PATH);
    let config_path = PathBuf::from(CONFIG_PATH);

    let entries = report_err(MenuEntry::load_dir(entry_path));
    let config = report_err(AppConfig::from_file(config_path));
    (MenuState { config, entries }, Task::none())
}

fn update(_: &mut MenuState, msg: Message) {
    match msg {
        Message::Launch(cmd) => match launch_command(&cmd).spawn() {
            Ok(child) => std::mem::forget(child),
            Err(err) => eprintln!("Could not launch {cmd}:\n{err}"),
        },
    }
}

fn view(state: &MenuState) -> Row<'_, Message> {
    let mut entry_elements = vec![];
    for entry in state.entries.iter() {
        let title = text(&entry.title)
            .height(Length::Fixed(state.config.entries.text_size))
            .width(Length::Fill)
            .center();
        let image: Element<Message> = if let Some(ref icon) = entry.icon {
            image(icon).height(Length::Fill).width(Length::Fill).into()
        } else {
            text("").height(Length::Fill).width(Length::Fill).into()
        };
        let column = Column::new()
            .height(state.config.entries.height)
            .width(state.config.entries.width)
            .push(image)
            .push(title);
        let container = Container::new(column).style(|_| {
            container::Style::default()
                .color(color_to_color(&state.config.entries.text_color))
                .background(color_to_color(&state.config.entries.background))
                .border(Border::default().rounded(state.config.entries.border_radius))
        });
        let button = Button::new(container)
            .on_press(Message::Launch(entry.launch.clone()))
            .style(|_, _| button::Style::default());
        entry_elements.push(button.into());
    }
    Row::from_vec(entry_elements).padding(state.config.padding)
}
