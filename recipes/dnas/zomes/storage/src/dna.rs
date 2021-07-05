use devhub_types::{ DevHubResponse, EntityResponse, EntityCollectionResponse,
		    ENTITY_MD, ENTITY_COLLECTION_MD };
use hc_entities::{ Entity, Collection, EntryModel };
use hc_dna_utils as utils;
use crate::catch;
use hdk::prelude::*;

use crate::constants::{ TAG_DNA };
use crate::entry_types::{ DnaEntry, DnaInfo, DnaSummary, DeveloperProfileLocation, DeprecationNotice };



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

#[hdk_extern]
fn create_dna(input: DnaInput) -> ExternResult<EntityResponse<DnaInfo>> {
    debug!("Creating DNA: {}", input.name );
    let pubkey = agent_info()?.agent_initial_pubkey;

    let dna = DnaEntry {
	name: input.name,
	description: input.description,
	icon: input.icon,
	published_at: input.published_at
	    .unwrap_or( sys_time()?.as_millis() as u64 ),
	last_updated: input.last_updated
	    .unwrap_or( sys_time()?.as_millis() as u64 ),
	collaborators: input.collaborators,
	developer: DeveloperProfileLocation {
	    pubkey: pubkey.clone(),
	},
	deprecation: None,
    };

    let header_hash = create_entry(&dna)?;
    let entry_hash = hash_entry(&dna)?;

    debug!("Linking pubkey ({}) to DNA: {}", pubkey, entry_hash );
    create_link(
	pubkey.into(),
	entry_hash.clone(),
	LinkTag::new(TAG_DNA)
    )?;

    let info = dna.to_info();

    Ok( EntityResponse::success(Entity {
	id: entry_hash.clone(),
	address: entry_hash,
	header: header_hash,
	ctype: info.get_type(),
	content: info,
    }, ENTITY_MD ))
}




#[derive(Debug, Deserialize)]
pub struct GetDnaInput {
    pub addr: EntryHash,
}

#[hdk_extern]
fn get_dna(input: GetDnaInput) -> ExternResult<EntityResponse<DnaInfo>> {
    debug!("Get DNA: {}", input.addr );
    let entity = catch!( utils::get_entity( &input.addr ) );
    let info = catch!( DnaEntry::try_from(&entity.content) ).to_info();

    Ok( EntityResponse::success(
	entity.new_content( info ), ENTITY_MD
    ))
}



