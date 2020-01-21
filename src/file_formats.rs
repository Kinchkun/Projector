use serde::Deserialize;
use std::path::{Path};
use toml;
use std::fs::read_to_string;
use crate::error::*;
use crate::error::ProjectorError::CouldNotOpenResourceError;


#[derive(Debug, PartialEq, Deserialize)]
pub struct ProjectDescription {
    pub name: Option<String>,
    pub description: Option<String>,
    pub version: Option<String>,
}

pub fn load_project_description(path: &Path) -> Result<ProjectDescription> {
    let content = read_to_string(path)
        .map_err( |err| ProjectorError::from(err))?;
    match toml::from_str(content.as_str()) {
        Ok(result) => Ok(result),
        Err(error) => Err(ProjectorError::from(error))
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn can_load_from_description_from_toml()  {
        let expected = ProjectDescription {
            name        : Some("Example Project".to_string()),
            description : Some("Example project description".to_string()),
            version     : Some("1.2.3-SNAPSHOT".to_string()),
        };
        let actual = load_project_description("test_assets/project_description.toml".as_ref())
            .expect("The file should be opened");

        assert_eq!(actual, expected);
    }
}

