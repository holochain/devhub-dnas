use hdk::prelude::*;
use hc_dna_reply_types::{ ReplyWithSingle, ReplyWithCollection, Entity, EntryModel };
use hc_dna_utils as utils;

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
fn create_dna(input: DnaInput) -> ExternResult<ReplyWithSingle<DnaInfo>> {
    debug!("Creating DNA: {}", input.name );
    let pubkey = agent_info()?.agent_initial_pubkey;

    let dna = DnaEntry {
	name: input.name,
	description: input.description,
	icon: input.icon,
	published_at: match input.published_at {
	    None => {
		sys_time()?.as_millis() as u64
	    },
	    Some(t) => t,
	},
	last_updated: match input.last_updated {
	    None => {
		sys_time()?.as_millis() as u64
	    },
	    Some(t) => t,
	},
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
	LinkTag::new(*TAG_DNA)
    )?;

    let info = dna.to_info();

    Ok( ReplyWithSingle::new(Entity {
	id: entry_hash.clone(),
	address: entry_hash,
	header: header_hash,
	ctype: info.get_type(),
	content: info,
    }) )
}




#[derive(Debug, Deserialize)]
pub struct GetDnaInput {
    pub addr: EntryHash,
}

#[hdk_extern]
fn get_dna(input: GetDnaInput) -> ExternResult<ReplyWithSingle<DnaInfo>> {
    debug!("Get DNA: {}", input.addr );
    let entity = utils::fetch_entity(input.addr)?;
    let info = DnaEntry::try_from(&entity.content)?.to_info();

    Ok( ReplyWithSingle::new(
	entity.replace_content( info )
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
	Some(LinkTag::new(*TAG_DNA))
    )?.into();

    Ok( (base, all_links) )
}

#[hdk_extern]
fn get_my_dnas(_:()) -> ExternResult<ReplyWithCollection<DnaSummary>> {
    get_dnas(GetDnasInput {
	agent: None,
    })
}

#[hdk_extern]
fn get_my_deprecated_dnas(_:()) -> ExternResult<ReplyWithCollection<DnaSummary>> {
    get_deprecated_dnas(GetDnasInput {
	agent: None,
    })
}

#[derive(Debug, Deserialize)]
pub struct GetDnasInput {
    pub agent: Option<AgentPubKey>,
}

#[hdk_extern]
fn get_dnas(input: GetDnasInput) -> ExternResult<ReplyWithCollection<DnaSummary>> {
    let (base, links) = get_dna_links( input.agent.clone() )?;

    let dnas = links.into_iter()
	.filter_map(|link| {
	    utils::fetch_entity(link.target).ok()
	})
	.filter_map(|entity| {
	    let mut answer : Option<Entity<DnaSummary>> = None;

	    if let Some(dna) = DnaEntry::try_from(&entity.content).ok() {
		if dna.deprecation.is_none() {
		    answer.replace( entity.replace_content( dna.to_summary() ) );
		}
	    }

	    answer
	})
	.collect();
    Ok( ReplyWithCollection::new( base, dnas ) )
}

#[hdk_extern]
fn get_deprecated_dnas(input: GetDnasInput) -> ExternResult<ReplyWithCollection<DnaSummary>> {
    let (base, links) = get_dna_links( input.agent.clone() )?;

    let dnas = links.into_iter()
	.filter_map(|link| {
	    utils::fetch_entity(link.target).ok()
	})
	.filter_map(|entity| {
	    let mut answer : Option<Entity<DnaSummary>> = None;

	    if let Some(dna) = DnaEntry::try_from(&entity.content).ok() {
		if dna.deprecation.is_some() {
		    answer.replace( entity.replace_content( dna.to_summary() ) );
		}
	    }

	    answer
	})
	.collect();
    Ok( ReplyWithCollection::new( base, dnas ) )
}




#[derive(Debug, Deserialize)]
pub struct UpdateDnaInput {
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
fn update_dna(input: UpdateDnaInput) -> ExternResult<ReplyWithSingle<DnaInfo>> {
    debug!("Updating DNA: {}", input.addr );
    let entity = utils::fetch_entity(input.addr.clone())?;
    let current_dna = DnaEntry::try_from( &entity.content )?;

    let dna = DnaEntry {
	name: match input.properties.name {
	    None => current_dna.name,
	    Some(v) => v,
	},
	description: match input.properties.description {
	    None => current_dna.description,
	    Some(v) => v,
	},
	icon: match input.properties.icon {
	    None => current_dna.icon,
	    x => x,
	},
	published_at: match input.properties.published_at {
	    None => current_dna.published_at,
	    Some(v) => v,
	},
	last_updated: match input.properties.last_updated {
	    None => {
		sys_time()?.as_millis() as u64
	    },
	    Some(v) => v,
	},
	collaborators: match input.properties.collaborators {
	    None => current_dna.collaborators,
	    x => x,
	},
	developer: current_dna.developer,
	deprecation: current_dna.deprecation,
    };

    update_entry(entity.header.clone(), &dna)?;
    let entry_hash = hash_entry(&dna)?;

    debug!("Linking original ({}) to DNA: {}", input.addr, entry_hash );
    create_link(
	input.addr.clone(),
	entry_hash.clone(),
	LinkTag::new(utils::TAG_UPDATE)
    )?;

    Ok(ReplyWithSingle::new(
	entity.replace_content( dna.to_info() )
    ))
}




#[derive(Debug, Deserialize)]
pub struct DeprecateDnaInput {
    pub addr: EntryHash,
    pub message: String,
}

#[hdk_extern]
fn deprecate_dna(input: DeprecateDnaInput) -> ExternResult<ReplyWithSingle<DnaInfo>> {
    debug!("Deprecating DNA: {}", input.addr );
    let entity = utils::fetch_entity(input.addr.clone())?;
    let current_dna = DnaEntry::try_from( &entity.content )?;

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

    update_entry(entity.header.clone(), &dna)?;
    let entry_hash = hash_entry(&dna)?;

    debug!("Linking original ({}) to DNA: {}", input.addr, entry_hash );
    create_link(
	input.addr.clone(),
	entry_hash.clone(),
	LinkTag::new(utils::TAG_UPDATE)
    )?;

    Ok( ReplyWithSingle::new(
	entity.replace_content( dna.to_info() )
    ))
}
