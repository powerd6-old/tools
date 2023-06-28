use std::collections::BTreeMap;

use fs::file_system::FileSystem;
use fs_data::EntryData;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use url::Url;

use crate::{module_type::ModuleType, ModuleError};

/// A document that contains information for a powerd6 module.
///
/// While this object does not perform validation on it's own,
/// it creates an uniform structure to do so.
#[derive(Serialize, Deserialize)]
pub struct Module {
    /// The title of the module.
    title: String,
    /// The human-readable description of what the module contains.
    description: String,
    /// A hyperlink to the where the module is hosted.
    source: Url,
    /// A collection of types that are defined in this module.
    #[serde(skip_serializing_if = "Option::is_none")]
    types: Option<BTreeMap<String, ModuleType>>,
    /// A collection of contents defined in this module, the keys of the map are the unique identifiers of the content pieces.
    #[serde(skip_serializing_if = "Option::is_none")]
    contents: Option<BTreeMap<String, BTreeMap<String, Value>>>,
}

impl TryFrom<FileSystem> for Module {
    type Error = ModuleError;

    fn try_from(filesystem: FileSystem) -> Result<Self, Self::Error> {
        let module = filesystem.module.try_get_data();
        todo!()
    }
}
