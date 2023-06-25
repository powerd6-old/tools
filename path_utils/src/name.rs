use std::{fmt::Debug, ops::Deref, path::Path};

use tracing::{debug, instrument};

pub trait NamePaths {
    /// Checks if a file has a certain name, regardless of it's extension
    fn is_named(&self, file_name: &str) -> bool;
    /// Gets the file or directory name, excluding any file extension
    fn get_name_without_extension(&self) -> String;
}

impl<T: AsRef<Path> + Debug> NamePaths for T {
    fn is_named(&self, name: &str) -> bool {
        let path: &Path = self.deref().as_ref();
        path.get_name_without_extension().eq(name)
    }

    #[instrument]
    fn get_name_without_extension(&self) -> String {
        let path: &Path = self.deref().as_ref();
        let name = path
            .file_name()
            .expect("Path should be a directory or a file and always have a name")
            .to_str()
            .expect("Path name should be a valid UTF-8 String");
        match path.extension() {
            Some(extension) => {
                debug!("Path has an extension, and it will be removed");
                name.replace(
                    extension
                        .to_str()
                        .expect("Extension should be a valid UTF-8 String"),
                    "",
                )
                .trim_end_matches('.')
                .to_string()
            }
            None => {
                debug!("Path did not have an extension");
                name.to_string()
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::{create_test_directory, create_test_file};

    use super::*;
    use pretty_assertions::assert_eq;
    use testdir::testdir;

    #[test]
    fn file_names_are_extracted_without_extension() {
        let dir = testdir!();

        let first = create_test_file(&dir.join("first.json"), "");
        let second = create_test_file(&dir.join("second.yaml"), "");

        assert_eq!(first.get_name_without_extension(), "first");
        assert_eq!(second.get_name_without_extension(), "second");
    }

    #[test]
    fn directory_names_are_extracted_unchanged() {
        let dir = testdir!();

        let first = create_test_directory(&dir.join("first"));
        let second = create_test_directory(&dir.join("second"));

        assert_eq!(first.get_name_without_extension(), "first");
        assert_eq!(second.get_name_without_extension(), "second");
    }

    #[test]
    fn file_names_are_correct() {
        let dir = testdir!();

        let first_json = create_test_file(&dir.join("first.json"), "");
        let first_yaml = create_test_file(&dir.join("first.yaml"), "");
        let second = create_test_file(&dir.join("second.json"), "");

        assert!(first_json.is_named("first"));
        assert!(first_yaml.is_named("first"));
        assert!(second.is_named("second"));
    }

    #[test]
    fn directory_names_are_correct() {
        let dir = testdir!();

        let first = create_test_directory(&dir.join("first"));
        let second = create_test_directory(&dir.join("second"));

        assert!(first.is_named("first"));
        assert!(second.is_named("second"));
    }
}
