use std::{
    fs::read_dir,
    path::{Path, PathBuf},
};

pub trait PathUtils {
    /// A helper function that maps all objects inside a directory into a `PathBuf` iterator
    fn get_children(&self) -> Vec<PathBuf>;
    /// Finds the first file (ordered alphabetically) in a directory with a specific filename, regardless of it's extension
    fn get_first_child_named(&self, name: &str) -> Option<PathBuf>;
    /// Checks if a file has a certain name, regardless of it's extension
    fn is_file_named(&self, file_name: &str) -> bool;
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
}
