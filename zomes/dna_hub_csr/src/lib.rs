use devhub_sdk::hdk;
use devhub_sdk::hdk_extensions;

use hdk::prelude::*;
use hdk_extensions::{
    must_get,
};
use dna_hub::hdi_extensions::{
    ScopedTypeConnector,
    // AnyLinkableHashTransformer,
};
use dna_hub::{
    DnaEntry, DnaManifestV1, ResourceMap,
    LinkTypes,
};



#[hdk_extern]
fn init(_: ()) -> ExternResult<InitCallbackResult> {
    Ok(InitCallbackResult::Pass)
}


#[hdk_extern]
fn whoami(_: ()) -> ExternResult<AgentInfo> {
    Ok( agent_info()? )
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateDnaEntryInput {
    pub manifest: DnaManifestV1,
    pub resources: ResourceMap,
}

#[hdk_extern]
fn create_dna_entry(input: CreateDnaEntryInput) -> ExternResult<ActionHash> {
    let agent_id = hdk_extensions::agent_id()?;
    let entry = DnaEntry {
        manifest: input.manifest,
        resources: input.resources,
    };

    let action_hash = create_entry( entry.to_input() )?;

    create_link( agent_id, action_hash.clone(), LinkTypes::Dna, () )?;

    Ok( action_hash )
}

#[hdk_extern]
fn get_dna_entry(addr: ActionHash) -> ExternResult<DnaEntry> {
    let record = must_get( &addr )?;

    Ok( DnaEntry::try_from_record( &record )? )
}
