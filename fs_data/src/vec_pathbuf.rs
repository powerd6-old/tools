use std::{collections::HashMap, path::PathBuf};

use file_data::FileData;
use path_utils::name::NamePaths;
use serde_json::Value;

use crate::{EntryData, FileSystemDataError};

impl EntryData for Vec<PathBuf> {
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

#[cfg(test)]
mod tests {

    use super::*;
    use path_utils::create_test_file;
    use pretty_assertions::assert_eq;
    use serde_json::json;
    use testdir::testdir;

    #[test]
    fn each_path_becomes_an_entry() {
        let dir = testdir!();

        let file_a = create_test_file(&dir.join("a.txt"), "a");
        let file_b = create_test_file(&dir.join("b.txt"), "b");
        let file_c = create_test_file(&dir.join("c.txt"), "c");
        let file_d = create_test_file(&dir.join("d.txt"), "d");

        let vect = vec![file_a, file_b, file_c, file_d];

        assert_eq!(
            vect.try_get_data().unwrap(),
            json!({
                "a": "a",
                "b": "b",
                "c": "c",
                "d": "d",
            })
        );
    }
}
