use snafu::{ensure, Backtrace, ErrorCompat, ResultExt, Snafu};
use std::path::PathBuf;

#[derive(Debug, Snafu)]
pub enum ProjectorError {
    #[snafu(display("Could not open project descriptor from {}: {}", filename.display(), source))]
    CouldNotOpenDescriptorFile {
        filename: PathBuf,
        source: std::io::Error
    }
}

pub type Result<T, E = ProjectorError> = std::result::Result<T, E>;