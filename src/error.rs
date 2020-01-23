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

pub type Result<T, E = ProjectorError> = std::result::Result<T, E>;

#[derive(Debug)]
pub enum ProjectorError {
    CouldNotOpenResourceError { what: &'static str },
    InvalidFormatError        { },
    OtherError                { }
}

pub struct ErrCtx {
    pub what: &'static str
}

pub trait ResultExt<T> : Sized  {
    fn err_context(self, ctx: &ErrCtx) -> Result<T, ProjectorError>;
}

impl<T>  ResultExt<T> for Result<T, io::Error> {
    fn err_context(self, ctx: &ErrCtx) -> Result<T, ProjectorError> {
        self.map_err( |err | CouldNotOpenResourceError { what: ctx.what} )
    }
}

impl<T>  ResultExt<T> for Result<T, toml::de::Error> {
    fn err_context(self, ctx: &ErrCtx) -> Result<T, ProjectorError> {
        self.map_err( |err | InvalidFormatError {} )
    }
}


impl Error for ProjectorError {}

impl fmt::Display for ProjectorError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            ProjectorError::InvalidFormatError {} =>
                write!(f, "Invalid format (TODO)"),
            ProjectorError::CouldNotOpenResourceError {..} =>
                write!(f, "The file could not be opened"),
            _ =>
                write!(f, "unkown error")
        }
    }
}

