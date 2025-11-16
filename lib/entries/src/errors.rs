use std::{
    fmt, io,
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub enum Error {
    ReadDir { path: PathBuf, reason: String },
    UnknownSection(String),
    UnknownKey(String),
    IconNotFound(String),
    Parser(parser::Error),
    Config(config::Error),
    NoEntriesFound,
}

impl Error {
    /// Create an [`Error::ReadDir`] from given path and [`io::Error`]
    #[must_use]
    pub fn read_dir(err: &io::Error, path: &Path) -> Self {
        Self::ReadDir {
            path: path.to_path_buf(),
            reason: err.to_string(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ReadDir { path, reason } => {
                write!(f, "Could not read dir {}:\n{reason}", path.display())
            }
            Self::UnknownSection(sec) => write!(f, "Menu Entry cannot have section {sec}"),
            Self::UnknownKey(key) => write!(f, "Menu Entry cannot have key {key}"),
            Self::IconNotFound(name) => write!(f, "Could not find icon {name}"),
            Self::Parser(err) => err.fmt(f),
            Self::NoEntriesFound => f.write_str("Could not find any menu entries"),
            Self::Config(err) => err.fmt(f),
        }
    }
}

impl std::error::Error for Error {}

impl From<parser::Error> for Error {
    fn from(err: parser::Error) -> Self {
        Self::Parser(err)
    }
}

impl From<config::Error> for Error {
    fn from(err: config::Error) -> Self {
        Self::Config(err)
    }
}
