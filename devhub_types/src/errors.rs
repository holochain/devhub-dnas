use hc_dna_utils::UtilsError;
use essence::EssenceError;
use hdk::prelude::*;
use thiserror::Error;



#[derive(Debug, Error)]
pub enum AppError {
    #[error("Unexpected state: {0}")]
    UnexpectedStateError(String),

    #[error("Agent '{3}' does not have the capability to call {0}::{1}->{2}( [args] )")]
    UnauthorizedError(CellId, ZomeName, FunctionName, AgentPubKey),

    #[error("{0}")]
    NetworkError(String),

    #[error("{0}")]
    DeserializeError(String),
}


#[derive(Debug, Error)]
pub enum UserError {
    #[error(transparent)]
    EntryNotFoundError(UtilsError),

    #[error("You already created a hApp with the name: {0}")]
    DuplicateHappNameError(String),

    #[error("{0}")]
    CustomError(&'static str),
}



#[derive(Debug, Error)]
pub enum ErrorKinds {
    #[error(transparent)]
    AppError(AppError),

    #[error(transparent)]
    UserError(UserError),

    #[error(transparent)]
    DnaUtilsError(UtilsError),

    #[error(transparent)]
    FailureResponseError(EssenceError),

    #[error(transparent)]
    HDKError(WasmError), // catch all for unhandled errors
}

impl From<AppError> for ErrorKinds  {
    fn from(error: AppError) -> Self {
        ErrorKinds::AppError(error)
    }
}

impl From<UserError> for ErrorKinds  {
    fn from(error: UserError) -> Self {
        ErrorKinds::UserError(error)
    }
}

impl From<UtilsError> for ErrorKinds  {
    fn from(error: UtilsError) -> Self {
	if let UtilsError::EntryNotFoundError(_) = error {
	    UserError::EntryNotFoundError(error).into()
	}
	else {
            ErrorKinds::DnaUtilsError(error)
	}
    }
}

impl From<EssenceError> for ErrorKinds  {
    fn from(error: EssenceError) -> Self {
        ErrorKinds::FailureResponseError(error)
    }
}

impl From<WasmError> for ErrorKinds  {
    fn from(error: WasmError) -> Self {
        ErrorKinds::HDKError(error)
    }
}
