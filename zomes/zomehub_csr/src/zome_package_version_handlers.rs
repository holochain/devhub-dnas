use crate::{
    hdk,
    hdk_extensions,
    ZomePackageBase,
};
use hdk::prelude::*;
use hdk_extensions::{
    must_get,
    hdi_extensions::{
        ScopedTypeConnector,
    },
};
use zomehub::{
    // LinkTypes,

    ZomePackageVersionEntry,
    hc_crud::{
        Entity, EntityId,
        EntryModel,
        create_entity,
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
    let entity = create_zome_package_version_entry( input.clone().try_into()? )?;

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
