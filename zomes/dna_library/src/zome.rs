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


fn zome_name_path(title: &str) -> AppResult<Path> {
    Ok( create_filter_path( "name", title )? )
}

fn filter_path(filter: &str, value: &str) -> AppResult<Path> {
    Ok( hc_crud::path_from_collection( vec![ "zome_by", filter, value ] )? )
}
fn create_filter_path(filter: &str, value: &str) -> AppResult<Path> {
    let path = filter_path( filter, value )?;
    path.ensure()?;

    Ok( path )
}



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

    let name_path = zome_name_path( &input.name )?;
    let name_path_hash = name_path.path_entry_hash()?;

    let name_path_lc = zome_name_path( &input.name.to_lowercase() )?;
    let name_path_lc_hash = name_path_lc.path_entry_hash()?;

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

    debug!("Linking 'name' path ({}) to ENTRY: {}", name_path_hash, entity.id );
    entity.link_from( &name_path_hash, TAG_ZOME.into() )?;

    debug!("Linking lowercase 'name' path ({}) to ENTRY: {}", name_path_lc_hash, entity.id );
    entity.link_from( &name_path_lc_hash, TAG_ZOME.into() )?;

    let all_zomes_path = crate::all_zomes_path();
    let all_zomes_hash = all_zomes_path.path_entry_hash()?;
    all_zomes_path.ensure()?;
    debug!("Linking all Zome path ({}) to ENTRY: {}", all_zomes_hash, entity.id );
    entity.link_from( &all_zomes_hash, TAG_ZOME.into() )?;

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



fn get_entities_for_links ( links: Vec<Link> ) -> Vec<Entity<ZomeSummary>> {
    links.into_iter()
	.filter_map(|link| {
	    get_entity::<ZomeEntry>( &link.target ).ok()
	})
	.map( |entity| {
	    entity.change_model( |zome| zome.to_summary() )
	})
	.collect()
}


pub fn get_zome_collection(maybe_pubkey: Option<AgentPubKey>) -> AppResult<Collection<Entity<ZomeSummary>>> {
    let base = crate::root_path_hash( maybe_pubkey )?;

    debug!("Getting ZOME links for Agent entry: {}", base );
    let links = get_links(
        base.to_owned(),
	Some( LinkTag::new( TAG_ZOME ) )
    )?;

    let zomes = get_entities_for_links( links );

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
    let mut previous_name = String::from("");

    let entity = update_entity(
	&input.addr,
	|current : ZomeEntry, _| {
	    previous_name = current.name.clone();

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

    let previous_name_path = zome_name_path( &previous_name )?;
    let previous_path_hash = previous_name_path.path_entry_hash()?;

    let new_name_path = zome_name_path( &entity.content.name )?;
    let new_path_hash = new_name_path.path_entry_hash()?;

    entity.move_link_from( TAG_ZOME.into(), &previous_path_hash, &new_path_hash )?;

    let previous_name_path = zome_name_path( &previous_name.to_lowercase() )?;
    let previous_path_hash = previous_name_path.path_entry_hash()?;

    let new_name_path = zome_name_path( &entity.content.name.to_lowercase() )?;
    let new_path_hash = new_name_path.path_entry_hash()?;

    entity.move_link_from( TAG_ZOME.into(), &previous_path_hash, &new_path_hash )?;

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


pub fn get_zomes_by_filter( filter: String, keyword: String ) -> AppResult<Collection<Entity<ZomeSummary>>> {
    let base = filter_path( &filter, &keyword )?.path_entry_hash()?;

    debug!("Getting hApp links for base: {:?}", base );
    let all_links = get_links(
        base.clone(),
	Some(LinkTag::new(TAG_ZOME))
    )?;

    let zomes = get_entities_for_links( all_links );

    Ok(Collection {
	base,
	items: zomes,
    })
}


pub fn get_all_zomes() -> AppResult<Collection<Entity<ZomeSummary>>> {
    let base = crate::all_zomes_path().path_entry_hash()?;

    debug!("Getting Zome links for base: {}", base );
    let links = get_links(
        base.clone(),
	Some(LinkTag::new(TAG_ZOME))
    )?;

    let zomes = get_entities_for_links( links );

    Ok(Collection {
	base,
	items: zomes,
    })
}
