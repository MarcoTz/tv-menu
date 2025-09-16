use std::{fmt, io, path::PathBuf};

#[derive(Debug)]
pub enum Error {
    ReadFile {
        path: PathBuf,
        reason: String,
    },
    InvalidFormat {
        path: PathBuf,
        line_nr: usize,
        reason: String,
    },
    MissingKey {
        path: PathBuf,
        key: String,
    },
}

impl Error {
    pub fn read_file(err: io::Error, path: &PathBuf) -> Error {
        Error::ReadFile {
            path: path.clone(),
            reason: err.to_string(),
        }
    }

    pub fn format(path: &PathBuf, line_nr: usize, reason: &str) -> Error {
        Error::InvalidFormat {
            path: path.clone(),
            line_nr,
            reason: reason.to_owned(),
        }
    }

    pub fn missing_key(path: &PathBuf, key: &str) -> Error {
        Error::MissingKey {
            path: path.clone(),
            key: key.to_owned(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::ReadFile { path, reason } => {
                write!(f, "Could not read file {path:?}:\n{reason}")
            }
            Error::InvalidFormat {
                path,
                line_nr,
                reason,
            } => {
                write!(f, "Could not parse line {line_nr} of {path:?}: {reason}")
            }
            Error::MissingKey { path, key } => {
                write!(f, "Missing key {key} in {path:?}")
            }
        }
    }
}

impl std::error::Error for Error {}
