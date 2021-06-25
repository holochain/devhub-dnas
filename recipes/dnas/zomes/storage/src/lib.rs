
mod profile;
mod dna;
mod dnaversions;
mod dnachunks;

mod errors;
mod constants;
mod entry_types;

use hdk::prelude::*;
use entry_types::{ ProfileEntry, DnaEntry, DnaVersionEntry, DnaChunkEntry };
use devhub_types::{ DevHubResponse, VALUE_MD };


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
fn whoami(_: ()) -> ExternResult<DevHubResponse<AgentInfo>> {
    Ok( DevHubResponse::success( agent_info()?, VALUE_MD ) )
}
