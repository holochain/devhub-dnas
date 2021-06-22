use hdk::prelude::*;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RuntimeError {
    #[error("Entry not found")]
    EntryNotFound,

    #[error("Failed to deserialize entry: {0:?}")]
    DeserializationError(Element),

    #[error("Deserialized entry did not match entry hash: {0:?}")]
    DeserializationWrongEntryTypeError(EntryHash, EntryHash),
}

impl From<RuntimeError> for WasmError  {
    fn from(error: RuntimeError) -> Self {
        WasmError::Guest(format!("{}", error))
    }
}
