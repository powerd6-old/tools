use tracing::debug;

use crate::entry::Entry;
use std::{
    ops::Deref,
    path::{Path, PathBuf},
};

/// A collection of Entries contained within a directory.
/// This structure does not represent the number of levels each entry is nested at.
#[derive(Debug, PartialEq)]
pub struct EntrySet {
    base_path: PathBuf,
    entries: Vec<Entry>,
}

pub trait EntrySetFromPath {
    /// Create an EntrySet from a file or directory inside the path.
    ///
    /// This method recursively maps the path.
    ///
    /// If the EntrySet is empty, then `None` is returned instead.
    fn to_entry_set(&self) -> Option<EntrySet>;
}

impl<T: AsRef<Path>> EntrySetFromPath for T {
    fn to_entry_set(&self) -> Option<EntrySet> {
        let path: &Path = self.deref().as_ref();
        if !path.exists() {
            return None;
        }
        todo!()
    }
}

#[cfg(test)]
mod tests {

    use crate::{RENDERING_DIRECTORY, UNDERSCORE_FILE_NAME};

    use super::*;
    use path_utils::{create_test_directory, create_test_file};
    use pretty_assertions::assert_eq;
    use testdir::testdir;

    #[test]
    fn maps_folder_with_underscore_file_as_single_entry() {
        let dir = testdir!();

        let underscore_file =
            create_test_file(&dir.join(format!("{}.json", UNDERSCORE_FILE_NAME)), "");
        let sibling_file = create_test_file(&dir.join("a.json"), "");

        assert_eq!(
            dir.to_entry_set().unwrap(),
            EntrySet {
                base_path: dir,
                entries: vec![Entry::Directory {
                    root_file: underscore_file,
                    extra_files: vec![sibling_file]
                }]
            }
        )
    }

    #[test]
    fn maps_folder_without_underscore_file_as_multiple_entries() {
        let dir = testdir!();

        let first_file = create_test_file(&dir.join("a.json"), "");
        let second_file = create_test_file(&dir.join("b.json"), "");

        assert_eq!(
            dir.to_entry_set().unwrap(),
            EntrySet {
                base_path: dir,
                entries: vec![Entry::File(first_file), Entry::File(second_file)]
            }
        )
    }

    #[test]
    fn maps_rendering_subdirectory_to_parent_entry() {
        let dir = testdir!();

        let underscore_file =
            create_test_file(&dir.join(format!("{}.json", UNDERSCORE_FILE_NAME)), "");
        let sibling_file = create_test_file(&dir.join("a.json"), "");
        let rendering_directory = create_test_directory(&dir.join(RENDERING_DIRECTORY));
        let rendering_file = create_test_file(&rendering_directory.join("txt.hjs"), "contents");

        assert_eq!(
            dir.to_entry_set().unwrap(),
            EntrySet {
                base_path: dir,
                entries: vec![Entry::RenderingDirectory {
                    root_file: underscore_file,
                    extra_files: vec![sibling_file],
                    rendering_files: vec![rendering_file]
                }]
            }
        )
    }

    #[test]
    fn maps_nested_directories() {
        let dir = testdir!();

        let first_dir = create_test_directory(&dir.join("first"));
        let first_file = create_test_file(&first_dir.join("a.json"), "");

        let second_dir = create_test_directory(&dir.join("second"));
        let second_file = create_test_file(&second_dir.join("b.json"), "");

        assert_eq!(
            dir.to_entry_set().unwrap(),
            EntrySet {
                base_path: dir,
                entries: vec![Entry::File(first_file), Entry::File(second_file)]
            }
        )
    }
}
