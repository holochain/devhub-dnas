use crate::hdk;
use crate::hdk_extensions;

use std::collections::BTreeMap;
use hdk::prelude::*;
use app_hub::{
    LinkTypes,
    WebAppPackageEntry,
    WebAppPackageVersionEntry,
    Authority,
    MemoryAddr,
    hc_crud::{
        Entity, EntityId,
        create_entity, get_entity,
    },
};
use app_hub_sdk::{
    WebAppPackageAnchor,
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


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LinkWebAppPackageVersionInput {
    pub version: String,
    pub webapp_package_id: EntityId,
    pub webapp_package_version_id: EntityId,
}

#[hdk_extern]
fn link_webapp_package_version(input: LinkWebAppPackageVersionInput) -> ExternResult<ActionHash> {
    create_link(
        input.webapp_package_id,
        input.webapp_package_version_id,
        LinkTypes::WebAppPackageVersion,
        input.version.as_bytes().to_vec()
    )
}


type VersionMap<T> = BTreeMap<String, T>;
type WebAppPackageVersionMap = VersionMap<Entity<WebAppPackageVersionEntry>>;

#[hdk_extern]
fn get_webapp_package_versions(webapp_package_id: EntityId) ->
    ExternResult<WebAppPackageVersionMap>
{
    let anchor = WebAppPackageAnchor::new( &webapp_package_id );

    Ok( anchor.versions()? )
}