fn get_dna_links(maybe_pubkey: Option<AgentPubKey>) -> ExternResult<(EntryHash, Vec<Link>)> {
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

#[hdk_extern]
fn get_my_dnas(_:()) -> ExternResult<EntityCollectionResponse<DnaSummary>> {
    get_dnas(GetDnasInput {
	agent: None,
    })
}

#[hdk_extern]
fn get_my_deprecated_dnas(_:()) -> ExternResult<EntityCollectionResponse<DnaSummary>> {
    get_deprecated_dnas(GetDnasInput {
	agent: None,
    })
}

#[derive(Debug, Deserialize)]
pub struct GetDnasInput {
    pub agent: Option<AgentPubKey>,
}

#[hdk_extern]
fn get_dnas(input: GetDnasInput) -> ExternResult<EntityCollectionResponse<DnaSummary>> {
    let (base, links) = catch!( get_dna_links( input.agent.clone() ) );

    let dnas = links.into_iter()
	.filter_map(|link| {
	    utils::get_entity( &link.target ).ok()
	})
	.filter_map(|entity| {
	    let mut answer : Option<Entity<DnaSummary>> = None;

	    if let Some(dna) = DnaEntry::try_from(&entity.content).ok() {
		if dna.deprecation.is_none() {
		    answer.replace( entity.new_content( dna.to_summary() ) );
		}
	    }

	    answer
	})
	.collect();
    Ok( EntityCollectionResponse::success(Collection {
	base,
	items: dnas
    }, ENTITY_COLLECTION_MD) )
}

#[hdk_extern]
fn get_deprecated_dnas(input: GetDnasInput) -> ExternResult<EntityCollectionResponse<DnaSummary>> {
    let (base, links) = catch!( get_dna_links( input.agent.clone() ) );

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

    Ok( EntityCollectionResponse::success(Collection {
	base,
	items: dnas
    }, ENTITY_COLLECTION_MD) )
}




#[derive(Debug, Deserialize)]
pub struct UpdateDnaInput {
    pub id: Option<EntryHash>,
    pub addr: EntryHash,
    pub properties: DnaUpdateOptions
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

#[hdk_extern]
fn update_dna(input: UpdateDnaInput) -> ExternResult<EntityResponse<DnaInfo>> {
    debug!("Updating DNA: {}", input.addr );
    let id = match input.id {
	Some(id) => id,
	None => {
	    catch!( utils::get_id_for_addr( input.addr.clone() ) )
	},
    };
    let (header, element) = catch!( utils::fetch_entry( input.addr.clone() ) );
    let current_dna = catch!( DnaEntry::try_from( &element ) );

    let dna = DnaEntry {
	name: input.properties.name
	    .unwrap_or( current_dna.name ),
	description: input.properties.description
	    .unwrap_or( current_dna.description ),
	icon: input.properties.icon
	    .or( current_dna.icon ),
	published_at: input.properties.published_at
	    .unwrap_or( current_dna.published_at ),
	last_updated: input.properties.last_updated
	    .unwrap_or( sys_time()?.as_millis() as u64 ),
	collaborators: input.properties.collaborators
	    .or( current_dna.collaborators ),
	developer: current_dna.developer,
	deprecation: current_dna.deprecation,
    };

    let header_hash = catch!( update_entry(header, &dna) );
    let entry_hash = catch!( hash_entry(&dna) );

    debug!("Linking original ({}) to DNA: {}", input.addr, entry_hash );
    catch!( create_link(
	input.addr.clone(),
	entry_hash.clone(),
	LinkTag::new(utils::TAG_UPDATE)
    ) );

    debug!("Linking DNA ({}) to original: {}", entry_hash, input.addr );
    catch!( create_link(
	entry_hash.clone(),
	input.addr.clone(),
	LinkTag::new(utils::TAG_ORIGIN)
    ) );

    let content = dna.to_info();

    Ok(EntityResponse::success(
	Entity {
	    id: id,
	    header: header_hash,
	    address: entry_hash,
	    ctype: content.get_type(),
	    content: content,
	},
	ENTITY_MD
    ))
}




#[derive(Debug, Deserialize)]
pub struct DeprecateDnaInput {
    pub addr: EntryHash,
    pub message: String,
}

#[hdk_extern]
fn deprecate_dna(input: DeprecateDnaInput) -> ExternResult<EntityResponse<DnaInfo>> {
    debug!("Deprecating DNA: {}", input.addr );
    let entity = catch!( utils::get_entity( &input.addr ) );
    let current_dna = catch!( DnaEntry::try_from( &entity.content ) );

    let dna = DnaEntry {
	name: current_dna.name,
	description: current_dna.description,
	icon: current_dna.icon,
	published_at: current_dna.published_at,
	last_updated: current_dna.last_updated,
	collaborators: None,
	developer: current_dna.developer,
	deprecation: Some(DeprecationNotice::new( input.message )),
    };

    let header_hash = catch!( update_entry(entity.header.clone(), &dna) );
    let entry_hash = catch!( hash_entry(&dna) );

    debug!("Linking original ({}) to DNA: {}", input.addr, entry_hash );
    catch!( create_link(
	input.addr.clone(),
	entry_hash.clone(),
	LinkTag::new(utils::TAG_UPDATE)
    ) );

    Ok(EntityResponse::success(
	entity.new_content( dna.to_info() )
	    .update_header( header_hash )
	    .update_address( entry_hash ),
	ENTITY_MD
    ))
}
