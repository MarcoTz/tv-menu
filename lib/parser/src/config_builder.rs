use crate::{ConfigContents, Error};

pub struct Key {
    key: String,
    optional: bool,
}

pub struct Section {
    section: String,
    optional: bool,
}

impl Section {
    pub fn new(sec: &str, optional: bool) -> Section {
        Section {
            section: sec.to_owned(),
            optional,
        }
    }

    pub fn empty() -> Section {
        Section {
            section: "".to_owned(),
            optional: false,
        }
    }
}

impl Key {
    pub fn new(key: &str, optional: bool) -> Key {
        Key {
            key: key.to_owned(),
            optional,
        }
    }
}

pub trait ConfigBuilder: Default {
    type Output;
    type Error: std::error::Error + From<Error>;
    fn sections() -> Vec<Section>;
    fn section_keys(section: &str) -> Result<Vec<Key>, Self::Error>;
    fn parse_value(&mut self, section: &str, key: &str, value: &str) -> Result<(), Self::Error>;
    fn build(self) -> Self::Output;
}

pub fn from_contents<Builder>(
    mut contents: ConfigContents,
) -> Result<Builder::Output, Builder::Error>
where
    Builder: ConfigBuilder,
{
    let mut builder = Builder::default();
    for section in Builder::sections() {
        let mut config_section = match contents.sections.remove(&section.section) {
            Some(sec) => sec,
            None => {
                if section.optional {
                    continue;
                } else {
                    return Err(Error::missing_section(&contents.path, &section.section).into());
                }
            }
        };
        for key in Builder::section_keys(&section.section)? {
            let value = match config_section.values.remove(&key.key) {
                Some(val) => val,
                None => {
                    if key.optional {
                        continue;
                    } else {
                        return Err(
                            Error::missing_key(&contents.path, &section.section, &key.key).into(),
                        );
                    }
                }
            };
            builder.parse_value(&section.section, &key.key, &value)?;
        }
        if !config_section.values.is_empty() {
            return Err(Error::unexpected_keys(
                &contents.path,
                &section.section,
                config_section.values.keys().collect(),
            )
            .into());
        }
    }
    if !contents.sections.is_empty() {
        return Err(
            Error::unexpected_sections(&contents.path, contents.sections.keys().collect()).into(),
        );
    }
    Ok(builder.build())
}
