
use devhub_types::{ DevHubResponse, VALUE_MD };
use hdk::prelude::*;

mod profile;
mod dna;
mod dnaversions;
mod dnachunks;

mod errors;
mod constants;
mod entry_types;

use entry_types::{ ProfileEntry, DnaEntry, DnaVersionEntry, DnaChunkEntry };


#[macro_export]
macro_rules! catch { // could change to "trap", "snare", or "capture"
    ( $r:expr ) => {
	match $r {
	    Ok(x) => x,
	    Err(e) => return Ok(DevHubResponse::failure( (&e).into(), None )),
	}
    };
    ( $r:expr, $e:expr ) => {
	match $r {
	    Ok(x) => x,
	    Err(e) => return Ok(DevHubResponse::failure( (&$e).into(), None )),
	}
    };
}


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
