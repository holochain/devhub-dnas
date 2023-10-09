use devhub_sdk::hdk;
use devhub_sdk::hdk_extensions;
use app_hub::hdi_extensions;
use app_hub::hc_crud;

use std::collections::BTreeMap;
use hdk::prelude::*;
use hdk_extensions::{
    must_get,
};
use hdi_extensions::{
    ScopedTypeConnector,
    // AnyLinkableHashTransformer,
};
use app_hub::{
    LinkTypes,
    AppEntry, AppManifestV1,
    UiEntry,
    WebAppEntry, WebAppManifestV1,
    WebAppPackageEntry,
    ResourceMap,
    Authority, MemoryAddr, // DeprecationNotice,
};
use hc_crud::{
    create_entity, get_entity,
    Entity,
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



#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateWebAppPackageEntryInput {
    pub title: String,
    pub subtitle: String,
    pub description: String,
    pub icon: MemoryAddr,
    #[serde(default)]
    pub metadata: BTreeMap<String, rmpv::Value>,

    // Optional
    pub maintainer: Option<Authority>,
    pub source_code_url: Option<String>,
}

#[hdk_extern]
fn create_webapp_package_entry(input: CreateWebAppPackageEntryInput) -> ExternResult<Entity<WebAppPackageEntry>> {
    let agent_id = hdk_extensions::agent_id()?;
    let entry = WebAppPackageEntry {
        title: input.title,
        subtitle: input.subtitle,
        description: input.description,
        maintainer: agent_id.clone().into(),
        icon: input.icon,
        source_code_url: input.source_code_url,
        deprecation: None,
        metadata: input.metadata,
    };

    let entity = create_entity( &entry )?;

    create_link( agent_id, entity.id.clone(), LinkTypes::WebAppPackage, () )?;

    Ok( entity )
}

#[hdk_extern]
fn get_webapp_package_entry(addr: ActionHash) -> ExternResult<Entity<WebAppPackageEntry>> {
    Ok( get_entity( &addr )? )
}
