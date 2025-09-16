use crate::{Error, MenuEntry};
use parser::{ConfigBuilder, Key, Section};

#[derive(Default)]
pub struct EntryBuilder {
    title: Option<String>,
    launch: Option<String>,
}

impl ConfigBuilder for EntryBuilder {
    type Output = MenuEntry;
    type Error = Error;

    fn sections() -> Vec<Section> {
        vec![Section::empty()]
    }

    fn section_keys(section: &str) -> Result<Vec<Key>, Self::Error> {
        if section.is_empty() {
            Ok(vec![Key::new("title", false), Key::new("launch", false)])
        } else {
            Err(Error::UnknownSection(section.to_owned()))
        }
    }

    fn parse_value(&mut self, section: &str, key: &str, value: &str) -> Result<(), Self::Error> {
        if !section.is_empty() {
            return Err(Error::UnknownSection(section.to_owned()));
        }

        match key.trim() {
            "title" => {
                self.title = Some(value.to_owned());
            }
            "launch" => {
                self.launch = Some(value.to_owned());
            }
            _ => return Err(Error::UnknownKey(key.to_owned())),
        };
        Ok(())
    }

    fn build(self) -> Self::Output {
        MenuEntry {
            title: self.title.unwrap(),
            launch: self.launch.unwrap(),
        }
    }
}
