pub use hdk_extensions::hdi;
pub use hdk_extensions::holo_hash;
pub use hdk_extensions::hdk;
pub use hdk_extensions::hdi_extensions;
pub use hdk_extensions;
pub use hc_crud;

use hdk::prelude::*;
use hdk::hash_path::path::{
    Path, Component,
};



/// Get a microsecond timestamp of now
pub fn timestamp() -> ExternResult<u64> {
    Ok( sys_time().map( |t| (t.as_micros() / 1000) as u64 )? )
}



pub struct PathInput(pub Vec<Component>);

impl From<Vec<Component>> for PathInput {
    fn from(input: Vec<Component>) -> Self {
        Self(input)
    }
}

impl From<Vec<&str>> for PathInput {
    fn from(input: Vec<&str>) -> Self {
        Self(
            input.into_iter()
                .map( |seg| Component::new( seg.as_bytes().to_vec() ) )
                .collect()
        )
    }
}

impl From<Vec<String>> for PathInput {
    fn from(input: Vec<String>) -> Self {
        Self::from( input.iter().map( |seg| seg.as_str() ).collect::<Vec<&str>>() )
    }
}

impl From<String> for PathInput {
    fn from(input: String) -> Self {
        Self::from( input.split(".").collect::<Vec<&str>>() )
    }
}

impl From<&str> for PathInput {
    fn from(input: &str) -> Self {
        Self::from( input.split(".").collect::<Vec<&str>>() )
    }
}

pub fn path<T>(input: T) -> ExternResult<Path>
where
    PathInput: From<T>,
{
    Ok( Path::from( PathInput::from(input).0 ) )
}
