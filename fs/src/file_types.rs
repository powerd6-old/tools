use std::{ffi::OsStr, path::Path};

use serde_json::Value;

use crate::FileSystemError;

pub enum FileType {
    JSON,
    YAML,
    TEXT,
    // IMAGE,
    // ETC
}

impl FileType {
    pub fn try_read_file(&self, path: &Path) -> Result<Value, FileSystemError> {
        match self {
            FileType::JSON => Json.read_file(path),
            FileType::YAML => Yaml.read_file(path),
            FileType::TEXT => Text.read_file(path),
        }
    }
}

struct Json;
pub mod json;

struct Yaml;
pub mod yaml;

struct Text;
pub mod text;

pub trait FileTypeReader {
    fn read_file(&self, path: &Path) -> Result<Value, FileSystemError>;
}

pub trait DetectFileTypes {
    fn get_file_type(&self) -> Result<FileType, FileSystemError>;
}

impl DetectFileTypes for Path {
    fn get_file_type(&self) -> Result<FileType, FileSystemError> {
        match self.extension().and_then(OsStr::to_str) {
            Some(extension) => match extension {
                "json" => Ok(FileType::JSON),
                "yaml" | "yml" => Ok(FileType::YAML),
                "txt" | "md" | "hjs" => Ok(FileType::TEXT),
                _ => Err(FileSystemError::UnrecognizableFileType),
            },
            None => Err(FileSystemError::UnrecognizableFileType),
        }
    }
}
