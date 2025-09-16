use std::{
    fmt, io,
    path::{Path, PathBuf},
};

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
        section: String,
        key: String,
    },
    MissingSection {
        path: PathBuf,
        section: String,
    },
    UnexpectedKeys {
        path: PathBuf,
        section: String,
        keys: Vec<String>,
    },
    UnexpectedSections {
        path: PathBuf,
        sections: Vec<String>,
    },
}

impl Error {
    pub fn read_file(err: io::Error, path: &Path) -> Error {
        Error::ReadFile {
            path: path.to_path_buf(),
            reason: err.to_string(),
        }
    }

    pub fn format(path: &Path, line_nr: usize, reason: &str) -> Error {
        Error::InvalidFormat {
            path: path.to_path_buf(),
            line_nr,
            reason: reason.to_owned(),
        }
    }

    pub fn missing_key(path: &Path, section: &str, key: &str) -> Error {
        Error::MissingKey {
            path: path.to_path_buf(),
            section: section.to_owned(),
            key: key.to_owned(),
        }
    }

    pub fn missing_section(path: &Path, section: &str) -> Error {
        Error::MissingSection {
            path: path.to_path_buf(),
            section: section.to_owned(),
        }
    }

    pub fn unexpected_keys(path: &Path, section: &str, keys: Vec<&String>) -> Error {
        Error::UnexpectedKeys {
            path: path.to_path_buf(),
            section: section.to_owned(),
            keys: keys.into_iter().cloned().collect(),
        }
    }

    pub fn unexpected_sections(path: &Path, sections: Vec<&String>) -> Error {
        Error::UnexpectedSections {
            path: path.to_path_buf(),
            sections: sections.into_iter().cloned().collect(),
        }
    }
}

fn format_section(sec: &str) -> String {
    if sec.is_empty() {
        "".to_owned()
    } else {
        format!(" for section {sec}")
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
            Error::MissingKey { path, section, key } => {
                write!(
                    f,
                    "Missing key {key}{} in {path:?}",
                    format_section(section)
                )
            }
            Error::MissingSection { path, section } => {
                write!(f, "Missing section {section} in {path:?}")
            }
            Error::UnexpectedKeys {
                path,
                section,
                keys,
            } => {
                write!(
                    f,
                    "Unexpected keys {}{} in {path:?}",
                    keys.join(", "),
                    format_section(section)
                )
            }
            Error::UnexpectedSections { path, sections } => {
                write!(
                    f,
                    "Unexpected sections {} in path {path:?}",
                    sections.join(", ")
                )
            }
        }
    }
}

impl std::error::Error for Error {}
