use essence::{ EssenceResponse };
use hdk::prelude::*;

mod entry_types;
mod errors;
mod handlers;

pub use entry_types::{ MemoryEntry, MemoryBlockEntry, SequencePosition };


pub type Response<T> = EssenceResponse<T, (), ()>;

pub fn success<T>(payload: T) -> Response<T> {
    Response::success( payload, None )
}


entry_defs![
    MemoryEntry::entry_def(),
    MemoryBlockEntry::entry_def()
];


#[hdk_extern]
fn init(_: ()) -> ExternResult<InitCallbackResult> {
    Ok(InitCallbackResult::Pass)
}


#[hdk_extern]
fn save_bytes(sbytes: SerializedBytes) -> ExternResult<Response<EntryHash>> {
    let entry = catch!( handlers::remember_bytes( sbytes.bytes() ) );

    Ok(success( entry ))
}

#[hdk_extern]
fn retrieve_bytes(addr: EntryHash) -> ExternResult<Response<Vec<u8>>> {
    let bytes = catch!( handlers::retrieve_bytes( addr ) );

    Ok(success( bytes ))
}


// Memory
#[hdk_extern]
fn create_memory(input: handlers::CreateInput) -> ExternResult<Response<EntryHash>> {
    let entry = catch!( handlers::create_memory_entry( input ) );

    Ok(success( entry ))
}

#[hdk_extern]
fn get_memory(addr: EntryHash) -> ExternResult<Response<MemoryEntry>> {
    let hash = catch!( handlers::get_memory_entry( addr) );

    Ok(success( hash ))
}


// Memory Blocks
#[hdk_extern]
fn create_memory_block(input: MemoryBlockEntry) -> ExternResult<Response<EntryHash>> {
    let entry = catch!( handlers::create_memory_block_entry( input ) );

    Ok(success( entry ))
}

#[hdk_extern]
fn get_memory_block(addr: EntryHash) -> ExternResult<Response<MemoryBlockEntry>> {
    let hash = catch!( handlers::get_memory_block_entry( addr ) );

    Ok(success( hash ))
}
