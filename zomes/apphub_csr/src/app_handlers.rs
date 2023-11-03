use crate::{
    hdk,
    hdk_extensions,
    hdi_extensions,
    MY_APPS_ANCHOR,
};

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
};
use apphub_sdk::{
    LinkBase,
    RolesDnaTokensInput,
};


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateAppEntryInput {
    pub manifest: AppManifestV1,
    pub roles_dna_tokens: RolesDnaTokensInput,
}

#[hdk_extern]
pub fn create_app_entry(input: CreateAppEntryInput) -> ExternResult<EntryHash> {
    let entry = AppEntry::new(
        input.manifest,
        input.roles_dna_tokens.into_iter()
            .map( |(role_name, dna_token_input)| (role_name, dna_token_input.into()) )
            .collect()
    )?;

    let entry_hash = hash_entry( entry.clone() )?;
    create_entry( entry.to_input() )?;

    MY_APPS_ANCHOR.create_link_if_not_exists( &entry_hash, () )?;

    Ok( entry_hash )
}


#[hdk_extern]
pub fn get_app_entry(addr: AnyDhtHash) -> ExternResult<AppEntry> {
    let record = must_get( &addr )?;

    Ok( AppEntry::try_from_record( &record )? )
}


#[hdk_extern]
pub fn get_app_entries_for_agent(maybe_agent_id: Option<AgentPubKey>) -> ExternResult<Vec<AppEntry>> {
    let agent_id = match maybe_agent_id {
        Some(agent_id) => agent_id,
        None => hdk_extensions::agent_id()?,
    };
    let agent_anchor = LinkBase::new( agent_id, LinkTypes::App );

    let apps = agent_anchor.get_links( None )?.into_iter()
        .filter_map(|link| {
            let addr = link.target.into_entry_hash()?;
            get_app_entry( addr.into() ).ok()
        })
        .collect();

    Ok( apps )
}
