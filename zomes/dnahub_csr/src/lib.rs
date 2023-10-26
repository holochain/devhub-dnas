use devhub_sdk::hdk;
use devhub_sdk::hdk_extensions;

use hdk::prelude::*;
use hdk_extensions::{
    must_get,
};
use dnahub::hdi_extensions::{
    ScopedTypeConnector,
    // AnyLinkableHashTransformer,
};
use dnahub::{
    DnaEntry, DnaManifestV1, ResourceMap,
    LinkTypes,
};



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


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateDnaEntryInput {
    pub manifest: DnaManifestV1,
    pub resources: ResourceMap,
}

#[hdk_extern]
fn create_dna_entry(input: CreateDnaEntryInput) -> ExternResult<EntryHash> {
    let agent_id = hdk_extensions::agent_id()?;
    let entry = DnaEntry {
        manifest: input.manifest,
        resources: input.resources,
    };

    let entry_hash = hash_entry( entry.clone() )?;
    create_entry( entry.to_input() )?;

    create_link( agent_id, entry_hash.clone(), LinkTypes::Dna, () )?;

    Ok( entry_hash )
}

#[hdk_extern]
fn get_dna_entry(addr: EntryHash) -> ExternResult<DnaEntry> {
    let record = must_get( &addr )?;

    Ok( DnaEntry::try_from_record( &record )? )
}

#[hdk_extern]
fn get_dna_entries_for_agent(maybe_agent_id: Option<AgentPubKey>) -> ExternResult<Vec<DnaEntry>> {
    let agent_id = match maybe_agent_id {
        Some(agent_id) => agent_id,
        None => hdk_extensions::agent_id()?,
    };
    let dnas = get_links( agent_id, LinkTypes::Dna, None )?.into_iter()
        .filter_map(|link| {
            let addr = link.target.into_entry_hash()?;
            get_dna_entry( addr ).ok()
        })
        .collect();

    Ok( dnas )
}
