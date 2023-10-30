mod dna_handlers;

pub use dnahub::hdi;
pub use dnahub::hdi_extensions;
use devhub_sdk::hdk;
use devhub_sdk::hdk_extensions;

use hdk::prelude::*;



#[hdk_extern]
fn init(_: ()) -> ExternResult<InitCallbackResult> {
    let zome_name = zome_info()?.name;
    debug!("'{}' init", zome_name );

    portal_sdk::register_if_exists!({
        dna: dna_info()?.hash,
        granted_functions: vec![
            ( zome_name.0.as_ref(), "get_dna_entry" ),
            ( zome_name.0.as_ref(), "get_dna_entries_for_agent" ),
        ],
    })?;

    Ok(InitCallbackResult::Pass)
}


#[hdk_extern]
fn whoami(_: ()) -> ExternResult<AgentInfo> {
    Ok( agent_info()? )
}
