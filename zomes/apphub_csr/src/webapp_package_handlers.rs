use crate::{
    hdk,
    hdk_extensions,
    hdi_extensions,
    WebAppPackageBase,
    MY_WEBAPP_PACKS_ANCHOR,
    ALL_WEBAPP_PACKS_ANCHOR,
};

use std::collections::BTreeMap;
use hdk::prelude::*;
use hdk_extensions::{
    must_get,
};
use hdi_extensions::{
    trace_origin_root,
    ScopedTypeConnector,
};
use apphub::{
    LinkTypes,
    WebAppPackageEntry,
    Authority,
    MemoryAddr,
    DeprecationNotice,
    hc_crud::{
        Entity, EntityId,
        UpdateEntityInput,
        EntryModel,
        create_entity, get_entity, update_entity,
    },
};
use apphub_sdk::{
    EntityPointerMap,
    WebAppPackageEntryInput,
    CreateWebAppPackageInput,
    WebAppPackageVersionMap,
};


fn create_webapp_package_entry_handler(entry: WebAppPackageEntry) ->
    ExternResult<Entity<WebAppPackageEntry>>
{
    let entity = create_entity( &entry )?;

    MY_WEBAPP_PACKS_ANCHOR.create_link_if_not_exists( &entity.id, () )?;
    ALL_WEBAPP_PACKS_ANCHOR.create_link_if_not_exists( &entity.id, () )?;

    Ok( entity )
}


#[hdk_extern]
pub fn create_webapp_package_entry(input: WebAppPackageEntryInput) ->
    ExternResult<Entity<WebAppPackageEntry>>
{
    create_webapp_package_entry_handler( input.into() )
}


#[hdk_extern]
pub fn create_webapp_package(input: CreateWebAppPackageInput) ->
    ExternResult<Entity<WebAppPackageEntry>>
{
    create_webapp_package_entry_handler( input.try_into()? )
}


#[hdk_extern]
pub fn get_webapp_package_entry(addr: AnyDhtHash) -> ExternResult<Entity<WebAppPackageEntry>> {
    let record = must_get( &addr )?;
    let content = WebAppPackageEntry::try_from_record( &record )?;
    let id = trace_origin_root( record.action_address() )?.0;
    let addr = hash_entry( content.clone() )?;

    Ok(
        Entity {
            id: id.clone(),
            action: id,
	    address: addr,
	    ctype: content.get_type(),
	    content: content,
        }
    )
}

#[hdk_extern]
pub fn get_webapp_package(addr: EntityId) -> ExternResult<Entity<WebAppPackageEntry>> {
    Ok( get_entity( &addr )? )
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateLinkWebAppPackageVersionInput {
    pub version: String,
    pub webapp_package_id: EntityId,
    pub webapp_package_version_id: EntityId,
}

#[hdk_extern]
pub fn create_webapp_package_link_to_version(input: CreateLinkWebAppPackageVersionInput) ->
    ExternResult<ActionHash>
{
    create_link(
        input.webapp_package_id,
        input.webapp_package_version_id,
        LinkTypes::WebAppPackageVersion,
        input.version.as_bytes().to_vec()
    )
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeleteLinkWebAppPackageVersionInput {
    pub version: String,
    pub webapp_package_id: EntityId,
}

#[hdk_extern]
pub fn delete_webapp_package_links_to_version(input: DeleteLinkWebAppPackageVersionInput) ->
    ExternResult<Vec<ActionHash>>
{
    let base = WebAppPackageBase::new( &input.webapp_package_id );
    let links = base.links_for_version( &input.version )?;

    let mut deleted_links = Vec::new();

    for link in links {
        delete_link( link.create_link_hash.clone() )?;
        deleted_links.push( link.create_link_hash );
    }

    Ok( deleted_links )
}


#[hdk_extern]
pub fn get_webapp_package_version_links(webapp_package_id: EntityId) ->
    ExternResult<Vec<Link>>
{
    let base = WebAppPackageBase::new( &webapp_package_id );

    Ok( base.version_links()? )
}


#[hdk_extern]
pub fn get_webapp_package_version_targets(webapp_package_id: EntityId) ->
    ExternResult<EntityPointerMap>
{
    let base = WebAppPackageBase::new( &webapp_package_id );

    Ok( base.version_targets()? )
}


#[hdk_extern]
pub fn get_webapp_package_versions(webapp_package_id: EntityId) ->
    ExternResult<WebAppPackageVersionMap>
{
    let base = WebAppPackageBase::new( &webapp_package_id );

    Ok( base.versions()? )
}


#[hdk_extern]
pub fn get_all_webapp_packages(_: ()) -> ExternResult<Vec<Entity<WebAppPackageEntry>>> {
    let webapps = ALL_WEBAPP_PACKS_ANCHOR.get_links( None )?.into_iter()
        .filter_map(|link| {
            let addr = link.target.into_action_hash()?;
            get_webapp_package( addr ).ok()
        })
        .collect();

    Ok( webapps )
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateWebAppPackageInput {
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub description: Option<String>,
    pub icon: Option<MemoryAddr>,
    pub maintainer: Option<Authority>,
    pub source_code_uri: Option<String>,
    pub deprecation: Option<DeprecationNotice>,
    pub metadata: Option<BTreeMap<String, rmpv::Value>>,
}

#[hdk_extern]
pub fn update_webapp_package(input: UpdateEntityInput<UpdateWebAppPackageInput>) ->
    ExternResult<Entity<WebAppPackageEntry>>
{
    let changes = input.properties;
    let entity = update_entity( &input.base, |package: WebAppPackageEntry, _| {
        let entry = WebAppPackageEntry {
            title: changes.title
                .unwrap_or( package.title ),
            subtitle: changes.subtitle
                .unwrap_or( package.subtitle ),
            description: changes.description
                .unwrap_or( package.description ),
            maintainer: changes.maintainer
                .unwrap_or( package.maintainer ).into(),
            icon: changes.icon
                .unwrap_or( package.icon ),
            source_code_uri: changes.source_code_uri
                .or( package.source_code_uri ),
            deprecation: changes.deprecation
                .or( package.deprecation ),
            metadata: changes.metadata
                .unwrap_or( package.metadata ),
        };

	Ok( entry )
    })?;

    Ok( entity )
}


#[hdk_extern]
pub fn deprecate_webapp_package(input: UpdateEntityInput<DeprecationNotice>) ->
    ExternResult<Entity<WebAppPackageEntry>>
{
    let entity = update_entity( &input.base, |mut package: WebAppPackageEntry, _| {
        package.deprecation = Some(input.properties.clone());

	Ok( package )
    })?;

    ALL_WEBAPP_PACKS_ANCHOR.delete_all_my_links_to_target( &entity.id, None )?;

    Ok( entity )
}
