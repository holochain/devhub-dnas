use devhub_types::{
    DevHubResponse, EntityResponse,
    constants::{ VALUE_MD, ENTITY_MD },
    web_asset_entry_types::{ FileEntry, FileInfo, FileChunkEntry },
    composition,
    catch,
};
use hc_entities::{ GetEntityInput };
use hdk::prelude::*;

mod files;
mod constants;



entry_defs![
    FileEntry::entry_def(),
    FileChunkEntry::entry_def()
];



#[hdk_extern]
fn init(_: ()) -> ExternResult<InitCallbackResult> {
    Ok(InitCallbackResult::Pass)
}


#[hdk_extern]
fn whoami(_: ()) -> ExternResult<DevHubResponse<AgentInfo>> {
    Ok(composition( agent_info()?, VALUE_MD ))
}


// Files
#[hdk_extern]
fn create_file(input: files::CreateInput) -> ExternResult<EntityResponse<FileInfo>> {
    let entity = catch!( files::create_file( input ) );

    Ok(composition( entity, ENTITY_MD ))
}

#[hdk_extern]
fn get_file(input: GetEntityInput) -> ExternResult<EntityResponse<FileInfo>> {
    let entity = catch!( files::get_file( input ) );

    Ok(composition( entity, ENTITY_MD ))
}


// File Chunks
#[hdk_extern]
fn create_file_chunk(input: FileChunkEntry) -> ExternResult<EntityResponse<FileChunkEntry>> {
    let entity = catch!( files::create_file_chunk( input ) );

    Ok(composition( entity, ENTITY_MD ))
}

#[hdk_extern]
fn get_file_chunk(input: files::GetFileChunkInput) -> ExternResult<EntityResponse<FileChunkEntry>> {
    let entity = catch!( files::get_file_chunk( input ) );

    Ok(composition( entity, ENTITY_MD ))
}
