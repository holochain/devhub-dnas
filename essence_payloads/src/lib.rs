//! # Basic Usage
//!
//! ## Defining your Essence package
//! Essence packages have 3 options for structure variations
//!
//! 1. Payload struct
//! 2. Payload metadata struct
//! 3. Error metadata struct
//!
//! Example using a generic payload struct with no metadata structs
//! ```
//! use essence::{ EssenceResponse };
//! pub type MyResponse<T> = EssenceResponse<T, (), ()>;
//! ```

mod utils;
use thiserror::Error;

use serde::{Serialize, Deserialize};

use utils::{ type_of, struct_name };


/// The standard shell of all Essence packages
#[derive(Debug, Serialize, Deserialize)]
pub struct EssencePackage<T, M> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<M>,
    pub payload: T,
}

/// The payload definition of Essence failure responses
#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorPayload {
    pub kind: String,
    pub error: String,
    pub message: String,
    pub stack: Vec<String>,
}

impl<T> From<&T> for ErrorPayload
where
    T: std::error::Error
{
    fn from(error: &T) -> Self {
	let kind = type_of(&error);
	let name = struct_name(&error);

	ErrorPayload {
	    kind: kind,
	    error: name,
	    message: format!("{}", error),
	    stack: vec![],
	}
    }
}


/// The possible errors that could be raised by this crate
#[derive(Debug, Error)]
pub enum EssenceError {
    #[error("[{0}::{1}( {2} )]")]
    ErrorPayload(String, String, String),
}

/// This defines the struct of an Essence Error package
pub type EssenceErrorPackage<M> = EssencePackage<ErrorPayload, M>;


/// Defines the 2 possible package types (success or failure)
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
pub enum EssenceResponse<P, PM, EM> {
    Success(EssencePackage<P, PM>),
    Failure(EssenceErrorPackage<EM>),
}


impl<P, PM, EM> EssenceResponse<P, PM, EM> {
    pub fn new<E>(payload: Result<P, E>, success_metadata: Option<PM>, error_metadata: Option<EM>, ) -> Self
    where
	E: std::error::Error {
	match payload {
	    Ok(data) => EssenceResponse::Success(EssencePackage {
		metadata: success_metadata,
		payload: data,
	    }),
	    Err(error) => EssenceResponse::Failure(EssencePackage {
		metadata: error_metadata,
		payload: ErrorPayload::from( &error ),
	    }),
	}
    }

    pub fn success(payload: P, metadata: Option<PM>) -> Self {
	EssenceResponse::Success(EssencePackage {
	    metadata: metadata,
	    payload: payload,
	})
    }

    pub fn failure(error: ErrorPayload, metadata: Option<EM>) -> Self {
	EssenceResponse::Failure(EssencePackage {
	    metadata: metadata,
	    payload: error,
	})
    }

    pub fn as_result(self) -> Result<P, EssenceError> {
	match self {
	    EssenceResponse::Success(pack) => Ok(pack.payload),
	    EssenceResponse::Failure(pack) => Err(EssenceError::ErrorPayload(pack.payload.kind.clone(), pack.payload.error.clone(), pack.payload.message.clone())),
	}
    }
}


#[cfg(test)]
pub mod tests {
    use super::*;

    use serde_json::json;
    use std::fmt;
    use thiserror::Error;

    #[test]
    ///
    fn type_of_test() {
	let vector = vec![1,2,3,4];

	assert_eq!(
	    type_of(&vector),
	    String::from("Vec<i32>") );
    }

    #[test]
    ///
    fn struct_name_test() {
	let vector = vec![1,2,3,4];

	assert_eq!(
	    struct_name(&vector),
	    String::from("Error") );
    }

    #[test]
    ///
    fn enum_test() {
	#[derive(Debug, Error)]
	enum AppError<'a> {
	    #[error("This is so bad input: {0}")]
	    BadInput(&'a str),
	}

	let error = AppError::BadInput("This is so bad...");
	let payload = ErrorPayload::from( &error );

	assert_eq!(
	    serde_json::to_string_pretty( &json!(payload) ).unwrap(),
	    String::from(r#"{
  "error": "BadInput",
  "kind": "AppError",
  "message": "This is so bad input: This is so bad...",
  "stack": []
}"#));
	println!("{:?}", error );
    }

    #[test]
    ///
    fn struct_test() {
	#[derive(Debug, Error)]
	struct MyError {
	    msg: String,
	}

	impl fmt::Display for MyError {
	    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.msg)
	    }
	}

	let error = MyError { msg: "EntryHash(uhCEkNBaVvGRYmJUqsGNrfO8jC9Ij-t77QcmnAk3E3B8qh6TU09QN)".into() };
	let payload = ErrorPayload::from( &error );

	assert_eq!(
	    serde_json::to_string_pretty( &json!(payload) ).unwrap(),
	    String::from(r#"{
  "error": "MyError",
  "kind": "MyError",
  "message": "EntryHash(uhCEkNBaVvGRYmJUqsGNrfO8jC9Ij-t77QcmnAk3E3B8qh6TU09QN)",
  "stack": []
}"#));
	println!("{:?}", error );
    }

    #[test]
    ///
    fn tuple_test() {
	#[derive(Debug, Error)]
	struct MyError<'a>(&'a str, String);

	impl<'a> fmt::Display for MyError<'a> {
	    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}: {}", self.0, self.1 )
	    }
	}

	let error = MyError( "BadInput", "This is so bad...".into() );
	let payload = ErrorPayload::from( &error );

	assert_eq!(
	    serde_json::to_string_pretty( &json!(payload) ).unwrap(),
	    String::from(r#"{
  "error": "MyError",
  "kind": "MyError",
  "message": "BadInput: This is so bad...",
  "stack": []
}"#));
	println!("{:?}", error );
    }
}
