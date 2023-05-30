use std::path::{Path, PathBuf};

use crate::path_utils::PathUtils;

/// A helper function that maps all objects inside a directory into a `PathBuf` iterator
pub fn get_paths_in_directory(path: &Path) -> impl Iterator<Item = PathBuf> {
    path.get_children().into_iter()
}
