use crate::types::Result;

pub struct ProjectDescription {
    pub name: Option<String>,
    pub description: Option<String>,
    pub version: Option<String>,
    pub artifact_notation: Option<String>
}

pub fn load_project_description() -> Result<ProjectDescription> {
    unimplemented!()
}


