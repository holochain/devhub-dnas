mod link_base;

pub use hdk_extensions::hdi;
pub use hdk_extensions::holo_hash;
pub use hdk_extensions::hdk;
pub use hdk_extensions::hdi_extensions;
pub use hdk_extensions;
pub use hc_crud;
pub use link_base::*;

use hdi_extensions::{
    guest_error,
};
use hdk::prelude::*;



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

impl From<&[&str]> for PathInput {
    fn from(input: &[&str]) -> Self {
        Self::from( input.to_vec() )
    }
}

impl From<Vec<String>> for PathInput {
    fn from(input: Vec<String>) -> Self {
        Self::from(
            input.iter()
                .map( |seg| seg.as_str() )
                .collect::<Vec<&str>>()
        )
    }
}

impl From<&[String]> for PathInput {
    fn from(input: &[String]) -> Self {
        Self::from( input.to_vec() )
    }
}

impl From<&str> for PathInput {
    fn from(input: &str) -> Self {
        Self::from(
            input.split(".")
                .collect::<Vec<&str>>()
        )
    }
}

impl From<String> for PathInput {
    fn from(input: String) -> Self {
        Self::from( input.as_str() )
    }
}

impl TryFrom<PathInput> for AnyLinkableHash {
    type Error = WasmError;

    fn try_from(input: PathInput) -> ExternResult<Self> {
        Ok( path( input ).path_entry_hash()?.into() )
    }
}

pub fn path<T>(input: T) -> Path
where
    PathInput: From<T>,
{
    Path::from( PathInput::from(input).0 )
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MoveLinkInput<T> {
    pub from: T,
    pub to: T,
}


pub fn unwrap_response(response: ZomeCallResponse) -> ExternResult<ExternIO> {
    match response {
        ZomeCallResponse::Ok(extern_io) => Ok( extern_io ),
        ZomeCallResponse::Unauthorized(auth, cell_id, zome, func, agent) => Err(guest_error!(format!(
            "Unauthorized: {:?} {:?}::{}->{}( ... ) [{}]",
            auth, cell_id, zome, func, agent,
        )))?,
        ZomeCallResponse::NetworkError(msg) => Err(guest_error!(format!(
            "NetworkError: {}", msg,
        )))?,
        ZomeCallResponse::CountersigningSession(msg) => Err(guest_error!(format!(
            "CountersigningSession: {}", msg,
        )))?,
    }
}


#[derive(Clone, Debug, Default)]
pub struct CallOptions {
    pub cap_secret: Option<CapSecret>,
}

impl Into<CallOptions> for () {
    fn into(self) -> CallOptions {
        CallOptions::default()
    }
}

pub fn call_zome<Z,F,I,O,T>(
    zome_name: Z,
    func_name: F,
    payload: I,
    options: O,
) -> ExternResult<T>
where
    Z: Into<ZomeName>,
    F: Into<FunctionName>,
    I: Serialize + std::fmt::Debug,
    T: serde::de::DeserializeOwned + std::fmt::Debug,
    O: Into<CallOptions>,
{
    let call_opts : CallOptions = options.into();

    let response = call(
        CallTargetCell::Local,
        zome_name.into(),
        func_name.into(),
        call_opts.cap_secret,
        payload,
    )?;

    let extern_io = unwrap_response( response )?;

    Ok(
        extern_io.decode()
            .map_err(|err| guest_error!(format!(
                "{:?}", err,
            )))?
    )
}

pub fn call_role<R,Z,F,I,O,T>(
    role_name: R,
    zome_name: Z,
    func_name: F,
    payload: I,
    options: O,
) -> ExternResult<T>
where
    R: Into<String>,
    Z: Into<ZomeName>,
    F: Into<FunctionName>,
    I: Serialize + std::fmt::Debug,
    T: serde::de::DeserializeOwned + std::fmt::Debug,
    O: Into<CallOptions>,
{
    let call_opts : CallOptions = options.into();

    let response = call(
        CallTargetCell::OtherRole(role_name.into()),
        zome_name.into(),
        func_name.into(),
        call_opts.cap_secret,
        payload,
    )?;

    let extern_io = unwrap_response( response )?;

    Ok(
        extern_io.decode()
            .map_err(|err| guest_error!(format!(
                "{:?}", err,
            )))?
    )
}
