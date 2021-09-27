use devhub_types::{
    AppResult,
    dnarepo_entry_types::{ ZomeEntry, ZomeInfo, ZomeSummary, DeveloperProfileLocation, DeprecationNotice },
};
use hc_crud::{
    now, create_entity, get_entity, update_entity,
    Entity, Collection, UpdateEntityInput
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
	.new_content( zome.to_info() );

    debug!("Linking pubkey ({}) to ENTRY: {}", pubkey, entity.id );
    create_link(
	pubkey.into(),
	entity.id.clone(),
	LinkTag::new( TAG_ZOME )
    )?;

    Ok( entity )
}




#[derive(Debug, Deserialize)]
pub struct GetZomeInput {
    pub id: EntryHash,
}

pub fn get_zome(input: GetZomeInput) -> AppResult<Entity<ZomeInfo>> {
    debug!("Get ZOME: {}", input.id );
    let entity = get_entity( &input.id )?;
    let info = ZomeEntry::try_from( &entity.content )?.to_info();

    Ok( entity.new_content( info ) )
}



pub fn get_zome_links(maybe_pubkey: Option<AgentPubKey>) -> AppResult<(EntryHash, Vec<Link>)> {
    let base : EntryHash = match maybe_pubkey {
	None => agent_info()?.agent_initial_pubkey,
	Some(agent) => agent,
    }.into();

    debug!("Getting ZOME links for Agent entry: {}", base );
    let all_links: Vec<Link> = get_links(
        base.clone(),
	Some(LinkTag::new(TAG_ZOME))
    )?.into();

    Ok( (base, all_links) )
}

#[derive(Debug, Deserialize)]
pub struct GetZomesInput {
    pub agent: Option<AgentPubKey>,
}

pub fn get_zomes(input: GetZomesInput) -> AppResult<Collection<Entity<ZomeSummary>>> {
    let (base, links) = get_zome_links( input.agent.clone() )?;

    let zomes = links.into_iter()
	.filter_map(|link| {
	    get_entity( &link.target ).ok()
	})
	.filter_map(|entity| {
	    let mut maybe_entity : Option<Entity<ZomeSummary>> = None;

	    if let Some(zome) = ZomeEntry::try_from( &entity.content ).ok() {
		if zome.deprecation.is_none() {
		    let summary = zome.to_summary();
		    let entity = entity.new_content( summary );

		    maybe_entity.replace( entity );
		}
	    }

	    maybe_entity
	})
	.collect();

    Ok(Collection {
	base,
	items: zomes
    })
}

pub fn get_deprecated_zomes(input: GetZomesInput) -> AppResult<Collection<Entity<ZomeSummary>>> {
    let (base, links) = get_zome_links( input.agent.clone() )?;

    let zomes = links.into_iter()
	.filter_map(|link| {
	    get_entity( &link.target ).ok()
	})
	.filter_map(|entity| {
	    let mut maybe_entity : Option<Entity<ZomeSummary>> = None;

	    if let Some(zome) = ZomeEntry::try_from(&entity.content).ok() {
		if zome.deprecation.is_some() {
		    let summary = zome.to_summary();
		    let entity = entity.new_content( summary );

		    maybe_entity.replace( entity );
		}
	    }

	    maybe_entity
	})
	.collect();

    Ok(Collection {
	base,
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

    let entity : Entity<ZomeEntry> = update_entity(
	input.id, input.addr,
	|element| {
	    let current = ZomeEntry::try_from( &element )?;

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

    let info = entity.content.to_info();

    Ok( entity.new_content( info ) )
}




#[derive(Debug, Deserialize)]
pub struct DeprecateZomeInput {
    pub addr: EntryHash,
    pub message: String,
}

pub fn deprecate_zome(input: DeprecateZomeInput) -> AppResult<Entity<ZomeInfo>> {
    debug!("Deprecating ZOME: {}", input.addr );
    let entity : Entity<ZomeEntry> = update_entity(
	None, input.addr.clone(),
	|element| {
	    let current = ZomeEntry::try_from( &element )?;

	    Ok(ZomeEntry {
		name: current.name,
		description: current.description,
		published_at: current.published_at,
		last_updated: current.last_updated,
		developer: current.developer,
		deprecation: Some(DeprecationNotice::new( input.message )),
	    })
	})?;

    let info = entity.content.to_info();

    Ok( entity.new_content( info ) )
}
