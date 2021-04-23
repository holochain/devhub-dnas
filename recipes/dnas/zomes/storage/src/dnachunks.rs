use hdk::prelude::*;

use crate::entry_types::{ DnaChunkEntry };


#[hdk_extern]
fn create_dna_chunk(chunk: DnaChunkEntry) -> ExternResult<(EntryHash, DnaChunkEntry)> {
    debug!("Creating DNA chunk ({}/{}): {}", chunk.sequence.position, chunk.sequence.length, chunk.bytes.bytes().len() );

    create_entry(&chunk)?;
    let entry_hash = hash_entry(&chunk)?;

    Ok( (entry_hash, chunk) )
}
