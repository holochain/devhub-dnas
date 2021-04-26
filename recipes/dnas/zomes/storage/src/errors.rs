use hdk::prelude::{WasmError, Element};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RuntimeError {
    #[error("Entry not found")]
    EntryNotFound,

    #[error("Failed to deserialize entry: {0:?}")]
    DeserializationError(Element),
}

impl From<RuntimeError> for WasmError  {
    fn from(error: RuntimeError) -> Self {
        WasmError::Guest(format!("{}", error))
    }
}
