
mod dna;
mod dnaversions;
mod dnachunks;

mod constants;
mod entry_types;

use hdk::prelude::*;
use entry_types::{ AppEntry, ManifestEntry, DnaEntry, DnaVersionEntry, DnaChunkEntry };


entry_defs![
    AppEntry::entry_def(),
    ManifestEntry::entry_def(),
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
