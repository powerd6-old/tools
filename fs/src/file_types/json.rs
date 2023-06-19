use std::{fs::File, io::BufReader};

use crate::FileSystemError;

use super::FileTypeReader;

impl FileTypeReader for super::Json {
    fn read_file(
        &self,
        path: &std::path::Path,
    ) -> Result<serde_json::Value, crate::FileSystemError> {
        match File::open(path) {
            Ok(file) => {
                let reader = BufReader::new(file);
                serde_json::from_reader(reader)
                    .map_err(|e| FileSystemError::UnableToOpenFile(Box::new(e)))
            }
            Err(e) => Err(FileSystemError::UnableToOpenFile(Box::new(e))),
        }
    }
}
