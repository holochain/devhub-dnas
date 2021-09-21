#![macro_use]

use hdk::prelude::*;
use thiserror::Error;


#[derive(Debug, Error)]
pub enum ErrorKinds {
    #[error("Entry not found for address: {0:?}")]
    EntryNotFoundError(EntryHash),

    #[error(transparent)]
    HDKError(WasmError),
}

impl From<WasmError> for ErrorKinds  {
    fn from(error: WasmError) -> Self {
        ErrorKinds::HDKError(error)
    }
}


macro_rules! catch {
    ( $r:expr ) => {
	match $r {
	    Ok(x) => x,
	    Err(e) => {
		return Ok(Response::failure( (&e).into(), None ))
	    },
	}
    };
}
