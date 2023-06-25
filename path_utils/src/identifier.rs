use std::{
    fmt::Debug,
    ops::Deref,
    path::{Component, Path},
};

use pathdiff::diff_paths;
use tracing::{debug, instrument};

use crate::name::NamePaths;

pub trait IdentifierPaths {
    /// Returns a String identifier for a path, using the relative path between it and a base path
    fn get_id_from_path(&self, base: &Path) -> Option<String>;
}

impl<T: AsRef<Path> + Debug> IdentifierPaths for T {
    #[instrument]
    fn get_id_from_path(&self, base: &Path) -> Option<String> {
        let mut path: &Path = self.deref().as_ref();

        if path.is_named("_") {
            path = path.ancestors().nth(1).expect("Path ends with a file named `_` therefore the second ancestor should be a valid path to it's parent");
            debug!("The path ended with a file named `_`. Using the ancestor path to create an identifier instead.")
        }

        diff_paths(path, base).map(|p| {
            let mut result = p
                .components()
                .filter(|c| c.ne(&Component::CurDir))
                .filter(|c| c.ne(&Component::ParentDir))
                .map(|c| {
                    c.as_os_str()
                        .to_str()
                        .expect("Path fragment should be valid UTF-8 String")
                })
                .collect::<Vec<&str>>()
                .join("_");
            if let Some(extension) = p.extension() {
                result = result.replace(
                    &format!(
                        ".{}",
                        extension
                            .to_str()
                            .expect("Extensions should be a valid UTF-8 String")
                    ),
                    "",
                )
            }
            result
        })
    }
}

#[cfg(test)]
mod tests {

    use crate::{create_test_directory, create_test_file};

    use super::*;
    use pretty_assertions::assert_eq;
    use testdir::testdir;

    #[test]
    fn getting_identifier_of_sibling_files_returns_the_file_name() {
        let dir = testdir!();

        let base_file = create_test_file(&dir.join("a.json"), "");
        let sibling_file = create_test_file(&dir.join("sibling.json"), "");

        assert_eq!(
            sibling_file.get_id_from_path(&base_file).unwrap(),
            String::from("sibling")
        )
    }
    #[test]
    fn getting_identifier_for_file_named_underscore_returns_its_parent_name() {
        let dir = testdir!();

        let directory = create_test_directory(&dir.join("a"));
        let underscore_file = create_test_file(&directory.join("_.json"), "");

        assert_eq!(
            underscore_file.get_id_from_path(&dir).unwrap(),
            String::from("a")
        )
    }
    #[test]
    fn identifier_of_deeply_nested_files_include_all_levels() {
        let dir = testdir!();

        let first_level = create_test_directory(&dir.join("a"));
        let second_level = create_test_directory(&first_level.join("b"));
        let nested_file = create_test_file(&second_level.join("filename.json"), "");

        assert_eq!(
            nested_file.get_id_from_path(&dir).unwrap(),
            String::from("a_b_filename")
        )
    }
}
