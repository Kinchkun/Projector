//! A module support module for input/output operations

use crate::error::*;

use serde::Deserialize;
use serde::de::DeserializeOwned;
use std::fs::{read_to_string, read};
use std::path::Path;
use toml;
use toml::de::Error;

pub fn parse_file<T : DeserializeOwned>(path: &Path) -> Result<T> {
    let content = read_to_string(path)?;
    match toml::from_str(content.as_str()) {
        Ok(result) => Ok(result),
        Err(err) => Err(err.into())
    }
}
