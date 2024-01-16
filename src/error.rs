use std::{fmt, path::PathBuf};

pub type MorfoResult<T> = Result<T, MorfoError>;

#[derive(PartialEq, Debug)]
pub enum MorfoError {
    FileNotFound(PathBuf),
    InvlidConfig(String),
    InvalidConfigExtension(String),
    InvalidUnicode,
    IoError(String),
    MissingConfigFile,
    MissingHomeDirectory,
}

impl fmt::Display for MorfoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MorfoError::FileNotFound(path) => write!(f, "File not found: {}", path.display()),
            MorfoError::InvlidConfig(msg) => write!(f, "Invalid config: {}", msg),
            MorfoError::InvalidConfigExtension(ext) => {
                write!(f, "The config file must be a TOML file. Found: {}.", *ext)
            }
            MorfoError::InvalidUnicode => write!(f, "Invalid unicode"),
            MorfoError::MissingConfigFile => write!(f, "Config file missing."),
            MorfoError::MissingHomeDirectory => write!(f, "Home directory missing"),
            MorfoError::IoError(error) => write!(f, "IO error: {}", error),
        }
    }
}

impl From<std::io::Error> for MorfoError {
    fn from(error: std::io::Error) -> Self {
        MorfoError::IoError(error.to_string())
    }
}

impl From<toml::de::Error> for MorfoError {
    fn from(error: toml::de::Error) -> Self {
        let msg = error.message();
        MorfoError::InvlidConfig(msg.to_owned())
    }
}
