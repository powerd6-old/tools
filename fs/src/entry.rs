use std::{
    ops::Deref,
    path::{Path, PathBuf},
};

use path_utils::{children::ChildrenPaths, name::NamePaths};

use crate::{FileSystemError, UNDERSCORE_FILE_NAME};

/// A collection of one or more file system resources that corresponds to a single data value.
#[derive(Debug, PartialEq)]
pub enum Entry {
    /// The data object is represented by a single file.
    File(PathBuf),
    /// The data object is represented by a sparse directory,
    /// containing a root file and zero or more additional files.
    Directory {
        root_file: PathBuf,
        extra_files: Vec<PathBuf>,
    },
    /// The data object is represented by a sparse directory,
    /// containing a root file and zero or more additional files for data,
    /// as well as zero or more additional files for rendering templates.
    RenderingDirectory {
        root_file: PathBuf,
        extra_files: Vec<PathBuf>,
        rendering_files: Vec<PathBuf>,
    },
}

pub trait EntryFromNamedPath {
    /// Create an Entry from a file or directory inside the path with a given name, if it exists
    fn has_entry_named(&self, name: String) -> Option<Entry>;
}

impl<T: AsRef<Path>> EntryFromNamedPath for T {
    fn has_entry_named(&self, name: String) -> Option<Entry> {
        let path: &Path = self.deref().as_ref();
        if let Some(file) = path.get_first_child_named(&name) {
            Some(Entry::File(file))
        } else {
            let named_directory = path.join(name);
            named_directory
                .get_first_child_named(UNDERSCORE_FILE_NAME)
                .map(|underscore_file| Entry::Directory {
                    root_file: underscore_file,
                    extra_files: named_directory
                        .get_children()
                        .into_iter()
                        .filter(|e| e.is_file())
                        .filter(|f| !f.is_named(UNDERSCORE_FILE_NAME))
                        .collect(),
                })
        }
    }
}
