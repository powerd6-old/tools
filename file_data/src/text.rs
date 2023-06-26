use std::fs::{self};

use crate::{FileDataError, FileTypeDataReader};

pub struct TEXT;

impl FileTypeDataReader for TEXT {
    fn try_read_file(path: &std::path::Path) -> Result<serde_json::Value, crate::FileDataError> {
        fs::read_to_string(path)
            .map(serde_json::Value::String)
            .map_err(|e| FileDataError::UnableToOpenFile(e.into()))
    }
}
