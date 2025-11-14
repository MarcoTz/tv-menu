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
    #[must_use]
    pub fn read_file(err: &io::Error, path: &Path) -> Self {
        Self::ReadFile {
            path: path.to_path_buf(),
            reason: err.to_string(),
        }
    }

    #[must_use]
    pub fn format(path: &Path, line_nr: usize, reason: &str) -> Self {
        Self::InvalidFormat {
            path: path.to_path_buf(),
            line_nr,
            reason: reason.to_owned(),
        }
    }

    #[must_use]
    pub fn missing_key(path: &Path, section: &str, key: &str) -> Self {
        Self::MissingKey {
            path: path.to_path_buf(),
            section: section.to_owned(),
            key: key.to_owned(),
        }
    }

    #[must_use]
    pub fn missing_section(path: &Path, section: &str) -> Self {
        Self::MissingSection {
            path: path.to_path_buf(),
            section: section.to_owned(),
        }
    }

    #[must_use]
    pub fn unexpected_keys(path: &Path, section: &str, keys: Vec<&String>) -> Self {
        Self::UnexpectedKeys {
            path: path.to_path_buf(),
            section: section.to_owned(),
            keys: keys.into_iter().cloned().collect(),
        }
    }

    #[must_use]
    pub fn unexpected_sections(path: &Path, sections: Vec<&String>) -> Self {
        Self::UnexpectedSections {
            path: path.to_path_buf(),
            sections: sections.into_iter().cloned().collect(),
        }
    }
}

fn format_section(sec: &str) -> String {
    if sec.is_empty() {
        String::new()
    } else {
        format!(" for section {sec}")
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ReadFile { path, reason } => {
                write!(f, "Could not read file {}:\n{reason}", path.display())
            }
            Self::InvalidFormat {
                path,
                line_nr,
                reason,
            } => {
                write!(
                    f,
                    "Could not parse line {line_nr} of {}: {reason}",
                    path.display()
                )
            }
            Self::MissingKey { path, section, key } => {
                write!(
                    f,
                    "Missing key {key}{} in {}",
                    format_section(section),
                    path.display()
                )
            }
            Self::MissingSection { path, section } => {
                write!(f, "Missing section {section} in {}", path.display())
            }
            Self::UnexpectedKeys {
                path,
                section,
                keys,
            } => {
                write!(
                    f,
                    "Unexpected keys {}{} in {}",
                    keys.join(", "),
                    format_section(section),
                    path.display()
                )
            }
            Self::UnexpectedSections { path, sections } => {
                write!(
                    f,
                    "Unexpected sections {} in path {}",
                    path.display(),
                    sections.join(", ")
                )
            }
        }
    }
}

impl std::error::Error for Error {}
