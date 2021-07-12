use devhub_types::{
    AppResult,
    dna_entry_types::{ DnaChunkEntry },
};
use hc_entities::{ Entity };
use hc_dna_utils as utils;
use hdk::prelude::*;


pub fn create_dna_chunk(chunk: DnaChunkEntry) -> AppResult<Entity<DnaChunkEntry>> {
    debug!("Creating DNA chunk ({}/{}): {}", chunk.sequence.position, chunk.sequence.length, chunk.bytes.bytes().len() );
    let entity = utils::create_entity( &chunk )?;

    Ok( entity )
}

#[derive(Debug, Deserialize)]
pub struct GetDnaChunksInput {
    pub addr: EntryHash,
}

pub fn get_dna_chunk(input: GetDnaChunksInput) -> AppResult<Entity<DnaChunkEntry>> {
    debug!("Get DNA Chunk: {}", input.addr );
    let entity = utils::get_entity( &input.addr )?;
    let info = DnaChunkEntry::try_from( &entity.content )?;

    Ok( entity.new_content( info ) )
}
