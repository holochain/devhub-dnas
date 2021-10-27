use devhub_types::{
    AppResult, UpdateEntityInput,
    dnarepo_entry_types::{ ZomeEntry, ZomeInfo, ZomeSummary, DeveloperProfileLocation, DeprecationNotice },
};
use hc_crud::{
    now, create_entity, get_entity, update_entity,
    Entity, Collection,
};
use hdk::prelude::*;

use crate::constants::{ TAG_ZOME };



#[derive(Debug, Deserialize)]
pub struct ZomeInput {
    pub name: String,
    pub description: String,

    // optional
    pub published_at: Option<u64>,
    pub last_updated: Option<u64>,
}

pub fn create_zome(input: ZomeInput) -> AppResult<Entity<ZomeInfo>> {
    debug!("Creating ZOME: {}", input.name );
    let pubkey = agent_info()?.agent_initial_pubkey;
    let default_now = now()?;

    let zome = ZomeEntry {
	name: input.name,
	description: input.description,
	published_at: input.published_at
	    .unwrap_or( default_now ),
	last_updated: input.last_updated
	    .unwrap_or( default_now ),
	developer: DeveloperProfileLocation {
	    pubkey: pubkey.clone(),
	},
	deprecation: None,
    };

    let entity = create_entity( &zome )?
	.change_model( |zome| zome.to_info() );
    let base = crate::root_path_hash( None )?;

    debug!("Linking pubkey ({}) to ENTRY: {}", pubkey, entity.id );
    entity.link_from( &base, TAG_ZOME.into() )?;

    Ok( entity )
}




#[derive(Debug, Deserialize)]
pub struct GetZomeInput {
    pub id: EntryHash,
}

pub fn get_zome(input: GetZomeInput) -> AppResult<Entity<ZomeInfo>> {
    debug!("Get ZOME: {}", input.id );
    let entity = get_entity::<ZomeEntry>( &input.id )?;

    Ok( entity.change_model( |zome| zome.to_info() ) )
}

pub fn get_zome_collection(maybe_pubkey: Option<AgentPubKey>) -> AppResult<Collection<Entity<ZomeSummary>>> {
    let base = crate::root_path_hash( maybe_pubkey )?;

    debug!("Getting ZOME links for Agent entry: {}", base );
    let links: Vec<Link> = get_links(
        base.to_owned(),
	Some( LinkTag::new( TAG_ZOME ) )
    )?.into();

    let zomes = links.into_iter()
	.filter_map(|link| {
	    get_entity::<ZomeEntry>( &link.target ).ok()
	})
	.map( |entity| {
	    entity.change_model( |zome| zome.to_summary() )
	})
	.collect();

    Ok(Collection {
	base,
	items: zomes,
    })
}

#[derive(Debug, Deserialize)]
pub struct GetZomesInput {
    pub agent: Option<AgentPubKey>,
}

pub fn get_zomes(input: GetZomesInput) -> AppResult<Collection<Entity<ZomeSummary>>> {
    let zome_collection = get_zome_collection( input.agent.clone() )?;

    let zomes = zome_collection.items.into_iter()
	.filter( |entity| {
	    !entity.content.deprecation
	})
	.collect();

    Ok(Collection {
	base: zome_collection.base,
	items: zomes,
    })
}

pub fn get_deprecated_zomes(input: GetZomesInput) -> AppResult<Collection<Entity<ZomeSummary>>> {
    let zome_collection = get_zome_collection( input.agent.clone() )?;

    let zomes = zome_collection.items.into_iter()
	.filter( |entity| {
	    entity.content.deprecation
	})
	.collect();

    Ok(Collection {
	base: zome_collection.base,
	items: zomes
    })
}

pub fn get_my_zomes() -> AppResult<Collection<Entity<ZomeSummary>>> {
    get_zomes(GetZomesInput {
	agent: None,
    })
}

pub fn get_my_deprecated_zomes() -> AppResult<Collection<Entity<ZomeSummary>>> {
    get_deprecated_zomes(GetZomesInput {
	agent: None,
    })
}




#[derive(Debug, Deserialize)]
pub struct ZomeUpdateOptions {
    pub name: Option<String>,
    pub description: Option<String>,
    pub published_at: Option<u64>,
    pub last_updated: Option<u64>,
}
pub type ZomeUpdateInput = UpdateEntityInput<ZomeUpdateOptions>;

pub fn update_zome(input: ZomeUpdateInput) -> AppResult<Entity<ZomeInfo>> {
    debug!("Updating ZOME: {}", input.addr );
    let props = input.properties;

    let entity = update_entity(
	&input.addr,
	|current : ZomeEntry, _| {
	    Ok(ZomeEntry {
		name: props.name
		    .unwrap_or( current.name ),
		description: props.description
		    .unwrap_or( current.description ),
		published_at: props.published_at
		    .unwrap_or( current.published_at ),
		last_updated: props.last_updated
		    .unwrap_or( now()? ),
		developer: current.developer,
		deprecation: current.deprecation,
	    })
	})?;

    Ok( entity.change_model( |zome| zome.to_info() ) )
}




#[derive(Debug, Deserialize)]
pub struct DeprecateZomeInput {
    pub addr: EntryHash,
    pub message: String,
}

pub fn deprecate_zome(input: DeprecateZomeInput) -> AppResult<Entity<ZomeInfo>> {
    debug!("Deprecating ZOME: {}", input.addr );
    let entity : Entity<ZomeEntry> = update_entity(
	&input.addr,
	|current : ZomeEntry, _| {
	    Ok(ZomeEntry {
		name: current.name,
		description: current.description,
		published_at: current.published_at,
		last_updated: current.last_updated,
		developer: current.developer,
		deprecation: Some(DeprecationNotice::new( input.message.to_owned() )),
	    })
	})?;

    Ok( entity.change_model( |zome| zome.to_info() ) )
}
