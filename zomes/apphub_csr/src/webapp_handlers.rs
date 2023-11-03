use crate::{
    hdk,
    hdk_extensions,
    hdi_extensions,
    MY_WEBAPPS_ANCHOR,
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
    WebAppEntry, WebAppManifestV1,
};
use apphub_sdk::{
    LinkBase,
};


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateWebAppEntryInput {
    pub manifest: WebAppManifestV1,
}

#[hdk_extern]
pub fn create_webapp_entry(input: CreateWebAppEntryInput) -> ExternResult<EntryHash> {
    let entry = WebAppEntry::new( input.manifest )?;

    let entry_hash = hash_entry( entry.clone() )?;
    create_entry( entry.to_input() )?;

    MY_WEBAPPS_ANCHOR.create_link_if_not_exists( &entry_hash, () )?;

    Ok( entry_hash )
}

#[hdk_extern]
pub fn get_webapp_entry(addr: AnyDhtHash) -> ExternResult<WebAppEntry> {
    let record = must_get( &addr )?;

    Ok( WebAppEntry::try_from_record( &record )? )
}

#[hdk_extern]
pub fn get_webapp_entries_for_agent(maybe_agent_id: Option<AgentPubKey>) -> ExternResult<Vec<WebAppEntry>> {
    let agent_id = match maybe_agent_id {
        Some(agent_id) => agent_id,
        None => hdk_extensions::agent_id()?,
    };
    let agent_anchor = LinkBase::new( agent_id, LinkTypes::WebApp );
    let webapps = agent_anchor.get_links( None )?.into_iter()
        .filter_map(|link| {
            let addr = link.target.into_entry_hash()?;
            get_webapp_entry( addr.into() ).ok()
        })
        .collect();

    Ok( webapps )
}
