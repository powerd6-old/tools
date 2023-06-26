use self::{json::Json, text::Text, yaml::Yaml};
use crate::{FileDataError, FileTypeDataReader};
use serde_json::Value;
use std::{ffi::OsStr, ops::Deref, path::Path};
use strum_macros::{EnumString, EnumVariantNames};

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
            FileType::JSON => Json::try_read_file(path),
            FileType::YAML => Yaml::try_read_file(path),
            FileType::TEXT => Text::try_read_file(path),
        }
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

pub(crate) mod json;
pub(crate) mod text;
pub(crate) mod yaml;
