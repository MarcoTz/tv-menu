use config::AppConfig;
use eframe::{egui, egui::Widget};
use entries::MenuEntry;
use std::{fs::read_dir, path::PathBuf};
use widgets::EntryWidget;

mod errors;
use errors::Error;

pub const ENTRY_PATH: &str = "entries";
pub const CONFIG_PATH: &str = "tvmenu.conf";

struct MyApp {
    config: AppConfig,
    entries: Vec<MenuEntry>,
}

impl MyApp {
    pub fn new(entries: Vec<MenuEntry>, config: AppConfig) -> MyApp {
        MyApp { entries, config }
    }
}

fn main() -> Result<(), Error> {
    let entry_path = PathBuf::from(ENTRY_PATH);
    let config_path = PathBuf::from(CONFIG_PATH);
    let mut entries = vec![];
    for path_entry in read_dir(ENTRY_PATH).map_err(|err| Error::read_dir(err, &entry_path))? {
        let path_entry = path_entry.map_err(|err| Error::read_dir(err, &entry_path))?;
        let menu_entry = MenuEntry::from_file(path_entry.path())?;
        entries.push(menu_entry);
    }
    let config = AppConfig::from_file(config_path)?;
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "TV Menu",
        options,
        Box::new(|_| Ok(Box::new(MyApp::new(entries, config)))),
    )?;

    Ok(())
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                for entry in self.entries.iter() {
                    EntryWidget::new(&self.config, entry).ui(ui);
                }
            });
        });
    }
}
