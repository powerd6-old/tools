use std::{collections::BTreeMap, error::Error};

use serde_json::Value;
use thiserror::Error;

/// The errors that can happen when compiling or executing the rendering process for a Module.
#[derive(Error, Debug)]
pub enum RenderingError {
    #[error("no renderable types are present")]
    NoRenderableTypes,
    #[error("failed to register a rendering template")]
    FailedToRegisterTemplate(#[source] Box<dyn Error>),
    #[error(
        "tried to render the content, but it did not define the corresponding type (key `type`): {0:?}"
    )]
    ContentHasNoType(Box<BTreeMap<String, Value>>),
    #[error("failed to render the content piece")]
    FailedToRender(#[source] Box<dyn Error>),
}

const TYPE_KEY: &str = "type";

pub mod module;
pub mod renderer;
