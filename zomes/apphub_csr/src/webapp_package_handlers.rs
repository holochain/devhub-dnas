use crate::{
    hdk,
    hdk_extensions,
    hdi_extensions,
    WebAppPackageAnchor,
    ALL_APPS_ANCHOR,
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
fn create_webapp_package_entry(input: CreateWebAppPackageEntryInput) -> ExternResult<Entity<WebAppPackageEntry>> {
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
    create_link( ALL_APPS_ANCHOR.clone(), entity.id.clone(), LinkTypes::WebAppPackage, () )?;

    Ok( entity )
}

#[hdk_extern]
fn get_webapp_package_entry(addr: AnyDhtHash) -> ExternResult<WebAppPackageEntry> {
    let record = must_get( &addr )?;

    Ok( WebAppPackageEntry::try_from_record( &record )? )
}

#[hdk_extern]
fn get_webapp_package(addr: ActionHash) -> ExternResult<Entity<WebAppPackageEntry>> {
    Ok( get_entity( &addr )? )
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LinkWebAppPackageVersionInput {
    pub version: String,
    pub webapp_package_id: EntityId,
    pub webapp_package_version_id: EntityId,
}

#[hdk_extern]
fn create_webapp_package_link_to_version(input: LinkWebAppPackageVersionInput) -> ExternResult<ActionHash> {
    create_link(
        input.webapp_package_id,
        input.webapp_package_version_id,
        LinkTypes::WebAppPackageVersion,
        input.version.as_bytes().to_vec()
    )
}


#[hdk_extern]
fn get_webapp_package_version_links(webapp_package_id: EntityId) ->
    ExternResult<EntityPointerMap>
{
    let anchor = WebAppPackageAnchor::new( &webapp_package_id );

    Ok( anchor.version_links()? )
}


#[hdk_extern]
fn get_webapp_package_versions(webapp_package_id: EntityId) ->
    ExternResult<WebAppPackageVersionMap>
{
    let anchor = WebAppPackageAnchor::new( &webapp_package_id );

    Ok( anchor.versions()? )
}


#[hdk_extern]
fn get_all_webapp_packages(_: ()) -> ExternResult<Vec<Entity<WebAppPackageEntry>>> {
    let webapps = get_links( ALL_APPS_ANCHOR.clone(), LinkTypes::WebAppPackage, None )?.into_iter()
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
    pub metadata: Option<BTreeMap<String, rmpv::Value>>,
}

#[hdk_extern]
fn update_webapp_package(input: UpdateEntityInput<UpdateWebAppPackageInput>) ->
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
            deprecation: None,
            metadata: changes.metadata
                .unwrap_or( package.metadata ),
        };

	Ok( entry )
    })?;

    Ok( entity )
}
