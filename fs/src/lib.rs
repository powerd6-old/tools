use std::path::PathBuf;

use utils::{get_files_with_name, get_paths_in_directory, is_file_name};

use crate::utils::has_file_named;

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

/// A representation of a filesystem, tailored to build a Module from.
#[derive(Debug, PartialEq, Eq)]
pub struct FileSystem {
    /// The root directory this FileSystem was built from.
    root_directory: PathBuf,
    // The module information
    module: Entry,
    // An optional set of entries that define types.
    types: Option<EntrySet>,
    // An optional set of entries that define contents.
    contents: Option<EntrySet>,
}

impl FileSystem {
    pub fn new(root_directory: PathBuf, module: Entry) -> Self {
        FileSystem {
            root_directory,
            module,
            types: None,
            contents: None,
        }
    }
    pub fn with_types(mut self, types: EntrySet) -> Self {
        self.types = Some(types);
        self
    }
    pub fn with_contents(mut self, contents: EntrySet) -> Self {
        self.contents = Some(contents);
        self
    }
}

/// A single data object represented in the file system.
#[derive(Debug, PartialEq, Eq)]
pub enum Entry {
    /// The data object is represented by a single file.
    File(PathBuf),
    /// The data object is represented by a sparse directory, containing a root file and zero or more additional files.
    Directory {
        root_file: PathBuf,
        extra_files: Vec<PathBuf>,
    },
    RenderingDirectory {
        root_file: PathBuf,
        extra_files: Vec<PathBuf>,
        rendering_files: Vec<PathBuf>,
    },
}

/// A collection of data objects contained within a directory.
/// This structure does not represent the number of levels each entry is nested at.
#[derive(Debug, PartialEq, Eq)]
pub struct EntrySet {
    base_path: PathBuf,
    entries: Vec<Entry>,
}

impl EntrySet {
    pub fn new(base_path: PathBuf, entries: Vec<Entry>) -> Self {
        EntrySet { base_path, entries }
    }
}

/// The errors that can happen when constructing a FileSystem
#[derive(Debug, PartialEq, Eq)]
pub enum FileSystemError {
    InvalidPath,
    MissingRequiredEntry,
}

impl TryFrom<PathBuf> for FileSystem {
    type Error = FileSystemError;

    fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
        if !value.exists() || value.is_file() {
            return Err(FileSystemError::InvalidPath);
        }
        if !value.join(MODULE).exists() && !has_file_named(&value, MODULE) {
            return Err(FileSystemError::MissingRequiredEntry);
        }
        Ok(FileSystem::new(
            value.clone(),
            get_files_with_name(value.as_path().clone(), MODULE)
                .map(|f| Entry::File(f))
                .unwrap_or_else(|| Entry::Directory {
                    root_file: get_files_with_name(&value.join(MODULE), UNDERSCORE_FILE_NAME)
                        .unwrap(),
                    extra_files: get_paths_in_directory(&value.join(MODULE))
                        .filter(|e| e.is_file())
                        .filter(|f| !is_file_name(f, UNDERSCORE_FILE_NAME))
                        .collect(),
                }),
        ))
    }
}

mod utils;
