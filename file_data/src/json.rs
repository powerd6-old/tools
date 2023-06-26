use std::{fs::File, io::BufReader};

use crate::{FileDataError, FileTypeDataReader};

pub struct JSON;

impl FileTypeDataReader for JSON {
    fn try_read_file(path: &std::path::Path) -> Result<serde_json::Value, crate::FileDataError> {
        match File::open(path) {
            Ok(file) => {
                let reader = BufReader::new(file);
                serde_json::from_reader(reader)
                    .map_err(|e| FileDataError::InvalidFileContents(e.into()))
            }
            Err(e) => Err(FileDataError::UnableToOpenFile(e.into())),
        }
    }
}
