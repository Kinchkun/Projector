use std::error::Error;
use std::{fmt, io};
use serde::export::Formatter;
use std::borrow::BorrowMut;
use std::intrinsics::write_bytes;
use clap::Format;

pub type Result<T, E = ProjectorError> = std::result::Result<T, E>;

#[derive()]
pub struct ProjectorError {
    message: Option<String>,
    what: Option<&'static str>,
    item: Option<String>,
    cause: Option<Box<dyn Error>>,
    kind: ErrorKind,
}

#[derive(Debug)]
pub enum ErrorKind {
    IoError,
    FormatError,
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

impl ProjectorError {
    fn format(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.kind {
            ErrorKind::IoError => write!(f, "An io error occurred"),
            ErrorKind::FormatError => {
                write!(f, "An format error occurred")?;
                self.format_cause(f)
            },
            ErrorKind::Other => write!(f, "An unknown at compile time error occurred"),
        }
    }

    fn format_cause(&self, f: &mut Formatter) -> fmt::Result {
        if let Some(cause) = &self.cause {
            write!(f, "\n Caused by: ")?;
            return std::fmt::Display::fmt(&cause, f)
        }
        Ok(())
    }
}

impl Error for ProjectorError { }

impl fmt::Display for ProjectorError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.format(f)
    }
}

impl fmt::Debug for ProjectorError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.format(f)
    }
}


impl From<std::io::Error> for ProjectorError {
    fn from(cause: std::io::Error) -> Self {
        unimplemented!()
    }
}

impl From<toml::de::Error> for ProjectorError {
    fn from(cause: toml::de::Error) -> Self {
        ProjectorError {
            message: Some(String::from("asdf")),
            what: None,
            item: None,
            cause: Some(Box::new(cause)),
            kind: ErrorKind::FormatError
        }
    }
}
