use std::path::{PathBuf, Path};
use log::info;

#[derive(Debug)]
pub struct Projector {}

impl Projector {
    fn open_project(path: &Path) -> Project {
        info!("load project at {:#?}", path);
        unimplemented!()
    }

    fn discover(root_path: &Path) {
        unimplemented!()
    }
}

#[derive(Debug)]
pub struct Project {
    /// the name of the project
    pub name: String,
    pub description: Option<String>,
    pub version: Option<String>,
    pub path: PathBuf,
    // ??? Better a trait?
    pub kind: String
}

pub trait ProjectLoader {
    fn load(path: &Path) -> Project;
}
