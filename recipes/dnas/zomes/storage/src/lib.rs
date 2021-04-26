
mod dna;
mod dnaversions;
mod dnachunks;

mod errors;
mod constants;
mod entry_types;

use hdk::prelude::*;
use entry_types::{ DnaEntry, DnaVersionEntry, DnaChunkEntry };


entry_defs![
    DnaEntry::entry_def(),
    DnaVersionEntry::entry_def(),
    DnaChunkEntry::entry_def()
];


#[hdk_extern]
fn init(_: ()) -> ExternResult<InitCallbackResult> {
    Ok(InitCallbackResult::Pass)
}


#[hdk_extern]
fn whoami(_: ()) -> ExternResult<AgentInfo> {
    Ok(agent_info()?)
}
