use std::fs::{self};

use crate::FileSystemError;

use super::FileTypeReader;

impl FileTypeReader for super::Text {
    fn read_file(
        &self,
        path: &std::path::Path,
    ) -> Result<serde_json::Value, crate::FileSystemError> {
        fs::read_to_string(path)
            .map(serde_json::Value::String)
            .map_err(|e| FileSystemError::UnableToOpenFile(Box::new(e)))
    }
}
