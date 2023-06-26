use crate::{
    entry::{Entry, EntryFromNamedPath},
    entry_set::EntrySet,
    FileSystemError, CONTENTS_DIRECTORY, MODULE, TYPES_DIRECTORY,
};
use std::path::PathBuf;

use typed_builder::TypedBuilder;

/// A representation of a file system, meant to build Modules from.
#[derive(TypedBuilder, Debug)]
pub struct FileSystem {
    /// The root directory this FileSystem was built from.
    root_directory: PathBuf,
    // The module information
    module: Entry,
    // An optional set of entries that define types.
    #[builder(default, setter(strip_option))]
    types: Option<EntrySet>,
    // An optional set of entries that define contents.
    #[builder(default, setter(strip_option))]
    contents: Option<EntrySet>,
}

impl TryFrom<PathBuf> for FileSystem {
    type Error = FileSystemError;

    fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
        if value.is_file() {
            return Err(FileSystemError::ExpectedDirectory(value.into_boxed_path()));
        }
        let module_entry = value.has_entry_named(MODULE.to_string());
        if module_entry.is_none() {
            return Err(FileSystemError::MissingRequiredEntry(MODULE.to_string()));
        } else {
            let types_entry_set = value.join(TYPES_DIRECTORY).to_entry_set();
            let contents_entry_set = value.join(CONTENTS_DIRECTORY).to_entry_set();
        }
        todo!()
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
    fn fails_if_no_module_entry_exists() {
        let dir = testdir!();

        assert_eq!(
            FileSystem::try_from(dir).unwrap_err(),
            FileSystemError::MissingRequiredEntry(MODULE.to_string())
        )
    }
}
