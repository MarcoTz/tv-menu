use std::fmt;

#[derive(Debug)]
pub enum Error {
    Parse(parser::Error),
    InvalidColor(String),
    InvalidNumber(String),
    InvalidKey { section: String, key: String },
    InvalidSection(String),
    NoConfigFound(Vec<String>),
    HomeDir { path: String, msg: String },
}

impl Error {
    /// Create an [`Error::InvalidKey`] from a given section and key
    #[must_use]
    pub fn invalid_key(sec: &str, key: &str) -> Self {
        Self::InvalidKey {
            section: sec.to_owned(),
            key: key.to_owned(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Parse(err) => err.fmt(f),
            Self::InvalidColor(cl) => write!(f, "Not a valid color: {cl}"),
            Self::InvalidNumber(s) => write!(f, "Not a valid number: {s}"),
            Self::InvalidSection(sec) => write!(f, "Not a valid section: {sec}"),
            Self::InvalidKey { section, key } => {
                write!(f, "Not a valid key for section {section}: {key}")
            }
            Self::NoConfigFound(paths) => write!(
                f,
                "Could not find valid config file, searched:\n{}",
                paths.join("\n")
            ),
            Self::HomeDir { path, msg } => {
                write!(f, "Could not expand home directory for {path}:\n{msg}")
            }
        }
    }
}

impl std::error::Error for Error {}

impl From<parser::Error> for Error {
    fn from(err: parser::Error) -> Self {
        Self::Parse(err)
    }
}
