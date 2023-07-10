use std::collections::BTreeMap;
use web_assets_core::{
    LinkTypes,
};
use devhub_types::{
    AppResult, GetEntityInput,
    errors::{ UserError },
    web_asset_entry_types::{
	FileEntry,
	FilePackage,
    },
    call_local_zome,
};
use hc_crud::{
    now, create_entity, get_entity,
    Entity, EntityType,
};
use mere_memory_types::{ MemoryEntry };
use hdk::prelude::*;



#[derive(Debug, Deserialize)]
pub struct CreateInput {
    // optional
    pub mere_memory_addr: Option<EntryHash>,
    pub file_bytes: Option<SerializedBytes>,
    pub name: Option<String>,
    pub published_at: Option<u64>,
    pub last_updated: Option<u64>,
    pub metadata: Option<BTreeMap<String, serde_yaml::Value>>,
}


pub fn create_file(input: CreateInput) -> AppResult<Entity<FileEntry>> {
    debug!("Creating FILE: {:?}", input.name );
    let pubkey = agent_info()?.agent_initial_pubkey;
    let default_now = now()?;

    let mere_memory_addr = match input.mere_memory_addr {
	Some(addr) => addr,
	None => {
	    let bytes = input.file_bytes
		.ok_or( UserError::CustomError("You must supply an address or bytes for the file") )?;

	    call_local_zome("mere_memory_api", "save_bytes", bytes )?
	},
    };
    let memory : MemoryEntry = call_local_zome("mere_memory_api", "get_memory", mere_memory_addr.to_owned() )?;

    let file = FileEntry {
	author: pubkey.clone(),
	published_at: input.published_at
	    .unwrap_or( default_now ),
	last_updated: input.last_updated
	    .unwrap_or( default_now ),
	file_size: memory.memory_size,
	mere_memory_addr: mere_memory_addr,
	mere_memory_hash: memory.hash,
	name: input.name,
	metadata: input.metadata
	    .unwrap_or( BTreeMap::new() ),
    };

    let entity = create_entity( &file )?;
    let base = crate::root_path_hash( None )?;

    debug!("Linking pubkey ({}) to ENTRY: {}", base, entity.id );
    entity.link_from( &base, LinkTypes::File, None )?;

    Ok( entity )
}


pub fn get_file(input: GetEntityInput) -> AppResult<Entity<FileEntry>> {
    debug!("Get file: {}", input.id );
    Ok( get_entity( &input.id )? )
}


pub fn get_file_package(input: GetEntityInput) -> AppResult<Entity<FilePackage>> {
    debug!("Get file: {}", input.id );
    let entity : Entity<FileEntry> = get_entity( &input.id )?;

    let package = entity.content.to_package();

    Ok(Entity {
	id: entity.id,
	action: entity.action,
	address: entity.address,
	ctype: EntityType::new( "file", "package" ),
	content: package,
    })
}
