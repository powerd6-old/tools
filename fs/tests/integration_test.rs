use std::path::PathBuf;
use testdir::testdir;

use fs::{Entry, EntrySet, FileSystem, FileSystemError};

fn create_file(path: &PathBuf) -> PathBuf {
    std::fs::write(path, "").expect("File was created correctly");
    path.to_path_buf()
}

fn create_directory(path: &PathBuf) -> PathBuf {
    std::fs::create_dir(&path).expect("Directory was created correctly");
    path.to_path_buf()
}

#[test]
fn it_fails_on_non_directory() {
    let dir: PathBuf = testdir!();
    let empty_file = create_file(&dir.join("some.file"));

    assert_eq!(
        FileSystem::try_from(empty_file).unwrap_err(),
        FileSystemError::InvalidPath
    );
}

#[test]
fn it_fails_on_empty_directory() {
    let dir: PathBuf = testdir!();
    let empty_dir = create_directory(&dir.join("empty/"));

    assert_eq!(
        FileSystem::try_from(empty_dir).unwrap_err(),
        FileSystemError::MissingRequiredEntry
    );
}

#[test]
fn it_works_with_only_module_file() {
    let dir: PathBuf = testdir!();
    let module_file = create_file(&dir.join("module.json"));

    assert_eq!(
        FileSystem::try_from(dir.clone()).unwrap(),
        FileSystem::new(dir, Entry::File(module_file))
    );
}

#[test]
fn it_works_with_only_module_as_directory() {
    let dir: PathBuf = testdir!();
    let module_dir = create_directory(&dir.join("module"));
    let module_root = create_file(&module_dir.join("_.json"));
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
    let module_file = create_file(&dir.join("module.json"));
    let types_dir = create_directory(&dir.join("types"));
    let type_a_file = create_file(&types_dir.join("a.json"));
    let type_b_dir = create_directory(&types_dir.join("b"));
    let type_b_root_file = create_file(&type_b_dir.join("_.json"));
    let type_b_extra_file = create_file(&type_b_dir.join("description.txt"));

    assert_eq!(
        FileSystem::try_from(dir.clone()).unwrap(),
        FileSystem::new(dir, Entry::File(module_file)).with_types(EntrySet::new(
            types_dir.to_path_buf(),
            vec![
                Entry::File(type_a_file),
                Entry::Directory {
                    root_file: type_b_root_file,
                    extra_files: vec![type_b_extra_file]
                }
            ]
        ))
    );
}

#[test]
fn it_works_with_contents() {
    let dir: PathBuf = testdir!();
    let module_file = create_file(&dir.join("module.json"));
    let contents_dir = create_directory(&dir.join("contents"));
    let content_a_file = create_file(&contents_dir.join("a.json"));
    let content_b_dir = create_directory(&contents_dir.join("b"));
    let content_b_root_file = create_file(&content_b_dir.join("_.json"));
    let content_b_extra_file = create_file(&content_b_dir.join("description.txt"));

    assert_eq!(
        FileSystem::try_from(dir.clone()).unwrap(),
        FileSystem::new(dir, Entry::File(module_file)).with_contents(EntrySet::new(
            contents_dir.to_path_buf(),
            vec![
                Entry::File(content_a_file),
                Entry::Directory {
                    root_file: content_b_root_file,
                    extra_files: vec![content_b_extra_file]
                }
            ]
        ))
    );
}
