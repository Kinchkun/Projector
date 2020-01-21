use std::error::Error;
use std::{fmt, io};
use serde::export::Formatter;
use crate::error::ProjectorError::{CouldNotOpenResourceError, InvalidFormatError};

// TODO Learn proper error handling in rust
// Alternatives:
//   - one error type with kind, message, additional information
//   - several specialised error types
//   - libraries
//     - snafu: looks good, doesn't work with clion
//     - failure: good for apps, but for libs?


#[derive(Debug)]
pub enum ProjectorError {
    CouldNotOpenResourceError { message: String, filename: String, cause: io::Error },
    InvalidFormatError        { message: String }
}

pub type Result<T, E = ProjectorError> = std::result::Result<T, E>;

impl Error for ProjectorError {}

impl fmt::Display for ProjectorError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            ProjectorError::InvalidFormatError {..} =>
                write!(f, "Invalid format (TODO)"),
            ProjectorError::CouldNotOpenResourceError { message: _, filename: _, cause: _ } =>
                write!(f, "The file could not be opened")
        }
    }
}

impl From<io::Error> for ProjectorError {
    fn from(cause: io::Error) -> Self {
        CouldNotOpenResourceError {
            message: String::from("Message"),
            filename: String::from(""),
            cause
        }
    }
}

impl From<toml::de::Error> for ProjectorError {
    fn from(_: toml::de::Error) -> Self {
        InvalidFormatError {
            message: "".to_string()
        }
    }
}
