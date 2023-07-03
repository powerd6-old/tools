use self::{json::Json, text::Text, yaml::Yaml};
use crate::{FileDataError, FileTypeDataReader};
use serde_json::Value;
use std::{ffi::OsStr, ops::Deref, path::Path};
use strum_macros::{EnumString, EnumVariantNames};
use tracing::instrument;

/// The list of valid file types.
///
/// These are not necessarily mapped to file formats or extensions, instead,
/// they refer to the mechanism related to reading, parsing, and de/serializing
/// it's contents into valid [JSON Value](serde_json::Value).
#[derive(Debug, EnumString, EnumVariantNames)]
pub enum FileType {
    JSON,
    YAML,
    TEXT,
    // IMAGE,
    // ETC
}

impl FileType {
    /// Allows a specific file format to be read, choosing the correct
    /// corresponding implementation.
    #[instrument]
    pub(crate) fn try_read_file(&self, path: &Path) -> Result<Value, FileDataError> {
        match self {
            FileType::JSON => Json::try_read_file(path),
            FileType::YAML => Yaml::try_read_file(path),
            FileType::TEXT => Text::try_read_file(path),
        }
    }
}

/// Allows files to be read into [JSON Value](serde_json::Value).
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

pub(crate) mod json;
pub(crate) mod text;
pub(crate) mod yaml;
