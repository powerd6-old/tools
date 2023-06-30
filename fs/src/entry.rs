use std::{
    ops::Deref,
    path::{Path, PathBuf},
};

use path_utils::{children::ChildrenPaths, name::NamePaths};
use tracing::{debug, instrument};

use crate::{RENDERING_DIRECTORY, UNDERSCORE_FILE_NAME};

/// A collection of one or more file system resources that corresponds to a single data value.
#[derive(Debug, PartialEq, Clone)]
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
    /// Create an Entry from a file or directory
    fn to_entry(&self) -> Option<Entry>;
}

impl<T: AsRef<Path>> EntryFromNamedPath for T {
    #[instrument(skip(self))]
    fn has_entry_named(&self, name: String) -> Option<Entry> {
        let path: &Path = self.deref().as_ref();
        path.get_first_child_named(&name).and_then(|c| c.to_entry())
    }

    #[instrument(skip(self))]
    fn to_entry(&self) -> Option<Entry> {
        let path: &Path = self.deref().as_ref();
        if path.is_file() {
            debug!("Path is a file. Mapping to Entry::File.");
            Some(Entry::File(path.to_path_buf()))
        } else {
            match path.get_first_child_named(UNDERSCORE_FILE_NAME) {
                Some(underscore_file) => {
                    let rendering_directory = path.join(RENDERING_DIRECTORY);
                    if rendering_directory.exists() {
                        debug!("Path is a directory with an UNDERSCORE file and RENDERING directory. Mapping to Entry::RenderingDirectory.");
                        Some(Entry::RenderingDirectory {
                            root_file: underscore_file,
                            extra_files: path
                                .get_children()
                                .into_iter()
                                .filter(|e| e.is_file())
                                .filter(|f| !f.is_named(UNDERSCORE_FILE_NAME))
                                .collect(),
                            rendering_files: rendering_directory
                                .get_children()
                                .into_iter()
                                .filter(|e| e.is_file())
                                .collect(),
                        })
                    } else {
                        debug!("Path is a directory with an UNDERSCORE file. Mapping to Entry::Directory.");
                        Some(Entry::Directory {
                            root_file: underscore_file,
                            extra_files: path
                                .get_children()
                                .into_iter()
                                .filter(|e| e.is_file())
                                .filter(|f| !f.is_named(UNDERSCORE_FILE_NAME))
                                .collect(),
                        })
                    }
                }
                None => None,
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::UNDERSCORE_FILE_NAME;

    use super::*;
    use path_utils::{create_test_directory, create_test_file};
    use pretty_assertions::assert_eq;
    use testdir::testdir;

    #[test]
    fn returns_file_when_file_named_exists() {
        let dir = testdir!();

        let file_name = "file_name";

        let named_file = create_test_file(&dir.join(file_name), "");

        assert_eq!(
            dir.has_entry_named(file_name.to_string()).unwrap(),
            Entry::File(named_file)
        );
    }

    #[test]
    fn returns_none_when_named_subdirectory_exists_but_has_no_underscore_file() {
        let dir = testdir!();

        let dir_name = "some_dir";

        let some_dir = create_test_directory(&dir.join(dir_name));

        create_test_file(&some_dir.join("a.json"), "");

        assert!(dir.has_entry_named(dir_name.to_string()).is_none());
    }

    #[test]
    fn returns_directory_when_named_subdirectory_exists_and_has_underscore_file() {
        let dir = testdir!();

        let dir_name = "some_dir";

        let some_dir = create_test_directory(&dir.join(dir_name));

        let underscore_file =
            create_test_file(&some_dir.join(format!("{}.json", UNDERSCORE_FILE_NAME)), "");
        let an_extra_file = create_test_file(&some_dir.join("a.json"), "");

        assert_eq!(
            dir.has_entry_named(dir_name.to_string()).unwrap(),
            Entry::Directory {
                root_file: underscore_file,
                extra_files: vec![an_extra_file]
            }
        );
    }
}
