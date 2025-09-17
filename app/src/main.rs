use gui::{MenuState, Message, to_color};
use iced::{Task, application, application::Appearance, widget::Row};
use std::{process::Command, process::exit};

mod errors;
use errors::Error;

pub fn launch_command(cmd: &str) -> Command {
    let mut parts = cmd.split(" ");
    let mut cmd = Command::new(parts.next().unwrap());
    for arg in parts {
        cmd.arg(arg);
    }
    cmd
}

fn main() -> Result<(), Error> {
    application("TV Menu", update, view)
        .style(|state, _| Appearance {
            background_color: to_color(&state.config.background),
            text_color: to_color(&state.config.text_color),
        })
        .centered()
        .run_with(setup_app)?;

    {}
    Ok(())
}

fn setup_app() -> (MenuState, Task<Message>) {
    let state = report_err(MenuState::init());
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

fn update(_: &mut MenuState, msg: Message) {
    match msg {
        Message::Launch(cmd) => match launch_command(&cmd).spawn() {
            Ok(child) => std::mem::forget(child),
            Err(err) => eprintln!("Could not launch {cmd}:\n{err}"),
        },
    }
}

fn view(state: &MenuState) -> Row<'_, Message> {
    state.view()
}
