use pretty_assertions::assert_eq;
use std::path::PathBuf;
use testdir::testdir;

use fs::{
    sorted::Sorted, Entry, EntrySet, FileSystem, FileSystemError, CONTENTS_DIRECTORY, MODULE,
    RENDERING_DIRECTORY, TYPES_DIRECTORY, UNDERSCORE_FILE_NAME,
};

fn create_file(path: &PathBuf) -> PathBuf {
    std::fs::write(path, "").expect("File should be created correctly");
    path.to_path_buf()
}

fn create_directory(path: &PathBuf) -> PathBuf {
    std::fs::create_dir(path).expect("Directory should be created correctly");
    path.to_path_buf()
}

#[test]
fn it_fails_on_inexistent_path() {
    let dir: PathBuf = testdir!();

    let missing_path = dir.join("missing-path");

    assert_eq!(
        FileSystem::try_from(missing_path.clone())
            .unwrap_err()
            .to_string(),
        FileSystemError::InvalidPath(missing_path.into()).to_string()
    );
}

#[test]
fn it_fails_on_non_directory() {
    let dir: PathBuf = testdir!();
    let empty_file = create_file(&dir.join("some.file"));

    assert_eq!(
        FileSystem::try_from(empty_file.clone())
            .unwrap_err()
            .to_string(),
        FileSystemError::InvalidPath(empty_file.into()).to_string()
    );
}

#[test]
fn it_fails_on_empty_directory() {
    let dir: PathBuf = testdir!();
    let empty_dir = create_directory(&dir.join("empty/"));

    assert_eq!(
        FileSystem::try_from(empty_dir).unwrap_err().to_string(),
        FileSystemError::MissingRequiredEntry(MODULE.to_string()).to_string()
    );
}

#[test]
fn it_works_with_only_module_file() {
    let dir: PathBuf = testdir!();
    let module_file = create_file(&dir.join(format!("{}.json", MODULE)));

    assert_eq!(
        FileSystem::try_from(dir.clone()).unwrap(),
        FileSystem::new(dir, Entry::File(module_file))
    );
}

#[test]
fn it_works_with_only_module_as_directory() {
    let dir: PathBuf = testdir!();
    let module_dir = create_directory(&dir.join(MODULE));
    let module_root = create_file(&module_dir.join(format!("{}.json", UNDERSCORE_FILE_NAME)));
    let module_extra_file = create_file(&module_dir.join("description.txt"));

    assert_eq!(
        FileSystem::try_from(dir.clone()).unwrap(),
        FileSystem::new(
            dir,
            Entry::Directory {
                root_file: module_root,
                extra_files: vec![module_extra_file]
            }
        )
    );
}

#[test]
fn it_works_with_types() {
    let dir: PathBuf = testdir!();
    let module_file = create_file(&dir.join(format!("{}.json", MODULE)));
    let types_dir = create_directory(&dir.join(TYPES_DIRECTORY));
    let type_a_file = create_file(&types_dir.join("a.json"));
    let type_b_dir = create_directory(&types_dir.join("b"));
    let type_b_root_file = create_file(&type_b_dir.join(format!("{}.json", UNDERSCORE_FILE_NAME)));
    let type_b_extra_file = create_file(&type_b_dir.join("description.txt"));
    let type_b_rendering_dir = create_directory(&type_b_dir.join(RENDERING_DIRECTORY));
    let type_b_rendering_txt_file = create_file(&type_b_rendering_dir.join("txt.hjs"));
    let type_b_rendering_md_file = create_file(&type_b_rendering_dir.join("md.hjs"));
    let type_c_dir = create_directory(&types_dir.join("c"));
    let type_c_root_file = create_file(&type_c_dir.join(format!("{}.json", UNDERSCORE_FILE_NAME)));
    let type_c_extra_file = create_file(&type_c_dir.join("description.txt"));

    assert_eq!(
        FileSystem::try_from(dir.clone()).unwrap().sorted(),
        FileSystem::new(dir, Entry::File(module_file))
            .with_types(EntrySet::new(
                types_dir,
                vec![
                    Entry::File(type_a_file),
                    Entry::RenderingDirectory {
                        root_file: type_b_root_file,
                        extra_files: vec![type_b_extra_file],
                        rendering_files: vec![type_b_rendering_txt_file, type_b_rendering_md_file],
                    },
                    Entry::Directory {
                        root_file: type_c_root_file,
                        extra_files: vec![type_c_extra_file],
                    },
                ]
            ))
            .sorted()
    );
}

#[test]
fn it_works_with_contents() {
    let dir: PathBuf = testdir!();
    let module_file = create_file(&dir.join(format!("{}.json", MODULE)));
    let contents_dir = create_directory(&dir.join(CONTENTS_DIRECTORY));
    let content_a_file = create_file(&contents_dir.join("a.json"));
    let content_b_dir = create_directory(&contents_dir.join("b"));
    let content_b_root_file =
        create_file(&content_b_dir.join(format!("{}.json", UNDERSCORE_FILE_NAME)));
    let content_b_extra_file = create_file(&content_b_dir.join("description.txt"));
    let content_b_c_dir = create_directory(&content_b_dir.join("c"));
    let content_b_c_file = create_file(&content_b_c_dir.join("b-c.json"));

    assert_eq!(
        FileSystem::try_from(dir.clone()).unwrap().sorted(),
        FileSystem::new(dir, Entry::File(module_file))
            .with_contents(EntrySet::new(
                contents_dir,
                vec![
                    Entry::File(content_a_file),
                    Entry::Directory {
                        root_file: content_b_root_file,
                        extra_files: vec![content_b_extra_file]
                    },
                    Entry::File(content_b_c_file),
                ]
            ))
            .sorted()
    );
}
