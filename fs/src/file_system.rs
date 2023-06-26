use crate::{
    entry::{Entry, EntryFromNamedPath},
    entry_set::{EntrySet, EntrySetFromPath},
    FileSystemError, CONTENTS_DIRECTORY, MODULE, TYPES_DIRECTORY,
};
use std::path::PathBuf;

/// A representation of a file system, meant to build Modules from.
#[derive(Debug)]
pub struct FileSystem {
    /// The root directory this FileSystem was built from.
    root_directory: PathBuf,
    // The module information
    module: Entry,
    // An optional set of entries that define types.
    types: Option<EntrySet>,
    // An optional set of entries that define contents.
    contents: Option<EntrySet>,
}

impl TryFrom<PathBuf> for FileSystem {
    type Error = FileSystemError;

    fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
        if value.is_file() {
            return Err(FileSystemError::ExpectedDirectory(value.into_boxed_path()));
        }
        match value.has_entry_named(MODULE.to_string()) {
            None => Err(FileSystemError::MissingRequiredEntry(MODULE.to_string())),
            Some(module_entry) => {
                let types_entry_set = value.join(TYPES_DIRECTORY).to_entry_set();
                let contents_entry_set = value.join(CONTENTS_DIRECTORY).to_entry_set();

                Ok(FileSystem {
                    root_directory: value,
                    module: module_entry,
                    types: types_entry_set,
                    contents: contents_entry_set,
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::MODULE;

    use super::*;
    use path_utils::create_test_file;
    use pretty_assertions::assert_eq;
    use testdir::testdir;

    #[test]
    fn cannot_create_from_file() {
        let dir = testdir!();

        let file = create_test_file(&dir.join("file.json"), "");

        assert_eq!(
            FileSystem::try_from(file.clone()).unwrap_err(),
            FileSystemError::ExpectedDirectory(file.into_boxed_path())
        )
    }

    #[test]
    fn cannot_create_if_no_module_entry_exists() {
        let dir = testdir!();

        assert_eq!(
            FileSystem::try_from(dir).unwrap_err(),
            FileSystemError::MissingRequiredEntry(MODULE.to_string())
        )
    }

    #[test]
    fn creates_with_optional_types_and_contents() {
        todo!()
    }
}
