use serde_json::Value;
use std::error::Error;
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

pub mod entry;
pub mod vec_path_buffer;
