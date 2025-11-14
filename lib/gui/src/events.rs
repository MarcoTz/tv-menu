use crate::{MenuState, Message};
use entries::launch_command;
use iced::keyboard::{Key, key::Named};

fn launch_entry(cmd: &str, args: &[String]) {
    match launch_command(cmd, args).spawn() {
        Ok(child) => std::mem::forget(child),
        Err(err) => eprintln!("Could not launch {cmd}:\n{err}"),
    }
}

pub fn update(state: &mut MenuState, msg: Message) {
    match msg {
        Message::Launch(cmd, args) => launch_entry(&cmd, &args),
        Message::Resized { height, width } => state.window_size = (width, height),
        Message::KeyPress(key) => handle_key(state, &key),
        Message::FilterChanged(filter) => update_filter(state, &filter),
    }
}

fn handle_key(state: &mut MenuState, key: &Key) {
    match key {
        Key::Named(Named::ArrowDown) => {
            let widgets_per_col = state.widgets_per_col() as usize;
            let new_ind = state.selected_index + widgets_per_col;
            if new_ind < state.entries.len() - 1 {
                state.selected_index = new_ind;
            } else {
                state.selected_index = state.entries.len() - 1;
            }
        }
        Key::Named(Named::ArrowUp) => {
            let widgets_per_col = state.widgets_per_col() as usize;
            if widgets_per_col > state.selected_index {
                state.selected_index = 0;
            } else {
                state.selected_index -= widgets_per_col;
            }
        }
        Key::Named(Named::ArrowLeft) => {
            if state.selected_index > 0 {
                state.selected_index -= 1;
            }
        }
        Key::Named(Named::ArrowRight) => {
            if state.selected_index < state.entries.len() - 1 {
                state.selected_index += 1;
            }
        }
        Key::Named(Named::Enter) => {
            let selected = &state.entries[state.selected_index];
            launch_entry(&selected.launch, &selected.args);
        }
        _ => (),
    }
}

fn update_filter(state: &mut MenuState, filter_value: &str) {
    filter_value.clone_into(&mut state.filter_value);
    state.disabled_indices.clear();
    for (ind, entry) in state.entries.iter().enumerate() {
        if !match_strings(&entry.title, filter_value) {
            state.disabled_indices.push(ind);
        }
    }
}

fn match_strings(title: &str, filter: &str) -> bool {
    title
        .to_lowercase()
        .trim()
        .contains(filter.to_lowercase().trim())
}
