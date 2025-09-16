use eframe;
use entries;
use std::{fmt, io, path::PathBuf};

#[derive(Debug)]
pub enum Error {
    ReadDir { path: PathBuf, reason: String },
    Entries(entries::Error),
    EFrame(eframe::Error),
}

impl Error {
    pub fn read_dir(err: io::Error, path: &PathBuf) -> Error {
        Error::ReadDir {
            path: path.clone(),
            reason: err.to_string(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::ReadDir { path, reason } => write!(f, "Could not read dir {path:?}:\n{reason}"),
            Error::Entries(err) => err.fmt(f),
            Error::EFrame(err) => err.fmt(f),
        }
    }
}

impl From<entries::Error> for Error {
    fn from(err: entries::Error) -> Error {
        Error::Entries(err)
    }
}

impl From<eframe::Error> for Error {
    fn from(err: eframe::Error) -> Error {
        Error::EFrame(err)
    }
}
