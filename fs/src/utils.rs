use std::{
    fs::read_dir,
    path::{Path, PathBuf},
};

/// A helper function that maps all objects inside a directory into a `PathBuf` iterator
pub fn get_paths_in_directory(path: &Path) -> impl Iterator<Item = PathBuf> {
    read_dir(path)
        .into_iter()
        .flatten()
        .filter_map(|e| e.ok())
        .map(|e| e.path())
}

/// Finds the first file (ordered alphabetically) in a directory with a specific filename, regardless of it's extension
pub fn get_files_with_name(path: &Path, name: &str) -> Option<PathBuf> {
    get_paths_in_directory(path)
        .filter(|p| p.is_file())
        .find(|f| is_file_name(f, name))
}

/// Checks if a file has a certain name, regardless of it's extension
pub fn is_file_name(path: &Path, file_name: &str) -> bool {
    path.file_name()
        .map(|n| String::from(n.to_str().expect("File names must exist")))
        .map_or(false, |x| x.starts_with(file_name))
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

        assert_eq!(
            get_paths_in_directory(&dir).collect::<Vec<PathBuf>>(),
            [file, sub_directory]
        )
    }

    #[test]
    fn it_detects_file_names_correctly() {
        let dir: PathBuf = testdir!();

        let file_a_txt = create_file(&dir.join("a.txt"));
        let file_b_txt = create_file(&dir.join("b.txt"));
        let file_a_json = create_file(&dir.join("a.json"));

        assert!(is_file_name(&file_a_txt, "a"));
        assert!(is_file_name(&file_b_txt, "b"));
        assert!(is_file_name(&file_a_json, "a"));
    }

    #[test]
    fn it_finds_files_by_name() {
        let dir: PathBuf = testdir!();

        let file_a_json = create_file(&dir.join("a.json"));
        let file_b_json = create_file(&dir.join("b.json"));

        assert_eq!(get_files_with_name(&dir, "a").unwrap(), file_a_json);
        assert_eq!(get_files_with_name(&dir, "b").unwrap(), file_b_json);
    }

    #[test]
    fn it_finds_the_first_file_when_multiple_share_the_same_name() {
        let dir: PathBuf = testdir!();

        let file_a_json = create_file(&dir.join("a.json"));
        let _file_a_yaml = create_file(&dir.join("a.yaml"));

        assert!(get_files_with_name(&dir, "a").is_some());
        assert_eq!(get_files_with_name(&dir, "a").unwrap(), file_a_json);
    }
}
