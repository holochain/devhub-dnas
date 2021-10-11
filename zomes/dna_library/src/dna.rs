use devhub_types::{
    AppResult, UpdateEntityInput,
    dnarepo_entry_types::{ DnaEntry, DnaInfo, DnaSummary, DeveloperProfileLocation, DeprecationNotice },
};
use hc_crud::{
    now, create_entity, get_entity, update_entity,
    Entity, Collection,
};
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
    let default_now = now()?;

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

    let entity = create_entity( &dna )?
	.change_model( |dna| dna.to_info() );
    let base = crate::root_path_hash( None )?;

    debug!("Linking pubkey ({}) to ENTRY: {}", pubkey, entity.id );
    entity.link_from( &base, TAG_DNA.into() )?;

    Ok( entity )
}




#[derive(Debug, Deserialize)]
pub struct GetDnaInput {
    pub id: EntryHash,
}

pub fn get_dna(input: GetDnaInput) -> AppResult<Entity<DnaInfo>> {
    debug!("Get DNA: {}", input.id );
    let entity = get_entity::<DnaEntry>( &input.id )?;

    Ok( entity.change_model( |dna| dna.to_info() ) )
}



pub fn get_dna_collection(maybe_pubkey: Option<AgentPubKey>) -> AppResult<Collection<Entity<DnaSummary>>> {
    let base = crate::root_path_hash( maybe_pubkey )?;

    debug!("Getting DNA links for Agent entry: {}", base );
    let all_links: Vec<Link> = get_links(
        base.clone(),
	Some(LinkTag::new(TAG_DNA))
    )?.into();

    let dnas = all_links.into_iter()
	.filter_map(|link| {
	    get_entity::<DnaEntry>( &link.target ).ok()
	})
	.map( |entity| {
	    entity.change_model( |dna| dna.to_summary() )
	})
	.collect();

    Ok(Collection {
	base,
	items: dnas,
    })
}

#[derive(Debug, Deserialize)]
pub struct GetDnasInput {
    pub agent: Option<AgentPubKey>,
}

pub fn get_dnas(input: GetDnasInput) -> AppResult<Collection<Entity<DnaSummary>>> {
    let dna_collection = get_dna_collection( input.agent.clone() )?;

    let dnas = dna_collection.items.into_iter()
	.filter(|entity| {
	    entity.content.deprecation.is_none()
	})
	.collect();

    Ok(Collection {
	base: dna_collection.base,
	items: dnas,
    })
}

pub fn get_deprecated_dnas(input: GetDnasInput) -> AppResult<Collection<Entity<DnaSummary>>> {
    let dna_collection = get_dna_collection( input.agent.clone() )?;

    let dnas = dna_collection.items.into_iter()
	.filter(|entity| {
	    entity.content.deprecation.is_some()
	})
	.collect();

    Ok(Collection {
	base: dna_collection.base,
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

    let entity : Entity<DnaEntry> = update_entity(
	&input.addr,
	|current : DnaEntry, _| {
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
		    .unwrap_or( now()? ),
		collaborators: props.collaborators
		    .or( current.collaborators ),
		developer: current.developer,
		deprecation: current.deprecation,
	    })
	})?;

    Ok( entity.change_model( |dna| dna.to_info() ) )
}




#[derive(Debug, Deserialize)]
pub struct DeprecateDnaInput {
    pub addr: EntryHash,
    pub message: String,
}

pub fn deprecate_dna(input: DeprecateDnaInput) -> AppResult<Entity<DnaInfo>> {
    debug!("Deprecating DNA: {}", input.addr );
    let entity : Entity<DnaEntry> = update_entity(
	&input.addr,
	|current : DnaEntry, _| {
	    Ok(DnaEntry {
		name: current.name,
		description: current.description,
		icon: current.icon,
		published_at: current.published_at,
		last_updated: current.last_updated,
		collaborators: None,
		developer: current.developer,
		deprecation: Some(DeprecationNotice::new( input.message.to_owned() )),
	    })
	})?;

    Ok( entity.change_model( |dna| dna.to_info() ) )
}
