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
    /// Create a new section from a given name and if it is required
    #[must_use]
    pub fn new(sec: &str, optional: bool) -> Self {
        Self {
            section: sec.to_owned(),
            optional,
        }
    }

    /// Create a new required section
    #[must_use]
    pub const fn empty() -> Self {
        Self {
            section: String::new(),
            optional: false,
        }
    }
}

impl Key {
    /// Create a key from a given name and whether it is optional
    #[must_use]
    pub fn new(key: &str, optional: bool) -> Self {
        Self {
            key: key.to_owned(),
            optional,
        }
    }
}

/// Trait for building types from a parsed [`ConfigContents`] (see [`from_contents`])
pub trait ConfigBuilder: Default {
    /// The type built from `Self`
    type Output;
    /// The Error type if parsing fails
    type Error: std::error::Error + From<Error>;
    /// sections in [`ConfigContents`] used by `Self`
    fn sections() -> Vec<Section>;
    /// keys in a given config section used by `Self`
    /// # Errors
    /// returns an error if the section is not in [`Self::sections`]
    fn section_keys(section: &str) -> Result<Vec<Key>, Self::Error>;
    /// Parse a value in a given section and a given key
    /// # Errors
    /// Returns an error if the value cannot be parsed to the required type
    /// or when either section or config are not part of `Self`
    fn parse_value(&mut self, section: &str, key: &str, value: &str) -> Result<(), Self::Error>;
    /// After adding all key-value pairs required by `Self`, build the output type
    fn build(self) -> Self::Output;
}

/// Given a [`ConfigBuilder`] and [`ConfigContents`], build [`ConfigBuilder::Output`]
/// # Errors
/// returns an error if
/// - a required section could not be found
/// - a required key could not be found
/// - a value could not be parsed
/// - there are remaining keys after parsing all keys in [`ConfigBuilder::section_keys`]
/// - there are remaining sections after parsing all sections in [`ConfigBuilder::sections`]
pub fn from_contents<Builder>(
    mut contents: ConfigContents,
) -> Result<Builder::Output, Builder::Error>
where
    Builder: ConfigBuilder,
{
    let mut builder = Builder::default();
    for section in Builder::sections() {
        let Some(mut config_section) = contents.sections.remove(&section.section) else {
            if section.optional {
                continue;
            }
            return Err(Error::missing_section(&contents.path, &section.section).into());
        };
        for key in Builder::section_keys(&section.section)? {
            let Some(value) = config_section.values.remove(&key.key) else {
                if key.optional {
                    continue;
                }
                return Err(Error::missing_key(&contents.path, &section.section, &key.key).into());
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
