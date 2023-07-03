use std::path::Path;
use thiserror::Error;

/// The errors that can happen when reading a file into data.
#[derive(Error, Debug, PartialEq)]
pub enum FileSystemError {
    #[error("expected directory but found a file instead `{0}`")]
    ExpectedDirectory(Box<Path>),
    #[error("missing required entry {0}")]
    MissingRequiredEntry(String),
}

/// The name of the file that corresponds to the root of a sparse directory.
pub const UNDERSCORE_FILE_NAME: &str = "_";
/// The name of the directory or file that corresponds to the module information.
pub const MODULE: &str = "module";
/// The name of the directory that contains the types.
pub const TYPES_DIRECTORY: &str = "types";
/// The name of the directory or file that corresponds to the rendering templates of a type.
pub const RENDERING_DIRECTORY: &str = "rendering";
/// The name of the directory that contains the contents.
pub const CONTENTS_DIRECTORY: &str = "contents";

pub mod entry;
pub mod entry_set;
pub mod file_system;
