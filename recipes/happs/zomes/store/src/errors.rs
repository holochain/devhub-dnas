use hdk::prelude::*;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError<'a> {
    #[error("{0}")]
    CustomError(&'a str),
}
