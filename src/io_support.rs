//! A module support module for input/output operations

use crate::error::*;

use serde::Deserialize;
use serde::de::DeserializeOwned;
use std::fs::read_to_string;
use std::path::Path;
use toml;

pub fn parse_file<T : DeserializeOwned>(path: &Path) -> Result<T> {
    let content = read_to_string(path)
        .map_err( |err| ProjectorError::from(err))?;
    match toml::from_str(content.as_str()) {
        Ok(result) => Ok(result),
        Err(error) => Err(ProjectorError::from(error))
    }
}

