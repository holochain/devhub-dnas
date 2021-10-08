use devhub_types::{
    AppResult, GetEntityInput,
    web_asset_entry_types::{ FileEntry, FileInfo, FileChunkEntry },
};
use hc_crud::{
    now, create_entity, get_entity,
    Entity,
};
use hdk::prelude::*;

use crate::constants::{ TAG_FILE };



#[derive(Debug, Deserialize)]
pub struct CreateInput {
    pub file_size: u64,
    pub chunk_addresses: Vec<EntryHash>,

    // optional
    pub name: Option<String>,
    pub published_at: Option<u64>,
}


pub fn create_file(input: CreateInput) -> AppResult<Entity<FileInfo>> {
    debug!("Creating FILE ({}): {:?}", input.file_size, input.name );
    let pubkey = agent_info()?.agent_initial_pubkey;
    let default_now = now()?;

    let file = FileEntry {
	author: pubkey.clone(),
	published_at: input.published_at
	    .unwrap_or( default_now ),
	file_size: input.file_size,
	chunk_addresses: input.chunk_addresses,
	name: input.name,
    };

    let entity = create_entity( &file )?
	.change_model( |file| file.to_info() );
    let base = crate::root_path_hash( None )?;

    debug!("Linking pubkey ({}) to ENTRY: {}", base, entity.id );
    entity.link_from( &base, TAG_FILE.into() )?;

    Ok( entity )
}


pub fn get_file(input: GetEntityInput) -> AppResult<Entity<FileInfo>> {
    debug!("Get file: {}", input.id );
    let entity = get_entity::<FileEntry>( &input.id )?;

    Ok(	entity.change_model( |file| file.to_info() ) )
}




pub fn create_file_chunk(chunk: FileChunkEntry) -> AppResult<Entity<FileChunkEntry>> {
    debug!("Creating FILE chunk ({}/{}): {}", chunk.sequence.position, chunk.sequence.length, chunk.bytes.bytes().len() );
    let entity = create_entity( &chunk )?;

    Ok( entity )
}

#[derive(Debug, Deserialize)]
pub struct GetFileChunkInput {
    pub addr: EntryHash,
}

pub fn get_file_chunk(input: GetFileChunkInput) -> AppResult<Entity<FileChunkEntry>> {
    debug!("Get FILE Chunk: {}", input.addr );
    Ok( get_entity::<FileChunkEntry>( &input.addr )? )
}
