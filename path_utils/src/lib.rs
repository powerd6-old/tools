use std::path::{Path, PathBuf};

use children::ChildrenPaths;
use identifier::IdentifierPaths;
use name::NamePaths;

pub mod children;
pub mod identifier;
pub mod name;

pub trait PathUtils: ChildrenPaths + IdentifierPaths + NamePaths {}

/// Writes a file into the `path` with the given contents.
///
/// This is meant to be used **only in tests**
/// # Examples
///
/// ```
/// # use testdir::testdir;
/// # let dir = testdir!();
/// let test_file = create_test_file(&dir.join("file.txt"), "these are my contents");
/// assert_eq(fs::read_to_string(&test_file),"these are my contents");
/// ```
pub fn create_test_file(path: &PathBuf, contents: &str) -> PathBuf {
    std::fs::write(path, contents).expect("File should be created correctly");
    path.to_path_buf()
}

/// Creates an empty directory into the `path`.
///
/// This is meant to be used **only in tests**
/// # Examples
///
/// ```
/// # use testdir::testdir;
/// # let dir = testdir!();
/// let new_directory = create_test_directory(&dir.join("my_dir"));
/// ```
pub fn create_test_directory(path: &PathBuf) -> PathBuf {
    std::fs::create_dir(path).expect("Directory should be created correctly");
    path.to_path_buf()
}
