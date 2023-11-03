use crate::hdk;

use std::{
    str,
    collections::BTreeMap,
};
use hdk::prelude::*;
use apphub::{
    LinkTypes,
    hc_crud::{
        get_entity,
        EntityId,
    },
};
use apphub_sdk::{
    EntityPointerMap,
    WebAppPackageVersionMap,
};


pub struct WebAppPackageBase(pub EntityId);

impl WebAppPackageBase {
    pub fn new(id: &EntityId) -> Self {
        Self( id.to_owned() )
    }

    pub fn id(&self) -> EntityId {
        self.0.to_owned()
    }

    // pub fn package(&self) -> ExternResult<Entity<WebAppPackageEntry>> {
    //     Ok( get_entity( &self.id() )? )
    // }

    pub fn version_links(&self) -> ExternResult<Vec<Link>> {
        get_links(
            self.id(),
	    LinkTypes::WebAppPackageVersion,
	    None
        )
    }

    pub fn links_for_version(&self, version: &str ) -> ExternResult<Vec<Link>> {
        Ok(
            self.version_links()?.into_iter()
                .filter_map(|link| {
                    let tag = str::from_utf8( &link.tag.0 )
                        .map( |value| value.to_string() )
                        .map_err( |err| {
                            debug!("Failed to parse version from tag {:?}: {:#?}", link.tag, err );
                        }).ok()?;

                    match tag == version {
                        true => Some( link ),
                        false => None,
                    }
                })
                .collect()
        )
    }

    pub fn version_targets(&self) -> ExternResult<EntityPointerMap> {
        let version_links = self.version_links()?.iter()
            .filter_map(|link| {
                if let Some(target) = link.target.clone().into_action_hash() {
                    Some(( link, target ))
                } else {
                    debug!("Skipping link target because it is not an ActionHash; {:#?}", link.target );
                    None
                }
            })
            .filter_map(|(link, target)| {
                Some((
                    str::from_utf8( &link.tag.0 )
                        .map( |value| value.to_string() )
                        .map_err( |err| {
                            debug!("Failed to parse version from tag {:?}: {:#?}", link.tag, err );
                        }).ok()?,
                    target,
                ))
            })
            .collect();

        Ok( version_links )
    }

    pub fn versions(&self) -> ExternResult<WebAppPackageVersionMap> {
        let version_targets = self.version_targets()?;
        let mut version_map = BTreeMap::new();

        for (vname, version_id) in version_targets.into_iter() {
            debug!("Get WebApp package version: {}", version_id );
            let version = match get_entity( &version_id ) {
                Ok(value) => value,
                Err(err) => {
                    debug!("Dropping version '{}' because of failure to get version entry: {:#?}", vname, err );
                    continue;
                },
            };
            version_map.insert( vname, version );
        }

        Ok( version_map )
    }
}