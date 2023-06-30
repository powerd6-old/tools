use std::{collections::HashMap, path::PathBuf};

use file_data::FileData;
use path_utils::name::NamePaths;
use serde_json::Value;

use crate::{EntryData, FileSystemDataError};

impl EntryData for Vec<PathBuf> {
    // TODO: implement tests
    fn try_get_data(&self) -> Result<Value, FileSystemDataError> {
        let mut result: HashMap<String, Value> = HashMap::new();
        for file in self {
            match file.try_read_file() {
                Ok(value) => {
                    result.insert(file.get_name_without_extension(), value);
                }
                Err(e) => return Err(FileSystemDataError::UnableToReadFile(e.into())),
            }
        }
        serde_json::to_value(result)
            .map_err(|e| FileSystemDataError::UnableToSerializeResult(e.into()))
    }
}
