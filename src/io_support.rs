//! A module support module for input/output operations

use crate::error::*;

use serde::Deserialize;
use serde::de::DeserializeOwned;
use std::fs::{read_to_string, read};
use std::path::Path;
use toml;
use std::io::Read;
use crate::error::ResultExt;

pub fn parse_file<T : DeserializeOwned>(path: &Path) -> Result<T> {
    let content = read_to_string(path)
        .err_context(&ErrCtx { what: "Opening a file for reading"})?;
    return toml::from_str(content.as_str())
        .err_context(&ErrCtx { what: "Opening a file for reading"});
}

