use hdk::prelude::*;

use crate::{ MemoryEntry, MemoryBlockEntry, SequencePosition };
use crate::errors::{ ErrorKinds };


pub type AppResult<T> = Result<T, ErrorKinds>;

fn now() -> AppResult<u64> {
    sys_time()
	.map( |t| (t.as_micros() / 1000) as u64 )
	.map_err(ErrorKinds::HDKError)
}


const BLOCK_SIZE : usize = 4_194_304; // 4MB

pub fn remember_bytes(bytes: &Vec<u8>) -> AppResult<EntryHash> {
    debug!("Creating entries for remembering ({} bytes)", bytes.len() );

    let memory_size = bytes.len();
    let chunks = bytes.chunks( BLOCK_SIZE );
    let block_count = chunks.len();

    let mut blocks : Vec<EntryHash> = vec![];
    for (i, chunk) in chunks.enumerate() { // 1mb block size
	let block_addr = create_memory_block_entry(MemoryBlockEntry{
	    sequence: SequencePosition {
		position: (i+1) as u64,
		length: block_count as u64,
	    },
	    bytes: SerializedBytes::from( UnsafeBytes::from(chunk.to_owned()) ),
	})?;

	blocks.push( block_addr );
    }

    create_memory_entry(CreateInput {
	memory_size: memory_size as u64,
	block_addresses: blocks,
    })
}

pub fn retrieve_bytes(addr: EntryHash) -> AppResult<Vec<u8>> {
    let memory_info = get_memory_entry( addr )?;

    let mut chunks = vec![];
    for block_addr in memory_info.block_addresses.iter() {
	let block = get_memory_block_entry( block_addr.to_owned() )?;
	chunks.push( block.bytes.bytes().to_owned() );
    }

    Ok( chunks.into_iter().flatten().collect() )
}


#[derive(Debug, Deserialize)]
pub struct CreateInput {
    pub memory_size: u64,
    pub block_addresses: Vec<EntryHash>,
}

pub fn create_memory_entry(input: CreateInput) -> AppResult<EntryHash> {
    debug!("Creating 'MemoryEntry' ({} bytes): {}", input.memory_size, input.block_addresses.len() );
    let pubkey = agent_info()?.agent_initial_pubkey;
    let default_now = now()?;

    let memory = MemoryEntry {
	author: pubkey.clone(),
	published_at: default_now,
	memory_size: input.memory_size,
	block_addresses: input.block_addresses,
    };
    let entry_hash = hash_entry( &memory )?;

    create_entry( &memory )?;

    Ok( entry_hash )
}



pub fn get_memory_entry(addr: EntryHash) -> AppResult<MemoryEntry> {
    debug!("Get memory: {}", addr );
    let element = get( addr.clone(), GetOptions::latest() )?
	.ok_or( ErrorKinds::EntryNotFoundError(addr.clone()) )?;
    let memory = MemoryEntry::try_from( &element )?;

    Ok(	memory )
}



pub fn create_memory_block_entry(block: MemoryBlockEntry) -> AppResult<EntryHash> {
    debug!("Creating 'MemoryBlockEntry' ({}/{}): {}", block.sequence.position, block.sequence.length, block.bytes.bytes().len() );
    let entry_hash = hash_entry( &block )?;

    create_entry( &block )?;

    Ok( entry_hash )
}



pub fn get_memory_block_entry(addr: EntryHash) -> AppResult<MemoryBlockEntry> {
    debug!("Get 'MemoryBlockEntry': {}", addr );
    let element = get( addr.clone(), GetOptions::latest() )?
	.ok_or( ErrorKinds::EntryNotFoundError(addr.clone()) )?;
    let block = MemoryBlockEntry::try_from( &element )?;

    Ok(	block )
}
