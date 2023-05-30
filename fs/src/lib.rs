use std::path::PathBuf;

use path_utils::PathUtils;
use utils::get_paths_in_directory;

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

impl Entry {
    pub fn try_from_named(path: PathBuf, name: &str) -> Option<Entry> {
        if let Some(file) = path.get_first_child_named(name) {
            Some(Entry::File(file))
        } else {
            path.join(name)
                .get_first_child_named(UNDERSCORE_FILE_NAME)
                .map(|root_file| Entry::Directory {
                    root_file,
                    extra_files: get_paths_in_directory(&path.join(name))
                        .filter(|e| e.is_file())
                        .filter(|f| !f.is_file_named(UNDERSCORE_FILE_NAME))
                        .collect(),
                })
        }
    }
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
    pub fn try_from(base_path: PathBuf) -> Option<Self> {
        if !base_path.exists() {
            return None;
        }
        let mut entries: Vec<Entry> = Vec::new();
        if let Some(root_file) = base_path.get_first_child_named(UNDERSCORE_FILE_NAME) {
            // This path has an underscore file, will be mapped as Directory
            entries.push(Entry::Directory {
                root_file,
                extra_files: get_paths_in_directory(&base_path)
                    .filter(|e| e.is_file())
                    .filter(|f| !f.is_file_named(UNDERSCORE_FILE_NAME))
                    .collect(),
            })
        } else {
            // Each file in this path should be mapped to a new File
            entries.extend(
                get_paths_in_directory(&base_path)
                    .filter(|e| e.is_file())
                    .map(Entry::File),
            )
        }
        // Map children directories
        entries.extend(
            get_paths_in_directory(&base_path)
                .filter(|e| e.is_dir())
                .filter_map(EntrySet::try_from)
                .flat_map(|a| a.entries.into_iter()),
        );
        Some(EntrySet { base_path, entries })
    }
    pub fn try_from_with_rendering(base_path: PathBuf) -> Option<Self> {
        if !base_path.exists() {
            return None;
        }
        let mut entries: Vec<Entry> = Vec::new();
        if let Some(root_file) = base_path.get_first_child_named(UNDERSCORE_FILE_NAME) {
            // This path has an underscore file, will be mapped as Directory or RenderingDirectory
            if base_path.join(RENDERING_DIRECTORY).exists() {
                entries.push(Entry::RenderingDirectory {
                    root_file,
                    extra_files: get_paths_in_directory(&base_path)
                        .filter(|e| e.is_file())
                        .filter(|f| !f.is_file_named(UNDERSCORE_FILE_NAME))
                        .collect(),
                    rendering_files: get_paths_in_directory(&base_path.join(RENDERING_DIRECTORY))
                        .filter(|e| e.is_file())
                        .collect(),
                })
            } else {
                entries.push(Entry::Directory {
                    root_file,
                    extra_files: get_paths_in_directory(&base_path)
                        .filter(|e| e.is_file())
                        .filter(|f| !f.is_file_named(UNDERSCORE_FILE_NAME))
                        .collect(),
                })
            }
        } else {
            // Each file in this path should be mapped to a new File
            entries.extend(
                get_paths_in_directory(&base_path)
                    .filter(|e| e.is_file())
                    .map(Entry::File),
            )
        }
        // Map children directories (excluded RENDERING_DIRECTORY)
        entries.extend(
            get_paths_in_directory(&base_path)
                .filter(|e| e.is_dir())
                .filter(|d| !d.ends_with(RENDERING_DIRECTORY))
                .filter_map(EntrySet::try_from_with_rendering)
                .flat_map(|a| a.entries.into_iter()),
        );
        Some(EntrySet { base_path, entries })
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
        if let Some(module_entry) = Entry::try_from_named(value.clone(), MODULE) {
            let mut result = FileSystem::new(value.clone(), module_entry);
            if let Some(types_entries) =
                EntrySet::try_from_with_rendering(value.join(TYPES_DIRECTORY))
            {
                result = result.with_types(types_entries);
            }
            if let Some(content_entries) = EntrySet::try_from(value.join(CONTENTS_DIRECTORY)) {
                result = result.with_contents(content_entries);
            }
            Ok(result)
        } else {
            Err(FileSystemError::MissingRequiredEntry)
        }
    }
}

pub mod path_utils;
mod utils;
