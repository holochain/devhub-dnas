use crate::{
    hdk,
    hdk_extensions,
    ZomePackageBase,
};
use std::collections::BTreeMap;

use hdk::prelude::*;
use hdk_extensions::{
    must_get,
    hdi_extensions::{
        ScopedTypeConnector,
    },
};
use zomehub::{
    // LinkTypes,
    RmpvValue,
    Authority,
    ApiCompatibility,

    ZomePackageVersionEntry,
    hc_crud::{
        Entity, EntityId,
        EntryModel,
        create_entity, update_entity,
        UpdateEntityInput,
    },
};
use zomehub_sdk::{
    // LinkBase,
    EntityPointerMap,
    ZomePackageVersionMap,
    CreateZomePackageVersionInput,
};



#[hdk_extern]
fn create_zome_package_version_entry(input: ZomePackageVersionEntry) ->
    ExternResult<Entity<ZomePackageVersionEntry>>
{
    let entity = create_entity( &input )?;

    // TODO: Link from package

    Ok( entity )
}


#[hdk_extern]
fn create_zome_package_version(input: CreateZomePackageVersionInput) ->
    ExternResult<Entity<ZomePackageVersionEntry>>
{
    let entry : ZomePackageVersionEntry = input.clone().try_into()?;

    let entity = create_zome_package_version_entry( entry )?;

    create_zome_package_link_to_version(CreateLinkZomePackageVersionInput {
	version: input.version,
	zome_package_id: input.for_package,
	zome_package_version_addr: entity.id.clone(),
    })?;

    Ok( entity )
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateLinkZomePackageVersionInput {
    pub version: String,
    pub zome_package_id: EntityId,
    pub zome_package_version_addr: ActionHash,
}

#[hdk_extern]
pub fn create_zome_package_link_to_version(input: CreateLinkZomePackageVersionInput) ->
    ExternResult<ActionHash>
{
    let zome_base = ZomePackageBase::new( &input.zome_package_id );

    zome_base.create_version_link( &input.zome_package_version_addr, input.version.as_str() )
}


#[hdk_extern]
pub fn get_zome_package_version_links(zome_package_id: EntityId) ->
    ExternResult<Vec<Link>>
{
    let base = ZomePackageBase::new( &zome_package_id );

    Ok( base.version_links()? )
}


#[hdk_extern]
pub fn get_zome_package_version_targets(zome_package_id: EntityId) ->
    ExternResult<EntityPointerMap>
{
    let base = ZomePackageBase::new( &zome_package_id );

    Ok( base.version_targets()? )
}


#[hdk_extern]
pub fn get_zome_package_versions(zome_package_id: EntityId) ->
    ExternResult<ZomePackageVersionMap>
{
    let base = ZomePackageBase::new( &zome_package_id );

    Ok( base.versions()? )
}


#[hdk_extern]
fn get_zome_package_version_entry(addr: AnyDhtHash) ->
    ExternResult<Entity<ZomePackageVersionEntry>>
{
    let record = must_get( &addr )?;
    let content = ZomePackageVersionEntry::try_from_record( &record )?;
    let id = record.action_address().to_owned();
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


#[derive(Debug, Deserialize, Clone)]
pub struct UpdateProperties {
    pub for_package: Option<EntityId>,
    pub maintainer: Option<Authority>,
    pub changelog: Option<String>,
    pub source_code_revision_uri: Option<String>,
    pub api_compatibility: Option<ApiCompatibility>,
    pub metadata: Option<BTreeMap<String, RmpvValue>>,
}
pub type UpdateInput = UpdateEntityInput<UpdateProperties>;

#[hdk_extern]
pub fn update_zome_package_version(input: UpdateInput) -> ExternResult<Entity<ZomePackageVersionEntry>> {
    debug!("Updating zome package: {}", input.base );
    let props = input.properties.clone();

    let entity = update_entity(
	&input.base,
	|mut current : ZomePackageVersionEntry, _| {
	    current.for_package = props.for_package
		.unwrap_or( current.for_package );
	    current.maintainer = props.maintainer
		.unwrap_or( current.maintainer );
	    current.changelog = props.changelog
		.or( current.changelog );
	    current.source_code_revision_uri = props.source_code_revision_uri
		.or( current.source_code_revision_uri );
	    current.api_compatibility = props.api_compatibility
		.unwrap_or( current.api_compatibility );
	    current.metadata = props.metadata
		.unwrap_or( current.metadata );

	    Ok( current )
	})?;

    Ok( entity )
}
