use hdk::prelude::*;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum UtilsError {
    #[error("HDK raised error: {0:?}")]
    HDKError(WasmError),

    #[error("Entry not found for address: {0:?}")]
    EntryNotFoundError(EntryHash),

    #[error("Found multiple origin links for entry: {0:?}")]
    MultipleOriginsError(EntryHash),

    #[error("Failed to deserialize entry: {0:?}")]
    DeserializationError(Element),

    #[error("Deserialized entry did not match entry hash: {0:?}")]
    DeserializationWrongEntryTypeError(EntryHash, EntryHash),
}

impl From<UtilsError> for WasmError  {
    fn from(error: UtilsError) -> Self {
        WasmError::Guest(format!("{}", error))
    }
}

impl Into<UtilsError> for WasmError  {
    fn into(self) -> UtilsError {
        UtilsError::HDKError(self)
    }
}

pub type UtilsResult<T> = Result<T, UtilsError>;
