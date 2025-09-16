use std::fmt;

#[derive(Debug)]
pub enum Error {
    Entries(entries::Error),
    Config(config::Error),
    EFrame(eframe::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
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
