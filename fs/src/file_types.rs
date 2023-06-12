use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};

use serde_json::Value;

use crate::FileSystemError;

pub enum FileType {
    JSON,
    YAML,
    TEXT,
    // IMAGE,
    // ETC
}

pub trait FileTypes {
    fn get_file_type(&self) -> Result<FileType, FileSystemError>;
}

impl FileTypes for Path {
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

impl FileType {
    pub fn read_file(&self, path: &Path) -> Result<Value, FileSystemError> {
        todo!("Based on the self type, use a different implementation to load the file and convert it into a Value.")
    }
}
