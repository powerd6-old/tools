use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};

/// The name of the file that corresponds to the root of a sparse directory.
pub const UNDERSCORE: &str = "_";
/// The name of the directory or file that corresponds to the module information.
const MODULE: &str = "module";
/// The name of the directory that contains the types.
const TYPES: &str = "types";
/// The name of the directory or file that corresponds to the rendering templates of a type.
const RENDERING: &str = "rendering";
/// The name of the directory that contains the contents.
const CONTENTS: &str = "contents";

/// A representation of a filesystem, tailored to build a Module from.
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

/// A single data object represented in the file system.
pub enum Entry {
    /// The data object is represented by a single file.
    File(PathBuf),
    /// The data object is represented by a sparse directory, containing a root file and zero or more additional files.
    Directory {
        root_file: PathBuf,
        extra_files: HashSet<PathBuf>,
    },
}

/// A collection of data objects contained within a directory.
/// This structure does not represent the number of levels each entry is nested at.
pub struct EntrySet {
    base_path: PathBuf,
    entries: HashSet<Entry>,
}

/// The errors that can happen when constructing a FileSystem
pub enum Error {
    MissingRequiredEntry,
}

impl TryFrom<&Path> for FileSystem {
    type Error = Error;

    fn try_from(value: &Path) -> Result<Self, Self::Error> {
        todo!("Write integration tests before implementation");
        todo!("Build the FileSystem from `value`.")
    }
}
