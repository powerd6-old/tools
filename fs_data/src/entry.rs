use file_data::FileData;
use fs::entry::Entry;
use serde_json::Value;
use tracing::instrument;

use crate::{EntryData, FileSystemDataError};

impl EntryData for Entry {
    #[instrument]
    fn try_get_data(&self) -> Result<Value, FileSystemDataError> {
        match self {
            Entry::File(file) => file
                .try_read_file()
                .map_err(|e| FileSystemDataError::UnableToReadFile(e.into())),
            Entry::Directory {
                root_file,
                extra_files,
            } => match root_file.try_read_file() {
                Ok(root_data) => {
                    if let Some(root_data_object) = root_data.as_object() {
                        let mut result = root_data_object.clone();
                        let extra_files_data = extra_files.try_get_data()?;
                        let extra_data = extra_files_data.as_object().expect("When a Vec of Path Buffers is transformed into data, the result is always a valid Object");
                        extra_data.iter().for_each(|(k, v)| {
                            result.insert(k.clone(), v.clone());
                        });
                        serde_json::to_value(result)
                            .map_err(|e| FileSystemDataError::UnableToSerializeResult(e.into()))
                    } else {
                        Err(FileSystemDataError::UnableToExtendRootFile)
                    }
                }
                Err(e) => Err(FileSystemDataError::UnableToReadFile(e.into())),
            },
            Entry::RenderingDirectory {
                root_file,
                extra_files,
                rendering_files,
            } => match root_file.try_read_file() {
                Ok(root_data) => {
                    if let Some(root_data_object) = root_data.as_object() {
                        let mut result = root_data_object.clone();
                        let extra_files_data = extra_files.try_get_data()?;
                        let extra_data = extra_files_data.as_object().expect("When a Vec of Path Buffers is transformed into data, the result is always a valid Object");
                        extra_data.iter().for_each(|(k, v)| {
                            result.insert(k.clone(), v.clone());
                        });
                        let rendering_files_data = rendering_files.try_get_data()?;
                        result.insert("rendering".to_string(), rendering_files_data);
                        serde_json::to_value(result)
                            .map_err(|e| FileSystemDataError::UnableToSerializeResult(e.into()))
                    } else {
                        Err(FileSystemDataError::UnableToExtendRootFile)
                    }
                }
                Err(e) => Err(FileSystemDataError::UnableToReadFile(e.into())),
            },
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use path_utils::{create_test_directory, create_test_file};
    use pretty_assertions::assert_eq;
    use serde_json::json;
    use testdir::testdir;

    #[test]
    fn file_gets_parsed_and_returned() {
        let dir = testdir!();

        let file = create_test_file(&dir.join("a.json"), r#"{"a":1}"#);

        assert_eq!(
            Entry::File(file).try_get_data().unwrap(),
            json!({
                "a": 1
            })
        )
    }

    #[test]
    fn directory_uses_extra_files_as_new_keys() {
        let dir = testdir!();

        let root_file = create_test_file(
            &dir.join("_.json"),
            r#"{
            "a": 1,
            "b": 0
        }"#,
        );
        let extra_file = create_test_file(&dir.join("b.txt"), "test");

        assert_eq!(
            Entry::Directory {
                root_file,
                extra_files: vec![extra_file]
            }
            .try_get_data()
            .unwrap(),
            json!({
                "a": 1,
                "b": "test"
            })
        )
    }

    #[test]
    fn rendering_directory_uses_extra_files_as_new_keys() {
        let dir = testdir!();

        let root_file = create_test_file(
            &dir.join("_.json"),
            r#"{
            "a": 1,
            "b": 0
        }"#,
        );
        let extra_file = create_test_file(&dir.join("b.txt"), "test");
        let rendering_dir = create_test_directory(&dir.join("rendering"));
        let rendering_file = create_test_file(&rendering_dir.join("md.hjs"), "");

        assert_eq!(
            Entry::RenderingDirectory {
                root_file,
                extra_files: vec![extra_file],
                rendering_files: vec![rendering_file]
            }
            .try_get_data()
            .unwrap(),
            json!({
                "a": 1,
                "b": "test",
                "rendering": {
                    "md": ""
                }
            })
        )
    }
}
