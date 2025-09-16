use std::{
    fmt, io,
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub enum Error {
    ReadDir { path: PathBuf, reason: String },
    Entries(entries::Error),
    Config(config::Error),
    EFrame(eframe::Error),
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
            Error::Entries(err) => err.fmt(f),
            Error::Config(err) => err.fmt(f),
            Error::EFrame(err) => err.fmt(f),
        }
    }
}

impl From<entries::Error> for Error {
    fn from(err: entries::Error) -> Error {
        Error::Entries(err)
    }
}

impl From<config::Error> for Error {
    fn from(err: config::Error) -> Error {
        Error::Config(err)
    }
}

impl From<eframe::Error> for Error {
    fn from(err: eframe::Error) -> Error {
        Error::EFrame(err)
    }
}
