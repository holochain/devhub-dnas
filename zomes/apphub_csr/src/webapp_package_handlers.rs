use crate::{
    hdk,
    hdk_extensions,
    hdi_extensions,
    WebAppPackageBase,
    ALL_WEBAPP_PACKS_ANCHOR,
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
    LinkTypes,
    WebAppPackageEntry,
    Authority,
    MemoryAddr,
    DeprecationNotice,
    hc_crud::{
        Entity, EntityId,
        UpdateEntityInput,
        create_entity, get_entity, update_entity,
    },
};
use apphub_sdk::{
    EntityPointerMap,
    WebAppPackageVersionMap,
};


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
    pub source_code_uri: Option<String>,
}

#[hdk_extern]
pub fn create_webapp_package_entry(input: CreateWebAppPackageEntryInput) ->
    ExternResult<Entity<WebAppPackageEntry>>
{
    let agent_id = hdk_extensions::agent_id()?;
    let entry = WebAppPackageEntry {
        title: input.title,
        subtitle: input.subtitle,
        description: input.description,
        maintainer: agent_id.clone().into(),
        icon: input.icon,
        source_code_uri: input.source_code_uri,
        deprecation: None,
        metadata: input.metadata,
    };

    let entity = create_entity( &entry )?;

    create_link( agent_id, entity.id.clone(), LinkTypes::WebAppPackage, () )?;

    ALL_WEBAPP_PACKS_ANCHOR.create_link_if_not_exists( &entity.id, () )?;

    Ok( entity )
}


#[hdk_extern]
pub fn create_webapp_package(input: CreateWebAppPackageEntryInput) ->
    ExternResult<Entity<WebAppPackageEntry>>
{
    create_webapp_package_entry( input )
}


#[hdk_extern]
pub fn get_webapp_package_entry(addr: AnyDhtHash) -> ExternResult<WebAppPackageEntry> {
    let record = must_get( &addr )?;

    Ok( WebAppPackageEntry::try_from_record( &record )? )
}

#[hdk_extern]
pub fn get_webapp_package(addr: ActionHash) -> ExternResult<Entity<WebAppPackageEntry>> {
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
