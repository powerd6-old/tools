use std::{
    ops::Deref,
    path::{Path, PathBuf},
};

use typed_builder::TypedBuilder;

use crate::entry::Entry;

/// A collection of Entries contained within a directory.
/// This structure does not represent the number of levels each entry is nested at.
#[derive(TypedBuilder, Debug)]
pub struct EntrySet {
    base_path: PathBuf,
    #[builder(default)]
    entries: Vec<Entry>,
}

pub trait EntrySetFromPath {
    /// Create an EntrySet from a file or directory inside the path
    fn to_entry_set(&self) -> EntrySet;
}

impl<T: AsRef<Path>> EntrySetFromPath for T {
    fn to_entry_set(&self) -> EntrySet {
        let path: &Path = self.deref().as_ref();
        todo!()
    }
}
