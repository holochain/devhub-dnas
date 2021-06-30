mod utils;

use serde::*;

use utils::{ type_of, struct_name };


#[derive(Debug, Serialize)]
pub struct EssencePackage<T, M> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<M>,
    pub payload: T,
}

#[derive(Debug, Serialize)]
pub struct ErrorPayload {
    pub kind: String,
    pub error: String,
    pub message: String,
    pub stack: Vec<String>,
}

impl<T: std::error::Error> From<&T> for ErrorPayload {
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


pub type ErrorEssencePackage<M> = EssencePackage<ErrorPayload, M>;


#[derive(Debug, Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
pub enum EssenceResponse<P, PM, EM> {
    Success(EssencePackage<P, PM>),
    Failure(ErrorEssencePackage<EM>),
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

    pub fn error(error: ErrorPayload, metadata: Option<EM>) -> Self {
	EssenceResponse::Failure(EssencePackage {
	    metadata: metadata,
	    payload: error,
	})
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
  "kind": "AppError",
  "error": "BadInput",
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

	let error = MyError { msg: "This is so bad...".into() };
	let payload = ErrorPayload::from( &error );

	assert_eq!(
	    serde_json::to_string_pretty( &json!(payload) ).unwrap(),
	    String::from(r#"{
  "kind": "MyError",
  "error": "MyError",
  "message": "This is so bad...",
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
  "kind": "MyError",
  "error": "MyError",
  "message": "BadInput: This is so bad...",
  "stack": []
}"#));
	println!("{:?}", error );
    }
}
