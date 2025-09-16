use std::fmt;

#[derive(Debug)]
pub enum Error {
    UnknownSection(String),
    UnknownKey(String),
    Parser(parser::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
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
