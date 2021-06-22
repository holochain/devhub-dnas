
mod profile;
mod dna;
mod dnaversions;
mod dnachunks;

mod constants;
mod errors;
mod entry_types;
mod reply_types;
mod utils;

use hdk::prelude::*;
use entry_types::{ ProfileEntry, DnaEntry, DnaVersionEntry, DnaChunkEntry };


entry_defs![
    ProfileEntry::entry_def(),
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
