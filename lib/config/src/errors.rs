use std::fmt;

#[derive(Debug)]
pub enum Error {
    Parse(parser::Error),
    InvalidColor(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Parse(err) => err.fmt(f),
            Error::InvalidColor(cl) => write!(f, "Not a valid color: {cl}"),
        }
    }
}

impl std::error::Error for Error {}

impl From<parser::Error> for Error {
    fn from(err: parser::Error) -> Error {
        Error::Parse(err)
    }
}
