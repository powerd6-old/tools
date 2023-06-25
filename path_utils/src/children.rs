use std::{
    fs::read_dir,
    path::{Path, PathBuf},
};

use crate::name::NamePaths;
pub trait ChildrenPaths {
    /// A helper function that maps all objects inside a directory into a `PathBuf` iterator
    fn get_children(&self) -> Vec<PathBuf>;
    /// Finds the first file (ordered alphabetically) in a directory with a specific filename, regardless of it's extension
    fn get_first_child_named(&self, name: &str) -> Option<PathBuf>;
}

impl<T: AsRef<Path>> ChildrenPaths for T {
    fn get_children(&self) -> Vec<PathBuf> {
        let mut path_children: Vec<PathBuf> = read_dir(self)
            .into_iter()
            .flatten()
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .collect();
        path_children.sort();
        path_children
    }

    fn get_first_child_named(&self, name: &str) -> Option<PathBuf> {
        self.get_children()
            .iter()
            .filter(|c| c.is_file())
            .find(|f| f.is_named(name))
            .cloned()
    }
}

#[cfg(test)]
mod tests {

    use crate::{create_test_directory, create_test_file};

    use super::*;
    use pretty_assertions::assert_eq;
    use testdir::testdir;

    #[test]
    fn nested_files_and_subdirectories_are_returned() {
        let dir = testdir!();

        let file = create_test_file(&dir.join("file.json"), "");
        let subdirectory = create_test_directory(&dir.join("subdirectory"));

        assert_eq!(dir.get_children(), [file, subdirectory]);
    }

    #[test]
    fn returns_first_file_alphabetically_when_multiple_options_are_present() {
        let dir = testdir!();

        let file_name = "file";

        let first_file = create_test_file(&dir.join(format!("{}.a", file_name)), "");
        let _second_file = create_test_file(&dir.join(format!("{}.b", file_name)), "");

        assert_eq!(dir.get_first_child_named(file_name).unwrap(), first_file);
    }
}
