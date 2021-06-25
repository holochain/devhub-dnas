use hdk::prelude::*;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError<'a> {
    #[error("Entry not found for address: {0:?}")]
    EntryNotFound(&'a EntryHash),

    // #[error("Header not found for address: {0:?}")]
    // HeaderNotFound(&'a HeaderHash),

    // #[error("{0}")]
    // General(&'a str),
}
