use std::error::Error;
use std::{fmt, io};
use serde::export::Formatter;
use std::borrow::BorrowMut;

// TODO Learn proper error handling in rust
// Alternatives:
//   - one error type with kind, message, additional information
//   - several specialised error types
//   - libraries
//     - snafu: looks good, doesn't work with clion
//     - failure: good for apps, but for libs?

pub type Result<T, E = ProjectorError> = std::result::Result<T, E>;

#[derive(Debug)]
pub struct ProjectorError {
    what: Option<&'static str>,
    item: Option<String>,
    kind: Kind
}

#[derive(Debug)]
pub enum Kind {
    IoError     {cause: io::Error},
    FormatError {cause: Box<dyn Error>},
    Other
}

impl ProjectorError {
    fn set_what(self, what: &'static str) -> ProjectorError {
        ProjectorError {
            what: Some(what),
            ..self
        }
    }
}


pub trait ResultExtProject<T> : Sized {
    fn what(self, what: &'static str) -> Result<T>;
}

impl<T> ResultExtProject<T> for Result<T> {
    fn what(self, what: &'static str) -> Result<T, ProjectorError> {
        return self.map_err(| err | err.set_what(what))
    }
}

impl Error for ProjectorError {}

impl fmt::Display for ProjectorError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        unimplemented!()
    }
}


impl From<std::io::Error> for ProjectorError {
    fn from(cause: std::io::Error) -> Self {
        unimplemented!()
    }
}

impl From<toml::de::Error> for ProjectorError {
    fn from(cause: toml::de::Error) -> Self {
        unimplemented!()
    }
}
