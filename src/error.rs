//! Error handling for morfo.

use std::{fmt, io::ErrorKind, path::PathBuf};

/// A specialized [`Result`] type for Morfo operations.
///
/// [`Result`]: https://doc.rust-lang.org/std/result/enum.Result.html
pub type MorfoResult<T> = Result<T, MorfoError>;

/// You can use the `MorfoError` type to handle errors in your code.
#[derive(PartialEq, Debug)]
pub enum MorfoError {
    CompilationFailure(Option<i32>),
    FileNotFound(PathBuf),
    InvlidConfig(String),
    InvalidConfigExtension(String),
    InvalidUnicode,
    IoError(ErrorKind),
    MissingConfigFile,
    MissingExecutable,
    MissingHomeDirectory,
}

impl fmt::Display for MorfoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MorfoError::CompilationFailure(code) => match code {
                Some(code) => {
                    write!(f, "Compilation failure: Process exited with code {}", code)
                }
                None => write!(f, "Compilation failure: Process terminated by signal"),
            },
            MorfoError::FileNotFound(path) => write!(f, "File not found: {}", path.display()),
            MorfoError::InvlidConfig(msg) => write!(f, "Invalid config: {}", msg),
            MorfoError::InvalidConfigExtension(ext) => {
                write!(f, "The config file must be a TOML file. Found: {}.", *ext)
            }
            MorfoError::InvalidUnicode => write!(f, "Invalid unicode"),
            MorfoError::MissingConfigFile => write!(f, "Config file missing."),
            MorfoError::MissingExecutable => write!(f, "Executable file missing."),
            MorfoError::MissingHomeDirectory => write!(f, "Home directory missing"),
            MorfoError::IoError(kind) => write!(f, "IO error: {}", kind),
        }
    }
}

impl From<std::io::Error> for MorfoError {
    fn from(error: std::io::Error) -> Self {
        MorfoError::IoError(error.kind())
    }
}

impl From<toml::de::Error> for MorfoError {
    fn from(error: toml::de::Error) -> Self {
        let msg = error.message();
        MorfoError::InvlidConfig(msg.to_owned())
    }
}
