use file_type::{FileDataType, FileType};
use serde_json::Value;
use std::error::Error;
use std::{ops::Deref, path::Path};
use strum::VariantNames;
use thiserror::Error;
use tracing::instrument;

/// The errors that can happen when reading a file into data
#[derive(Error, Debug)]
pub enum FileDataError {
    #[error("unsupported file type (expected one of {:?})", FileType::VARIANTS)]
    UnsupportedFileType(Box<Path>),
    #[error("unable to open file")]
    UnableToOpenFile(#[source] Box<dyn Error>),
    #[error("the contents of the file were invalid")]
    InvalidFileContents(#[source] Box<dyn Error>),
}

pub(crate) trait FileTypeDataReader {
    fn try_read_file(path: &Path) -> Result<Value, FileDataError>;
}

pub trait FileData {
    /// Attempts to read the file into a valid JSON Value
    fn try_read_file(&self) -> Result<Value, FileDataError>;
}

impl<T: AsRef<Path>> FileData for T {
    #[instrument(skip(self), fields(path=self.deref().as_ref().to_str().expect("Path should be a valid UTF-8 String")))]
    fn try_read_file(&self) -> Result<Value, FileDataError> {
        let path: &Path = self.deref().as_ref();
        let file_type = self.try_get_file_type()?;
        file_type.try_read_file(path)
    }
}

pub mod file_type;
