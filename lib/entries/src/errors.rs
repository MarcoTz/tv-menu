use std::{fmt, io, path::PathBuf};

#[derive(Debug)]
pub enum Error {
    ReadFile {
        path: PathBuf,
        msg: String,
    },
    UnknownKey {
        path: PathBuf,
        line_nr: usize,
        key: String,
    },
    Parser(parser::Error),
}

impl Error {
    pub fn read_file(err: io::Error, path: &PathBuf) -> Error {
        Error::ReadFile {
            path: path.clone(),
            msg: err.to_string(),
        }
    }

    pub fn unknown_key(path: &PathBuf, line_nr: usize, key: &str) -> Error {
        Error::UnknownKey {
            path: path.clone(),
            line_nr,
            key: key.to_owned(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::ReadFile { path, msg } => write!(f, "Could not read file {path:?}:\n{msg}"),
            Error::UnknownKey { path, line_nr, key } => {
                write!(f, "Invalid Key {key} in {path:?}, line {line_nr}")
            }
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
