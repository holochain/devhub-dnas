use hdk::prelude::*;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum UtilsError {
    #[error("HDK raised error: {0:?}")]
    HDKError(WasmError),

    #[error("Entry not found")]
    EntryNotFound,

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
