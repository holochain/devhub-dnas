use devhub_types::{ DevHubResponse, EntityResponse,
		    VALUE_MD, ENTITY_MD };
use hc_entities::{ GetEntityInput };
use hdk::prelude::*;

mod happ;

mod errors;
mod constants;
mod entry_types;

use entry_types::{ HappEntry };
use errors::{ AppError, ErrorKinds };


entry_defs![
    HappEntry::entry_def()
];



#[macro_export]
macro_rules! catch { // could change to "trap", "snare", or "capture"
    ( $r:expr ) => {
	match $r {
	    Ok(x) => x,
	    Err(e) => {
		let error = match e {
		    ErrorKinds::AppError(e) => (&e).into(),
		    ErrorKinds::UserError(e) => (&e).into(),
		    ErrorKinds::HDKError(e) => (&e).into(),
		    ErrorKinds::DnaUtilsError(e) => (&e).into(),
		};
		return Ok(DevHubResponse::failure( error, None ))
	    },
	}
    };
    ( $r:expr, $e:expr ) => {
	match $r {
	    Ok(x) => x,
	    Err(e) => return Ok(DevHubResponse::failure( (&$e).into(), None )),
	}
    };
}



#[hdk_extern]
fn init(_: ()) -> ExternResult<InitCallbackResult> {
    Ok(InitCallbackResult::Pass)
}


#[hdk_extern]
fn whoami(_: ()) -> ExternResult<DevHubResponse<AgentInfo>> {
    Ok( DevHubResponse::success( agent_info()?, VALUE_MD ) )
}


#[hdk_extern]
fn create_happ(input: happ::CreateInput) -> ExternResult<EntityResponse<entry_types::HappInfo>> {
    let entity = catch!( happ::create_happ( input ) );

    Ok(EntityResponse::success( entity, ENTITY_MD ))
}

#[hdk_extern]
fn get_happ(input: GetEntityInput) -> ExternResult<EntityResponse<entry_types::HappInfo>> {
    let entity = catch!( happ::get_happ( input ) );

    Ok(EntityResponse::success( entity, ENTITY_MD ))
}

#[hdk_extern]
fn update_happ(input: happ::HappUpdateInput) -> ExternResult<EntityResponse<entry_types::HappInfo>> {
    let entity = catch!( happ::update_happ( input ) );

    Ok(EntityResponse::success( entity, ENTITY_MD ))
}

#[hdk_extern]
fn deprecate_happ(input: happ::HappDeprecateInput) -> ExternResult<EntityResponse<entry_types::HappInfo>> {
    let entity = catch!( happ::deprecate_happ( input ) );

    Ok(EntityResponse::success( entity, ENTITY_MD ))
}
