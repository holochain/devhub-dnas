use devhub_types::{
    constants::{ AppResult },
    dna_entry_types::{ DnaEntry, DnaInfo, DnaSummary, DeveloperProfileLocation, DeprecationNotice },
};
use hc_entities::{ Entity, Collection, UpdateEntityInput };
use hc_dna_utils as utils;
use hdk::prelude::*;

use crate::constants::{ TAG_DNA };



#[derive(Debug, Deserialize)]
pub struct DnaInput {
    pub name: String,
    pub description: String,

    // optional
    pub icon: Option<SerializedBytes>,
    pub published_at: Option<u64>,
    pub last_updated: Option<u64>,
    pub collaborators: Option<Vec<(AgentPubKey, String)>>,
}

pub fn create_dna(input: DnaInput) -> AppResult<Entity<DnaInfo>> {
    debug!("Creating DNA: {}", input.name );
    let pubkey = agent_info()?.agent_initial_pubkey;
    let default_now = utils::now()?;

    let dna = DnaEntry {
	name: input.name,
	description: input.description,
	icon: input.icon,
	published_at: input.published_at
	    .unwrap_or( default_now ),
	last_updated: input.last_updated
	    .unwrap_or( default_now ),
	collaborators: input.collaborators,
	developer: DeveloperProfileLocation {
	    pubkey: pubkey.clone(),
	},
	deprecation: None,
    };

    let entity = utils::create_entity( &dna )?
	.new_content( dna.to_info() );

    debug!("Linking pubkey ({}) to ENTRY: {}", pubkey, entity.id );
    create_link(
	pubkey.into(),
	entity.id.clone(),
	LinkTag::new( TAG_DNA )
    )?;

    Ok( entity )
}




#[derive(Debug, Deserialize)]
pub struct GetDnaInput {
    pub id: EntryHash,
}

pub fn get_dna(input: GetDnaInput) -> AppResult<Entity<DnaInfo>> {
    debug!("Get DNA: {}", input.id );
    let entity = utils::get_entity( &input.id )?;
    let info = DnaEntry::try_from( &entity.content )?.to_info();

    Ok( entity.new_content( info ) )
}



pub fn get_dna_links(maybe_pubkey: Option<AgentPubKey>) -> AppResult<(EntryHash, Vec<Link>)> {
    let base : EntryHash = match maybe_pubkey {
	None => agent_info()?.agent_initial_pubkey,
	Some(agent) => agent,
    }.into();

    debug!("Getting DNA links for Agent entry: {}", base );
    let all_links: Vec<Link> = get_links(
        base.clone(),
	Some(LinkTag::new(TAG_DNA))
    )?.into();

    Ok( (base, all_links) )
}

#[derive(Debug, Deserialize)]
pub struct GetDnasInput {
    pub agent: Option<AgentPubKey>,
}

pub fn get_dnas(input: GetDnasInput) -> AppResult<Collection<Entity<DnaSummary>>> {
    let (base, links) = get_dna_links( input.agent.clone() )?;

    let dnas = links.into_iter()
	.filter_map(|link| {
	    utils::get_entity( &link.target ).ok()
	})
	.filter_map(|entity| {
	    let mut maybe_entity : Option<Entity<DnaSummary>> = None;

	    if let Some(dna) = DnaEntry::try_from( &entity.content ).ok() {
		if dna.deprecation.is_none() {
		    let summary = dna.to_summary();
		    let entity = entity.new_content( summary );

		    maybe_entity.replace( entity );
		}
	    }

	    maybe_entity
	})
	.collect();

    Ok(Collection {
	base,
	items: dnas
    })
}

pub fn get_deprecated_dnas(input: GetDnasInput) -> AppResult<Collection<Entity<DnaSummary>>> {
    let (base, links) = get_dna_links( input.agent.clone() )?;

    let dnas = links.into_iter()
	.filter_map(|link| {
	    utils::get_entity( &link.target ).ok()
	})
	.filter_map(|entity| {
	    let mut maybe_entity : Option<Entity<DnaSummary>> = None;

	    if let Some(dna) = DnaEntry::try_from(&entity.content).ok() {
		if dna.deprecation.is_some() {
		    let summary = dna.to_summary();
		    let entity = entity.new_content( summary );

		    maybe_entity.replace( entity );
		}
	    }

	    maybe_entity
	})
	.collect();

    Ok(Collection {
	base,
	items: dnas
    })
}

pub fn get_my_dnas() -> AppResult<Collection<Entity<DnaSummary>>> {
    get_dnas(GetDnasInput {
	agent: None,
    })
}

pub fn get_my_deprecated_dnas() -> AppResult<Collection<Entity<DnaSummary>>> {
    get_deprecated_dnas(GetDnasInput {
	agent: None,
    })
}




#[derive(Debug, Deserialize)]
pub struct DnaUpdateOptions {
    pub name: Option<String>,
    pub description: Option<String>,
    pub icon: Option<SerializedBytes>,
    pub published_at: Option<u64>,
    pub last_updated: Option<u64>,
    pub collaborators: Option<Vec<(AgentPubKey, String)>>,
}
pub type DnaUpdateInput = UpdateEntityInput<DnaUpdateOptions>;

pub fn update_dna(input: DnaUpdateInput) -> AppResult<Entity<DnaInfo>> {
    debug!("Updating DNA: {}", input.addr );
    let props = input.properties;

    let entity : Entity<DnaEntry> = utils::update_entity(
	input.id, input.addr,
	|element| {
	    let current = DnaEntry::try_from( &element )?;

	    Ok(DnaEntry {
		name: props.name
		    .unwrap_or( current.name ),
		description: props.description
		    .unwrap_or( current.description ),
		icon: props.icon
		    .or( current.icon ),
		published_at: props.published_at
		    .unwrap_or( current.published_at ),
		last_updated: props.last_updated
		    .unwrap_or( utils::now()? ),
		collaborators: props.collaborators
		    .or( current.collaborators ),
		developer: current.developer,
		deprecation: current.deprecation,
	    })
	})?;

    let info = entity.content.to_info();

    Ok( entity.new_content( info ) )
}




#[derive(Debug, Deserialize)]
pub struct DeprecateDnaInput {
    pub addr: EntryHash,
    pub message: String,
}

pub fn deprecate_dna(input: DeprecateDnaInput) -> AppResult<Entity<DnaInfo>> {
    debug!("Deprecating DNA: {}", input.addr );
    let entity : Entity<DnaEntry> = utils::update_entity(
	None, input.addr.clone(),
	|element| {
	    let current = DnaEntry::try_from( &element )?;

	    Ok(DnaEntry {
		name: current.name,
		description: current.description,
		icon: current.icon,
		published_at: current.published_at,
		last_updated: current.last_updated,
		collaborators: None,
		developer: current.developer,
		deprecation: Some(DeprecationNotice::new( input.message )),
	    })
	})?;

    let info = entity.content.to_info();

    Ok( entity.new_content( info ) )
}
