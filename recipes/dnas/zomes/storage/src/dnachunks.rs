use devhub_types::{ DevHubResponse, EntityResponse, ENTITY_MD };
use hc_entities::{ Entity, EntryModel };
use hc_dna_utils as utils;
use crate::catch;
use hdk::prelude::*;

use crate::entry_types::{ DnaChunkEntry };


#[hdk_extern]
fn create_dna_chunk(chunk: DnaChunkEntry) -> ExternResult<EntityResponse<DnaChunkEntry>> {
    debug!("Creating DNA chunk ({}/{}): {}", chunk.sequence.position, chunk.sequence.length, chunk.bytes.bytes().len() );

    let header_hash = catch!( create_entry(&chunk) );
    let entry_hash = catch!( hash_entry(&chunk) );

    Ok( EntityResponse::success(Entity {
	id: entry_hash.clone(),
	address: entry_hash,
	header: header_hash,
	ctype: chunk.get_type(),
	content: chunk,
    }, ENTITY_MD ))
}

#[derive(Debug, Deserialize)]
pub struct GetDnaChunksInput {
    pub addr: EntryHash,
}

#[hdk_extern]
fn get_dna_chunk(input: GetDnaChunksInput) -> ExternResult<EntityResponse<DnaChunkEntry>> {
    debug!("Get DNA Chunk: {}", input.addr );
    let entity = catch!( utils::get_entity( &input.addr ) );
    let info = catch!( DnaChunkEntry::try_from(&entity.content) );

    Ok( EntityResponse::success(
	entity.new_content( info ), ENTITY_MD
    ))
}
