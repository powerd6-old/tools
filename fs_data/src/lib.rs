use file_data::FileData;
use fs::entry::Entry;
use path_utils::name::NamePaths;
use serde_json::Value;
use std::{collections::HashMap, error::Error, path::PathBuf};
use thiserror::Error;

/// The errors that can happen when reading data from an Entry
#[derive(Error, Debug)]
pub enum FileSystemDataError {
    #[error(
        "the underscore file is not a valid object, and therefore can't be extended with extra files"
    )]
    UnableToExtendRootFile,
    #[error("unable to read the file")]
    UnableToReadFile(#[source] Box<dyn Error>),
    #[error("unable to serialize the result")]
    UnableToSerializeResult(#[source] Box<dyn Error>),
}

pub trait EntryData {
    /// Attempts to read the data into a valid format
    fn try_get_data(&self) -> Result<Value, FileSystemDataError>;
}

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
