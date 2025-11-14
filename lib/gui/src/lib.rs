use config::AppConfig;
use iced::{
    Color, Element, Task, application, application::Appearance, event, event::Event, keyboard,
    window, window::Settings,
};
use std::process::exit;

mod errors;
mod events;
mod menu_widget;
mod state;
pub use errors::Error;
use events::update;
use menu_widget::EntryWidget;
use state::{MenuState, Message};

pub const ENTRY_PATH: &str = "entries";
pub const EXIT_BUTTON: &str = "assets/exit.png";
pub const LOCK_BUTTON: &str = "assets/lock.png";
pub const REBOOT_BUTTON: &str = "assets/reboot.png";
pub const SHUTDOWN_BUTTON: &str = "assets/shutdown.png";
pub const CONFIG_NAMES: [&str; 4] = [
    "~/.config/tvmenu.conf",
    "~/.config/tvmenu/config",
    "~/.config/tvmenu/tvmenu.conf",
    "./tvmenu.conf",
];

pub fn run_app() -> Result<(), Error> {
    let mut config = AppConfig::load(&CONFIG_NAMES)?;
    let mut window_settings = Settings::default();
    if config.height != 0.0 {
        window_settings.size.height = config.height;
    } else {
        config.height = window_settings.size.height;
    }
    if config.width != 0.0 {
        window_settings.size.width = config.width;
    } else {
        config.width = window_settings.size.width;
    }

    let (w, h) = (window_settings.size.width, window_settings.size.height);

    let app = application("TV Menu", update, view)
        .style(|state, _| Appearance {
            background_color: to_color(&state.config.background),
            text_color: to_color(&state.config.text_color),
        })
        .centered()
        .subscription(|_| {
            event::listen_with(|event, _, _| match event {
                Event::Window(window::Event::Resized(size)) => Some(Message::Resized {
                    width: size.width,
                    height: size.height,
                }),
                Event::Keyboard(keyboard::Event::KeyPressed { key, .. }) => {
                    Some(Message::KeyPress(key))
                }
                _ => None,
            })
        });

    app.run_with(move || setup_app(config, w, h))?;
    Ok(())
}

fn setup_app(
    config: AppConfig,
    window_width: f32,
    window_height: f32,
) -> (MenuState, Task<Message>) {
    let state = report_err(MenuState::from_config(config, window_width, window_height));
    (state, Task::none())
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

fn view(state: &MenuState) -> Element<'_, Message> {
    state.view().into()
}

pub fn to_color(color: &config::Color) -> Color {
    Color::from_rgba(
        color.red as f32 / 255.0,
        color.green as f32 / 255.0,
        color.blue as f32 / 255.0,
        color.alpha as f32 / 255.0,
    )
}
