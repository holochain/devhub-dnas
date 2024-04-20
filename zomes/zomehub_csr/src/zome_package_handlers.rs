use crate::{
    hdk,
    MY_ZOME_PACKS_ANCHOR,
};
use hdk::prelude::*;
use zomehub::{
    ZomePackageEntry,
    hc_crud::{
        Entity,
        create_entity,
    },
};
use zomehub_sdk::{
    CreateZomePackageInput,
};



#[hdk_extern]
fn create_zome_package_entry(input: ZomePackageEntry) -> ExternResult<Entity<ZomePackageEntry>> {
    let entity = create_entity( &input )?;

    MY_ZOME_PACKS_ANCHOR.create_link_if_not_exists( &entity.address, () )?;

    // TODO: Link from package name

    Ok( entity )
}


#[hdk_extern]
fn create_zome_package(input: CreateZomePackageInput) -> ExternResult<Entity<ZomePackageEntry>> {
    let entry : ZomePackageEntry = input.try_into()?;

    create_zome_package_entry( entry )
}
