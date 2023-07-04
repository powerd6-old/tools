use std::{collections::BTreeMap, error::Error};

use serde_json::Value;
use thiserror::Error;

/// The errors that can happen when compiling or executing the rendering process
/// for a Module.
#[derive(Error, Debug)]
pub enum RenderingError {
    #[error("no renderable types are present")]
    NoRenderableTypes,
    #[error("failed to register a rendering template for type `{0}` and format `{1}`")]
    FailedToRegisterTemplate(String, String, #[source] Box<dyn Error>),
    #[error(
        "tried to render the content, but it did not define the corresponding type (key `type`): {0:#?}"
    )]
    ContentHasNoType(Box<BTreeMap<String, Value>>),
    #[error("failed to render the content piece")]
    FailedToRender(#[source] Box<dyn Error>),
}

const TYPE_KEY: &str = "type";

/// Handles integration with the [Module](module::module::Module) type.
pub mod module;
/// Handles the rendering setup and logic.
pub mod renderer;
