use hdk::prelude::*;
use hc_dna_utils as utils;

use crate::entry_types::{ DnaChunkEntry };


#[hdk_extern]
fn create_dna_chunk(chunk: DnaChunkEntry) -> ExternResult<(EntryHash, DnaChunkEntry)> {
    debug!("Creating DNA chunk ({}/{}): {}", chunk.sequence.position, chunk.sequence.length, chunk.bytes.bytes().len() );

    create_entry(&chunk)?;
    let entry_hash = hash_entry(&chunk)?;

    Ok( (entry_hash, chunk) )
}

#[derive(Debug, Deserialize)]
pub struct GetDnaChunksInput {
    pub addr: EntryHash,
}

#[hdk_extern]
fn get_dna_chunk(input: GetDnaChunksInput) -> ExternResult<DnaChunkEntry> {
    debug!("Get DNA Chunk: {}", input.addr );
    let (_, element) = utils::fetch_entry_latest(input.addr.clone())?;

    Ok(DnaChunkEntry::try_from(&element)?)
}
