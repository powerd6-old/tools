use fs::entry::Entry;
use serde_json::Value;
use std::{collections::BTreeMap, error::Error};
use thiserror::Error;

/// The errors that can happen when constructing a Module.
#[derive(Error, Debug)]
pub enum ModuleError {
    #[error("received a value that was not an object")]
    NotAnObject(Box<Value>),
    #[error("unable to get the data for a required field")]
    UnableToGetRequiredData(#[source] Box<dyn Error>),
    #[error("a required field was missing")]
    MissingRequired(Box<str>),
    #[error("the field is not of the expected type")]
    IncompatibleFieldType(Box<Value>),
    #[error("Unable to create a valid identifier from entry: `{0:#?}`.")]
    InvalidIdentifier(Box<Entry>),
}

/// The key to the contents property.
pub const CONTENTS: &str = "contents";
/// The key to the types property.
pub const TYPES: &str = "types";
/// The key to the description property.
pub const DESCRIPTION: &str = "description";
/// The key to the schema property.
pub const SCHEMA: &str = "schema";
/// The key to the rendering property.
pub const RENDERING: &str = "rendering";
/// The key to the title property.
pub const TITLE: &str = "title";
/// The key to the source property.
pub const SOURCE: &str = "source";

type JsonMap = BTreeMap<String, Value>;

/// Handles a Module.
pub mod module;
/// Handles a Type inside a Module.
pub mod module_type;
