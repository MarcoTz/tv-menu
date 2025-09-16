use std::{
    fmt, io,
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub enum Error {
    ReadDir { path: PathBuf, reason: String },
    UnknownSection(String),
    UnknownKey(String),
    Parser(parser::Error),
}

impl Error {
    pub fn read_dir(err: io::Error, path: &Path) -> Error {
        Error::ReadDir {
            path: path.to_path_buf(),
            reason: err.to_string(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::ReadDir { path, reason } => write!(f, "Could not read dir {path:?}:\n{reason}"),
            Error::UnknownSection(sec) => write!(f, "Menu Entry cannot have section {sec}"),
            Error::UnknownKey(key) => write!(f, "Menu Entry cannot have key {key}"),
            Error::Parser(err) => err.fmt(f),
        }
    }
}

impl std::error::Error for Error {}

impl From<parser::Error> for Error {
    fn from(err: parser::Error) -> Error {
        Error::Parser(err)
    }
}
