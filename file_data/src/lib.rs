use std::{ffi::OsStr, ops::Deref, path::Path};

use json::JSON;
use serde_json::Value;
use std::error::Error;
use strum::VariantNames;
use strum_macros::{EnumString, EnumVariantNames};
use text::TEXT;
use thiserror::Error;
use yaml::YAML;

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

#[derive(Debug, EnumString, EnumVariantNames)]
pub enum FileType {
    JSON,
    YAML,
    TEXT,
    // IMAGE,
    // ETC
}

impl FileType {
    pub fn try_read_file(&self, path: &Path) -> Result<Value, FileDataError> {
        match self {
            FileType::JSON => JSON::try_read_file(path),
            FileType::YAML => YAML::try_read_file(path),
            FileType::TEXT => TEXT::try_read_file(path),
        }
    }
}

pub trait FileData {
    /// Attempts to read the file into a valid JSON Value
    fn try_read_file(&self) -> Result<Value, FileDataError>;
}

impl<T: AsRef<Path>> FileData for T {
    fn try_read_file(&self) -> Result<Value, FileDataError> {
        let path: &Path = self.deref().as_ref();
        let file_type = self.try_get_file_type()?;
        file_type.try_read_file(path)
    }
}

pub trait FileDataType {
    /// Attempts to identify the FileType
    fn try_get_file_type(&self) -> Result<FileType, FileDataError>;
}

impl<T: AsRef<Path>> FileDataType for T {
    fn try_get_file_type(&self) -> Result<FileType, FileDataError> {
        let path = self.deref().as_ref();
        match path.extension().and_then(OsStr::to_str) {
            Some(extension) => match extension {
                "json" => Ok(FileType::JSON),
                "yaml" | "yml" => Ok(FileType::YAML),
                "txt" | "md" | "hjs" => Ok(FileType::TEXT),
                _ => Err(FileDataError::UnsupportedFileType(path.into())),
            },
            None => Err(FileDataError::UnsupportedFileType(path.into())),
        }
    }
}

pub mod json;
pub mod text;
pub mod yaml;
