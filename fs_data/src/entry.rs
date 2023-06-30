use file_data::FileData;
use fs::entry::Entry;
use serde_json::Value;

use crate::{EntryData, FileSystemDataError};

impl EntryData for Entry {
    // TODO: implement tests
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
                        let extra_data = extra_files_data.as_object().expect("When a Vec of PathBufs is transformed into data, the result is always a valid Object");
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
                        let extra_data = extra_files_data.as_object().expect("When a Vec of PathBufs is transformed into data, the result is always a valid Object");
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
