use std::fmt;

#[derive(Debug)]
pub enum Error {
    Config(config::Error),
    Entries(entries::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Config(err) => err.fmt(f),
            Error::Entries(err) => err.fmt(f),
        }
    }
}

impl std::error::Error for Error {}

impl From<config::Error> for Error {
    fn from(err: config::Error) -> Error {
        Error::Config(err)
    }
}

impl From<entries::Error> for Error {
    fn from(err: entries::Error) -> Error {
        Error::Entries(err)
    }
}
