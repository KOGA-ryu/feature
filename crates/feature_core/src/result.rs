use std::error::Error;
use std::fmt::{Display, Formatter};
use std::io;
use std::path::PathBuf;

pub type FeatureLabResult<T> = Result<T, FeatureLabError>;

#[derive(Debug)]
pub enum FeatureLabError {
    Io {
        path: Option<PathBuf>,
        source: io::Error,
    },
    Toml {
        path: Option<PathBuf>,
        message: String,
    },
    Validation(String),
    NotFound(String),
    CommandFailed {
        command: String,
        status_code: i32,
        stdout: String,
        stderr: String,
    },
}

impl FeatureLabError {
    pub fn io(path: impl Into<Option<PathBuf>>, source: io::Error) -> Self {
        Self::Io {
            path: path.into(),
            source,
        }
    }

    pub fn toml(path: impl Into<Option<PathBuf>>, error: toml::de::Error) -> Self {
        Self::Toml {
            path: path.into(),
            message: error.to_string(),
        }
    }
}

impl Display for FeatureLabError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io { path, source } => match path {
                Some(path) => write!(f, "I/O error at {}: {source}", path.display()),
                None => write!(f, "I/O error: {source}"),
            },
            Self::Toml { path, message } => match path {
                Some(path) => write!(f, "TOML parse error at {}: {message}", path.display()),
                None => write!(f, "TOML parse error: {message}"),
            },
            Self::Validation(message) => write!(f, "Validation error: {message}"),
            Self::NotFound(message) => write!(f, "Not found: {message}"),
            Self::CommandFailed {
                command,
                status_code,
                stdout,
                stderr,
            } => write!(
                f,
                "Command failed ({status_code}): {command}\nstdout:\n{stdout}\nstderr:\n{stderr}"
            ),
        }
    }
}

impl Error for FeatureLabError {}

impl From<io::Error> for FeatureLabError {
    fn from(value: io::Error) -> Self {
        Self::io(None, value)
    }
}

impl From<toml::de::Error> for FeatureLabError {
    fn from(value: toml::de::Error) -> Self {
        Self::toml(None, value)
    }
}
