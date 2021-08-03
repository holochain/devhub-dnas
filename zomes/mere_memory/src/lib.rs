//! # How to use `mere_memory` in your DNA
//!
//! ## Build the WASM
//! Clone the Github repo [holochain/devhub-dnas](https://github.com/holochain/devhub-dnas) and run
//!
//! ```ignore
//! nix-shell
//! [nix-shell:devhub-dnas$] make zomes/target/wasm32-unknown-unknown/release/mere_memory.wasm
//! ```
//!
//!
//! ## Include `mere_memory` WASM
//! Add the WASM for this zome to your DNA manifest (example)
//!
//! ```ignore
//! manifest_version: "1"
//! ...
//! zomes:
//!   - name: mere_memory
//!     bundled: path/to/mere_memory.wasm
//!   ...other zomes
//! ```
//!
//!
//! ## Add calls to your other zomes
//! Then, from your other zomes, you can call zome functions in 'mere_memory' (example)
//!
//! ```
//! let bytes : Vec<u8> = vec![188, 100, 88, 152, 212, 211, 212, 13];
//! let response = call(
//!     None,
//!     "mere_memory".into(),
//!     "save_bytes".into(),
//!     None,
//!     bytes,
//! )?;
//! ```

use essence::{ EssenceResponse };
use hdk::prelude::*;

mod entry_types;
mod errors;
mod handlers;

pub use entry_types::{ MemoryEntry, MemoryBlockEntry, SequencePosition };

/// Essence response definition
///
/// Types for each of the 3 parts (payload, payload metadata, error metadata)
///
/// 1. Generic <T>
/// 2. None
/// 3. None
pub type Response<T> = EssenceResponse<T, (), ()>;

fn success<T>(payload: T) -> Response<T> {
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
