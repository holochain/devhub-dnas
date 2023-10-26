use crate::hdk;
use crate::hdk_extensions;
use crate::hdi_extensions;

use hdk::prelude::*;
use hdk_extensions::{
    must_get,
};
use hdi_extensions::{
    ScopedTypeConnector,
};
use apphub::{
    LinkTypes,
    AppEntry, AppManifestV1,
    ResourceMap,
};


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateAppEntryInput {
    pub manifest: AppManifestV1,
    pub resources: ResourceMap,
}

#[hdk_extern]
fn create_app_entry(input: CreateAppEntryInput) -> ExternResult<EntryHash> {
    let agent_id = hdk_extensions::agent_id()?;
    let entry = AppEntry {
        manifest: input.manifest,
        resources: input.resources,
    };

    let entry_hash = hash_entry( entry.clone() )?;
    create_entry( entry.to_input() )?;

    create_link( agent_id, entry_hash.clone(), LinkTypes::App, () )?;

    Ok( entry_hash )
}

#[hdk_extern]
fn get_app_entry(addr: EntryHash) -> ExternResult<AppEntry> {
    let record = must_get( &addr )?;

    Ok( AppEntry::try_from_record( &record )? )
}

#[hdk_extern]
fn get_app_entries_for_agent(maybe_agent_id: Option<AgentPubKey>) -> ExternResult<Vec<AppEntry>> {
    let agent_id = match maybe_agent_id {
        Some(agent_id) => agent_id,
        None => hdk_extensions::agent_id()?,
    };
    let apps = get_links( agent_id, LinkTypes::App, None )?.into_iter()
        .filter_map(|link| {
            let addr = link.target.into_entry_hash()?;
            get_app_entry( addr ).ok()
        })
        .collect();

    Ok( apps )
}
