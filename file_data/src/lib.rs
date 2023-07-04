use file_type::{FileDataType, FileType};
use serde_json::Value;
use std::error::Error;
use std::{ops::Deref, path::Path};
use strum::VariantNames;
use strum_macros::EnumIs;
use thiserror::Error;
use tracing::instrument;

/// The errors that can happen when reading a file into data.
#[derive(Error, Debug, EnumIs)]
pub enum FileDataError {
    #[error(
        "unsupported file type `{0}` (expected one of {:?})",
        FileType::VARIANTS
    )]
    UnsupportedFileType(Box<Path>),
    #[error("unable to open file `{0}`")]
    UnableToOpenFile(Box<Path>, #[source] Box<dyn Error>),
    #[error("the contents of the file `{0}` were invalid")]
    InvalidFileContents(Box<Path>, #[source] Box<dyn Error>),
}

/// Private trait that is implemented by specific file formats.
///
/// Similar to [`FileData`], but meant only to be used inside this crate.
pub(crate) trait FileTypeDataReader {
    fn try_read_file(path: &Path) -> Result<Value, FileDataError>;
}

/// Allows files to be read into [JSON Values](serde_json::Value).
///
/// # Example
/// ```
/// # use testdir::testdir;
/// # use path_utils::create_test_file;
/// # use file_data::FileData;
/// # use serde_json::json;
/// # let dir = testdir!();
/// let test_file = create_test_file(&dir.join("file.json"), "{\"a\": 1}");
/// assert_eq!(test_file.try_read_file().unwrap(), json!({"a":1}))
/// ```
pub trait FileData {
    /// Attempts to read the file into a valid JSON Value.
    fn try_read_file(&self) -> Result<Value, FileDataError>;
}

impl<T: AsRef<Path>> FileData for T {
    #[instrument(
        skip(self),
        fields(path=self.deref().as_ref().to_str().expect("Path should be a valid UTF-8 String."))
    )]
    fn try_read_file(&self) -> Result<Value, FileDataError> {
        let path: &Path = self.deref().as_ref();
        let file_type = self.try_get_file_type()?;
        file_type.try_read_file(path)
    }
}

/// Handles reading files and getting their values.
pub mod file_type;
