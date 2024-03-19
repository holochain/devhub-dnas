use crate::{
    hdk,
    hdk_extensions,
    hdi_extensions,
    webapp_package_handlers,
    MY_WEBAPP_PACK_VERSIONS_ANCHOR,
};

use std::collections::BTreeMap;
use hdk::prelude::*;
use hdk_extensions::{
    must_get,
};
use hdi_extensions::{
    ScopedTypeConnector,
};
use apphub::{
    EntryTypes,
    LinkTypes,
    WebAppPackageVersionEntry,
    Authority,
    hc_crud::{
        Entity, EntityId,
        UpdateEntityInput,
        create_entity, get_entity, update_entity, delete_entity,
    },
};
use apphub_sdk::{
    LinkBase,
    MoveLinkInput,
    WebAppPackageVersionEntryInput,
    CreateWebAppPackageVersionInput,
};
use webapp_package_handlers::{
    DeleteLinkWebAppPackageVersionInput,
    CreateLinkWebAppPackageVersionInput,
    delete_webapp_package_links_to_version,
    create_webapp_package_link_to_version,
};


fn create_webapp_package_version_entry_handler(entry: WebAppPackageVersionEntry) ->
    ExternResult<Entity<WebAppPackageVersionEntry>>
{
    let entity = create_entity( &entry )?;

    MY_WEBAPP_PACK_VERSIONS_ANCHOR.create_link_if_not_exists( &entity.id, () )?;

    Ok( entity )
}


#[hdk_extern]
pub fn create_webapp_package_version_entry(input: WebAppPackageVersionEntryInput) ->
    ExternResult<Entity<WebAppPackageVersionEntry>>
{
    create_webapp_package_version_entry_handler( input.into() )
}


#[hdk_extern]
pub fn create_webapp_package_version(input: CreateWebAppPackageVersionInput) ->
    ExternResult<Entity<WebAppPackageVersionEntry>>
{
    let entity = create_webapp_package_version_entry_handler( input.clone().try_into()? )?;

    create_webapp_package_link_to_version(CreateLinkWebAppPackageVersionInput {
	version: input.version,
	webapp_package_id: input.for_package,
	webapp_package_version_addr: entity.id.clone(),
    })?;

    Ok( entity )
}


#[hdk_extern]
pub fn get_webapp_package_version_entry(addr: ActionHash) ->
    ExternResult<WebAppPackageVersionEntry>
{
    let record = must_get( &addr )?;

    Ok( WebAppPackageVersionEntry::try_from_record( &record )? )
}


#[hdk_extern]
pub fn get_webapp_package_version(addr: ActionHash) ->
    ExternResult<Entity<WebAppPackageVersionEntry>>
{
    Ok( get_entity( &addr )? )
}


#[hdk_extern]
pub fn get_webapp_package_version_entries_for_agent(maybe_agent_id: Option<AgentPubKey>) ->
    ExternResult<Vec<Entity<WebAppPackageVersionEntry>>>
{
    let agent_id = match maybe_agent_id {
        Some(agent_id) => agent_id,
        None => hdk_extensions::agent_id()?,
    };
    let agent_anchor = LinkBase::new( agent_id, LinkTypes::AgentToWebAppPackageVersion );
    let versions = agent_anchor.get_links( None )?.into_iter()
        .filter_map(|link| {
            let addr = link.target.into_action_hash()?;
            get_webapp_package_version( addr ).ok()
        })
        .collect();

    Ok( versions )
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateWebAppPackageVersionInput {
    pub for_package: Option<EntityId>,
    pub changelog: Option<String>,
    pub maintainer: Option<Authority>,
    pub source_code_revision_uri: Option<String>,
    pub metadata: Option<BTreeMap<String, rmpv::Value>>,
}

#[hdk_extern]
pub fn update_webapp_package_version(input: UpdateEntityInput<UpdateWebAppPackageVersionInput>) ->
    ExternResult<Entity<WebAppPackageVersionEntry>>
{
    let changes = input.properties;
    let entity = update_entity( &input.base, |version: WebAppPackageVersionEntry, _| {
        let entry = WebAppPackageVersionEntry {
            for_package: changes.for_package
                .unwrap_or( version.for_package ),
            webapp: version.webapp,
            webapp_token: version.webapp_token,
            changelog: changes.changelog
                .or( version.changelog ),
            maintainer: changes.maintainer
                .unwrap_or( version.maintainer ).into(),
            source_code_revision_uri: changes.source_code_revision_uri
                .or( version.source_code_revision_uri ),
            metadata: changes.metadata
                .unwrap_or( version.metadata ),
        };

	Ok( entry )
    })?;

    Ok( entity )
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MoveWebAppPackageVersionInput {
    pub version: String,
    pub webapp_package_version_id: ActionHash,
    pub webapp_package_ids: MoveLinkInput<ActionHash>,
}

#[hdk_extern]
pub fn move_webapp_package_version(input: MoveWebAppPackageVersionInput) ->
    ExternResult<Entity<WebAppPackageVersionEntry>>
{
    let deleted_links = delete_webapp_package_links_to_version(DeleteLinkWebAppPackageVersionInput {
	version: input.version.clone(),
	webapp_package_id: input.webapp_package_ids.from,
    })?;
    debug!("Deleted links: {:?}", deleted_links.iter().map( |hash| format!("{}", hash) ).collect::<Vec<String>>() );

    let version = get_webapp_package_version( input.webapp_package_version_id.clone() )?;
    let entity = update_webapp_package_version(UpdateEntityInput {
	base: version.action.clone(),
	properties: UpdateWebAppPackageVersionInput {
	    for_package: Some(input.webapp_package_ids.to.clone()),
            changelog: None,
            maintainer: None,
            source_code_revision_uri: None,
            metadata: None,
        },
    })?;

    create_webapp_package_link_to_version(CreateLinkWebAppPackageVersionInput {
	version: input.version,
	webapp_package_id: input.webapp_package_ids.to,
	webapp_package_version_addr: entity.action.clone(),
    })?;

    Ok( entity )
}


#[hdk_extern]
fn delete_webapp_package_version(addr: ActionHash) -> ExternResult<ActionHash> {
    Ok( delete_entity::<WebAppPackageVersionEntry,EntryTypes>( &addr )? )
}
