use crate::{Error, ICON_DIRS, MenuEntry};
use parser::{ConfigBuilder, Key, Section};
use std::{ffi::OsStr, fs::read_dir, path::PathBuf};

#[derive(Default)]
pub struct EntryBuilder {
    title: Option<String>,
    launch: Option<String>,
    args: Vec<String>,
    icon: Option<PathBuf>,
}

impl ConfigBuilder for EntryBuilder {
    type Output = MenuEntry;
    type Error = Error;

    fn sections() -> Vec<Section> {
        vec![Section::empty()]
    }

    fn section_keys(section: &str) -> Result<Vec<Key>, Self::Error> {
        if section.is_empty() {
            Ok(vec![
                Key::new("title", false),
                Key::new("launch", false),
                Key::new("icon", true),
            ])
        } else {
            Err(Error::UnknownSection(section.to_owned()))
        }
    }

    fn parse_value(&mut self, section: &str, key: &str, value: &str) -> Result<(), Self::Error> {
        if !section.is_empty() {
            return Err(Error::UnknownSection(section.to_owned()));
        }

        match key.trim() {
            "title" => self.title = Some(value.to_owned()),
            "launch" => {
                for part in value.split(' ') {
                    if self.launch.is_none() {
                        self.launch = Some(part.to_owned());
                    } else {
                        self.args.push(part.to_owned());
                    }
                }
            }
            "icon" => self.icon = Some(find_icon(value)?),
            _ => return Err(Error::UnknownKey(key.to_owned())),
        }
        Ok(())
    }

    fn build(self) -> Self::Output {
        MenuEntry {
            title: self.title.unwrap(),
            launch: self.launch.unwrap(),
            args: self.args,
            icon: self.icon,
        }
    }
}

fn find_icon(name: &str) -> Result<PathBuf, Error> {
    let mut icons = vec![];
    for dir in ICON_DIRS {
        icons.extend(find_icon_dir(name, &PathBuf::from(&dir))?);
    }
    let icon_path = icons
        .iter()
        .filter_map(|ic| ic.metadata().ok().map(|met| (ic, met)))
        .map(|(ic, met)| (ic, met.len()))
        .max_by(|(_, siz1), (_, siz2)| siz1.cmp(siz2))
        .ok_or_else(|| Error::IconNotFound(name.to_owned()))?
        .0;
    Ok(icon_path.clone())
}

fn find_icon_dir(name: &str, dir: &PathBuf) -> Result<Vec<PathBuf>, Error> {
    let mut icons = vec![];
    for path in read_dir(dir).map_err(|err| Error::read_dir(&err, dir))? {
        let path = path.map_err(|err| Error::read_dir(&err, dir))?.path();
        if path.is_dir() {
            let dir_icons = find_icon_dir(name, &path)?;
            icons.extend(dir_icons);
            continue;
        }

        if path.extension() == Some(OsStr::new("svg")) {
            continue;
        }

        if path.file_stem() == Some(OsStr::new(name)) {
            icons.push(path);
        }
    }
    Ok(icons)
}
