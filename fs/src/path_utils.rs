use std::{
    fs::read_dir,
    path::{Component, Path, PathBuf},
};

use pathdiff::diff_paths;
use tracing::{debug, instrument};

use crate::UNDERSCORE_FILE_NAME;

pub trait PathUtils {
    /// A helper function that maps all objects inside a directory into a `PathBuf` iterator
    fn get_children(&self) -> Vec<PathBuf>;
    /// Finds the first file (ordered alphabetically) in a directory with a specific filename, regardless of it's extension
    fn get_first_child_named(&self, name: &str) -> Option<PathBuf>;
    /// Checks if a file has a certain name, regardless of it's extension
    fn is_file_named(&self, file_name: &str) -> bool;
    /// Gets the file or directory name, excluding any file extension
    fn get_name_without_extension(&self) -> String;
    /// Returns a String identifier for a path, using the relative path between it and a base path
    fn get_id_from_path(&self, base_path: &Path) -> Option<String>;
}

impl PathUtils for Path {
    fn get_children(&self) -> Vec<PathBuf> {
        read_dir(self)
            .into_iter()
            .flatten()
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .collect()
    }

    fn get_first_child_named(&self, name: &str) -> Option<PathBuf> {
        self.get_children()
            .iter()
            .filter(|p| p.is_file())
            .find(|f| f.is_file_named(name))
            .cloned()
    }

    fn is_file_named(&self, file_name: &str) -> bool {
        self.file_name()
            .map(|n| String::from(n.to_str().expect("File names must exist")))
            .map_or(false, |x| x.starts_with(file_name))
    }

    fn get_name_without_extension(&self) -> String {
        let name = self
            .file_name()
            .expect("Files and Directories should have a name")
            .to_str()
            .expect("Name should be valid string");
        match self.extension() {
            Some(extension) => name
                .replace(
                    extension
                        .to_str()
                        .expect("Extension should be a valid string"),
                    "",
                )
                .trim_end_matches('.')
                .to_string(),
            None => name.to_string(),
        }
    }

    #[instrument]
    fn get_id_from_path(&self, base_path: &Path) -> Option<String> {
        diff_paths(self, base_path).map(|p| {
            let mut result = p
                .components()
                .filter(|c| c.ne(&Component::CurDir))
                .filter(|c| c.ne(&Component::ParentDir))
                .filter(|c| {
                    !c.as_os_str()
                        .to_str()
                        .expect("path fragments should be valid strings")
                        .starts_with(UNDERSCORE_FILE_NAME)
                })
                .map(|c| {
                    c.as_os_str()
                        .to_str()
                        .expect("path fragments should be valid strings")
                })
                .collect::<Vec<&str>>()
                .join("_");
            if let Some(extension) = p.extension() {
                result = result.replace(
                    &format!(
                        ".{}",
                        extension
                            .to_str()
                            .expect("extensions should be valid strings")
                    ),
                    "",
                )
            }
            debug!(
                result,
                "Create id from based on {:?} from {:?}", self, base_path
            );
            result
        })
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use pretty_assertions::assert_eq;
    use testdir::testdir;

    fn create_file(path: &PathBuf) -> PathBuf {
        std::fs::write(path, "").expect("File was created correctly");
        path.to_path_buf()
    }
    fn create_directory(path: &PathBuf) -> PathBuf {
        std::fs::create_dir(path).expect("Directory was created correctly");
        path.to_path_buf()
    }

    #[test]
    fn files_and_subdirectories_are_returned() {
        let dir = testdir!();

        let file = create_file(&dir.join("a.json"));
        let sub_directory = create_directory(&dir.join("sub_directory"));

        assert_eq!(dir.get_children().sort(), [file, sub_directory].sort())
    }

    #[test]
    fn it_detects_file_names_correctly() {
        let dir: PathBuf = testdir!();

        let file_a_txt = create_file(&dir.join("a.txt"));
        let file_b_txt = create_file(&dir.join("b.txt"));
        let file_a_json = create_file(&dir.join("a.json"));

        assert!(file_a_txt.is_file_named("a"));
        assert!(file_b_txt.is_file_named("b"));
        assert!(file_a_json.is_file_named("a"));
    }

    #[test]
    fn it_returns_names_without_extensions_correctly() {
        let dir: PathBuf = testdir!();

        let file_a_txt = create_file(&dir.join("a.txt"));
        let file_b_txt = create_file(&dir.join("b.txt"));
        let file_a_json = create_file(&dir.join("a.json"));

        assert_eq!(file_a_txt.get_name_without_extension(), "a");
        assert_eq!(file_b_txt.get_name_without_extension(), "b");
        assert_eq!(file_a_json.get_name_without_extension(), "a");
    }

    #[test]
    fn it_finds_files_by_name() {
        let dir: PathBuf = testdir!();

        let file_a_json = create_file(&dir.join("a.json"));
        let file_b_json = create_file(&dir.join("b.json"));

        assert_eq!(dir.get_first_child_named("a").unwrap(), file_a_json);
        assert_eq!(dir.get_first_child_named("b").unwrap(), file_b_json);
    }

    #[test]
    fn it_finds_the_first_file_when_multiple_share_the_same_name() {
        let dir: PathBuf = testdir!();

        let file_a_json = create_file(&dir.join("a.json"));
        let _file_a_yaml = create_file(&dir.join("a.yaml"));

        assert!(dir.get_first_child_named("a").is_some());
        assert_eq!(dir.get_first_child_named("a").unwrap(), file_a_json);
    }

    #[test]
    fn sibling_files_have_correct_nested_id() {
        let dir: PathBuf = testdir!();
        let base_path = create_file(&dir.join("a.json"));
        let sibling_path = create_file(&dir.join("something.json"));

        assert_eq!(
            sibling_path.get_id_from_path(&base_path).unwrap(),
            String::from("something")
        );
    }

    #[test]
    fn underscore_files_have_correct_nested_id() {
        let dir: PathBuf = testdir!();
        let base_path = create_directory(&dir.join("a"));
        let nested_path = create_directory(&base_path.join("b"));
        let sibling_path = create_file(&nested_path.join(format!("{}.json", UNDERSCORE_FILE_NAME)));

        assert_eq!(
            sibling_path.get_id_from_path(&base_path).unwrap(),
            String::from("b")
        );
    }

    #[test]
    fn deeply_nested_files_have_correct_nested_id() {
        let dir: PathBuf = testdir!();
        let base_path = create_directory(&dir.join("a"));
        let nested_path = create_directory(&base_path.join("b"));
        let another_nested_path = create_directory(&nested_path.join("c"));
        let sibling_path = create_file(&another_nested_path.join("something.yaml"));

        assert_eq!(
            sibling_path.get_id_from_path(&base_path).unwrap(),
            String::from("b_c_something")
        );
    }
}
