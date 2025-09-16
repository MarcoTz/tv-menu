use eframe::egui;
use entries::MenuEntry;
use std::{fs::read_dir, path::PathBuf};

mod errors;
use errors::Error;

pub const ENTRY_PATH: &str = "entries";

struct MyApp {
    entries: Vec<MenuEntry>,
}

impl MyApp {
    pub fn new(entries: Vec<MenuEntry>) -> MyApp {
        MyApp { entries }
    }
}

fn main() -> Result<(), Error> {
    let entry_path = PathBuf::from(ENTRY_PATH);
    let mut entries = vec![];
    for path_entry in read_dir(ENTRY_PATH).map_err(|err| Error::read_dir(err, &entry_path))? {
        let path_entry = path_entry.map_err(|err| Error::read_dir(err, &entry_path))?;
        let menu_entry = MenuEntry::from_file(path_entry.path())?;
        entries.push(menu_entry);
    }
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "TV Menu",
        options,
        Box::new(|_| Ok(Box::new(MyApp::new(entries)))),
    )?;

    Ok(())
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                for entry in self.entries.iter() {
                    ui.label(format!("{}\n{}", entry.title, entry.launch));
                }
            });
        });
    }
}
