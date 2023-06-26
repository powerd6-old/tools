use path_utils::{children::ChildrenPaths, name::NamePaths};
use tracing::{debug, error, instrument};

use crate::{
    entry::{Entry, EntryFromNamedPath},
    RENDERING_DIRECTORY, UNDERSCORE_FILE_NAME,
};
use std::{
    error,
    ops::Deref,
    path::{Path, PathBuf},
};

/// A collection of Entries contained within a directory.
/// This structure does not represent the number of levels each entry is nested at.
#[derive(Debug, PartialEq)]
pub struct EntrySet {
    pub(crate) base_path: PathBuf,
    pub(crate) entries: Vec<Entry>,
}

impl EntrySet {
    /// Extends an EntrySet with entries from another.
    ///
    /// It will use the `base_path` from the original EntrySet.
    fn extend_entries(&mut self, extension: EntrySet) -> &Self {
        self.entries.extend(extension.entries);
        self
    }
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
    #[instrument(skip(self), fields(path=self.deref().as_ref().to_str().expect("Path should be a valid UTF-8 String")))]
    fn to_entry_set(&self) -> Option<EntrySet> {
        let path: &Path = self.deref().as_ref();
        if !path.exists() {
            error!("Tried to map an inexistent Path to an EntrySet");
            return None;
        }
        let mut result: EntrySet;
        if let Some(path_entry) = path
            .get_first_child_named(UNDERSCORE_FILE_NAME)
            .and(path.to_entry())
        {
            debug!("Found an UNDERSCORE file in Path. Mapping it to single Entry::Directory (or Entry::RenderingDirectory).");
            result = EntrySet {
                base_path: path.to_path_buf(),
                entries: vec![path_entry],
            };
        } else {
            debug!("Found no UNDERSCORE file in Path. Mapping each nested file to their own Entry::File.");
            let path_entries = path
                .get_children()
                .into_iter()
                .filter(|e| e.is_file())
                .map(Entry::File)
                .collect();
            result = EntrySet {
                base_path: path.to_path_buf(),
                entries: path_entries,
            };
        }
        // Loop over nested directories (except RENDERING)
        let nested_entries = path
            .get_children()
            .into_iter()
            .filter(|e| e.is_dir())
            .filter(|d| !d.is_named(RENDERING_DIRECTORY))
            .filter_map(|d| d.to_entry_set());
        debug!("Mapping nested directories in Path.");
        nested_entries.for_each(|n| {
            debug!(
                nested_path = n
                    .base_path
                    .to_str()
                    .expect("Path should be a valid UTF-8 String"),
                "Extending results with nested EntrySet"
            );
            result.extend_entries(n);
        });
        Some(result)
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
