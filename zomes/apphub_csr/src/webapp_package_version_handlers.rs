use crate::hdk;
use crate::hdk_extensions;
use crate::hdi_extensions;
use crate::webapp_package_handlers;

use std::collections::BTreeMap;
use hdk::prelude::*;
use hdk_extensions::{
    must_get,
};
use hdi_extensions::{
    ScopedTypeConnector,
};
use apphub::{
    LinkTypes,
    WebAppEntry,
    WebAppPackageVersionEntry,
    Authority,
    BundleAddr,
    hc_crud::{
        Entity, EntityId,
        UpdateEntityInput,
        create_entity, get_entity, update_entity,
    },
};
use apphub_sdk::{
    MoveLinkInput,
};
use webapp_package_handlers::{
    DeleteLinkWebAppPackageVersionInput,
    CreateLinkWebAppPackageVersionInput,
    delete_webapp_package_links_to_version,
    create_webapp_package_link_to_version,
};


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateWebAppPackageVersionEntryInput {
    pub for_package: EntityId,
    pub webapp: BundleAddr,
    #[serde(default)]
    pub metadata: BTreeMap<String, rmpv::Value>,

    // Optional
    pub changelog: Option<String>,
    pub maintainer: Option<Authority>,
    pub source_code_revision_uri: Option<String>,
}

#[hdk_extern]
pub fn create_webapp_package_version_entry(input: CreateWebAppPackageVersionEntryInput) ->
    ExternResult<Entity<WebAppPackageVersionEntry>>
{
    let agent_id = hdk_extensions::agent_id()?;
    let webapp_entry = WebAppEntry::try_from_record( &must_get( &input.webapp )? )?;

    // Get webapp entry info to construct the integrity entry info
    let entry = WebAppPackageVersionEntry {
        for_package: input.for_package,
        webapp: input.webapp,
        webapp_token: webapp_entry.webapp_token,
        changelog: input.changelog,
        maintainer: agent_id.clone().into(),
        source_code_revision_uri: input.source_code_revision_uri,
        metadata: input.metadata,
    };

    let entity = create_entity( &entry )?;

    create_link( agent_id, entity.id.clone(), LinkTypes::WebAppPackageVersion, () )?;

    Ok( entity )
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateWebAppPackageVersionInput {
    pub version: String,
    #[serde(flatten)]
    webapp_package_version_input: CreateWebAppPackageVersionEntryInput,
}

#[hdk_extern]
pub fn create_webapp_package_version(input: CreateWebAppPackageVersionInput) ->
    ExternResult<Entity<WebAppPackageVersionEntry>>
{
    let version = input.webapp_package_version_input;
    let entity = create_webapp_package_version_entry(CreateWebAppPackageVersionEntryInput {
        for_package: version.for_package.clone(),
        webapp: version.webapp,
        metadata: version.metadata,
        changelog: version.changelog,
        maintainer: version.maintainer,
        source_code_revision_uri: version.source_code_revision_uri,
    })?;

    create_webapp_package_link_to_version(CreateLinkWebAppPackageVersionInput {
	version: input.version,
	webapp_package_id: version.for_package,
	webapp_package_version_id: entity.id.clone(),
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
	webapp_package_version_id: input.webapp_package_version_id.clone(),
    })?;

    Ok( entity )
}