use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;
use thiserror::Error;

const DESCRIPTION: &str = "description";

// A identifier string. Must be unique within it's context.
#[derive(Debug, Eq, Hash, PartialEq, Deserialize, Clone, PartialOrd, Ord, Serialize)]
pub struct Identifier(String);

impl From<String> for Identifier {
    fn from(value: String) -> Self {
        Identifier(value)
    }
}

/// A generic object that contains data
pub type JsonObject = HashMap<String, Value>;

/// The errors that can happen when constructing a Module
#[derive(Error, Debug)]
pub enum ModuleError {
    #[error("unable to build element")]
    UnableToBuildElement(#[from] Box<dyn Error>),
    #[error("received value is not an object")]
    NotAnObject,
    #[error("missing required field `{0}`")]
    MissingRequiredField(String),
}

pub mod module;
pub mod module_type;
