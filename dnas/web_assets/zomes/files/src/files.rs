use devhub_types::{
    constants::{ AppResult },
    web_asset_entry_types::{ FileEntry, FileInfo, FileChunkEntry },
};
use hc_entities::{ Entity, GetEntityInput };
use hdk::prelude::*;
use hc_dna_utils as utils;

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
    let default_now = utils::now()?;

    let file = FileEntry {
	author: pubkey.clone(),
	published_at: input.published_at
	    .unwrap_or( default_now ),
	file_size: input.file_size,
	chunk_addresses: input.chunk_addresses,
	name: input.name,
    };

    let entity = utils::create_entity( &file )?
	.new_content( file.to_info() );

    debug!("Linking pubkey ({}) to ENTRY: {}", pubkey, entity.id );
    create_link(
	pubkey.into(),
	entity.id.clone(),
	LinkTag::new( TAG_FILE )
    )?;

    Ok( entity )
}


pub fn get_file(input: GetEntityInput) -> AppResult<Entity<FileInfo>> {
    debug!("Get file: {}", input.id );
    let entity = utils::get_entity( &input.id )?;
    let info = FileEntry::try_from( &entity.content )?.to_info();

    Ok(	entity.new_content( info ) )
}




pub fn create_file_chunk(chunk: FileChunkEntry) -> AppResult<Entity<FileChunkEntry>> {
    debug!("Creating FILE chunk ({}/{}): {}", chunk.sequence.position, chunk.sequence.length, chunk.bytes.bytes().len() );
    let entity = utils::create_entity( &chunk )?;

    Ok( entity )
}

#[derive(Debug, Deserialize)]
pub struct GetFileChunkInput {
    pub addr: EntryHash,
}

pub fn get_file_chunk(input: GetFileChunkInput) -> AppResult<Entity<FileChunkEntry>> {
    debug!("Get FILE Chunk: {}", input.addr );
    let entity = utils::get_entity( &input.addr )?;
    let info = FileChunkEntry::try_from( &entity.content )?;

    Ok( entity.new_content( info ) )
}
