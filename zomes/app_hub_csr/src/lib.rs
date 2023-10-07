use devhub_sdk::hdk;
use devhub_sdk::hdk_extensions;

use hdk::prelude::*;
use hdk_extensions::{
    must_get,
};
use app_hub::hdi_extensions::{
    ScopedTypeConnector,
    // AnyLinkableHashTransformer,
};
use app_hub::{
    AppEntry, AppManifestV1,
    UiEntry,
    WebAppEntry, WebAppManifestV1,
    ResourceMap,
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
pub struct CreateAppEntryInput {
    pub manifest: AppManifestV1,
    pub resources: ResourceMap,
}

#[hdk_extern]
fn create_app_entry(input: CreateAppEntryInput) -> ExternResult<ActionHash> {
    let agent_id = hdk_extensions::agent_id()?;
    let entry = AppEntry {
        manifest: input.manifest,
        resources: input.resources,
    };

    let action_hash = create_entry( entry.to_input() )?;

    create_link( agent_id, action_hash.clone(), LinkTypes::App, () )?;

    Ok( action_hash )
}

#[hdk_extern]
fn get_app_entry(addr: ActionHash) -> ExternResult<AppEntry> {
    let record = must_get( &addr )?;

    Ok( AppEntry::try_from_record( &record )? )
}



#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateUiEntryInput {
    pub mere_memory_addr: EntryHash,
}

#[hdk_extern]
fn create_ui_entry(input: CreateUiEntryInput) -> ExternResult<ActionHash> {
    let agent_id = hdk_extensions::agent_id()?;
    let entry = UiEntry::new( input.mere_memory_addr )?;

    let action_hash = create_entry( entry.to_input() )?;

    create_link( agent_id, action_hash.clone(), LinkTypes::Ui, () )?;

    Ok( action_hash )
}

#[hdk_extern]
fn get_ui_entry(addr: ActionHash) -> ExternResult<UiEntry> {
    let record = must_get( &addr )?;

    Ok( UiEntry::try_from_record( &record )? )
}



#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateWebAppEntryInput {
    pub manifest: WebAppManifestV1,
    pub resources: ResourceMap,
}

#[hdk_extern]
fn create_webapp_entry(input: CreateWebAppEntryInput) -> ExternResult<ActionHash> {
    let agent_id = hdk_extensions::agent_id()?;
    let entry = WebAppEntry {
        manifest: input.manifest,
        resources: input.resources,
    };

    let action_hash = create_entry( entry.to_input() )?;

    create_link( agent_id, action_hash.clone(), LinkTypes::WebApp, () )?;

    Ok( action_hash )
}

#[hdk_extern]
fn get_webapp_entry(addr: ActionHash) -> ExternResult<WebAppEntry> {
    let record = must_get( &addr )?;

    Ok( WebAppEntry::try_from_record( &record )? )
}
