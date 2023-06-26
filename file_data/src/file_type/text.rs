use crate::{FileDataError, FileTypeDataReader};
use std::fs::{self};

pub struct Text;

impl FileTypeDataReader for Text {
    fn try_read_file(path: &std::path::Path) -> Result<serde_json::Value, crate::FileDataError> {
        fs::read_to_string(path)
            .map(serde_json::Value::String)
            .map_err(|e| FileDataError::UnableToOpenFile(e.into()))
    }
}
