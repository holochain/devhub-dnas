use std::collections::{ BTreeMap, BTreeSet };
use std::iter::FromIterator;

use hdk::prelude::*;
use hdk::hash_path::path::{ Component };

pub use hc_crud::{
    get_entity,
    Entity, EntryModel,
    UtilsError,
};

use crate::{
    AppResult, UserError, AppError,
    fmt_path, fmt_tag,
    constants::{
	ANCHOR_TAGS,
	ANCHOR_FILTERS,
	ANCHOR_HDK_VERSIONS,
    },
};


pub fn link_target_to_action( link: &Link, error: String ) -> AppResult<ActionHash> {
    Ok( link.target.to_owned().into_action_hash()
	.ok_or(AppError::UnexpectedStateError(error))? )
}


pub fn create_path<T>( base: &str, segments: T ) -> (Path, EntryHash)
where
    T: IntoIterator,
    T::Item: std::fmt::Display,
{
    let mut components : Vec<Component> = vec![];

    for seg in base.split(".") {
	let component = Component::from( format!("{}", seg ).as_bytes().to_vec() );
	components.push( component );
    }

    for seg in segments {
	let component = Component::from( format!("{}", seg ).as_bytes().to_vec() );
	components.push( component );
    }

    let path = Path::from( components );
    let hash = path.path_entry_hash().unwrap();

    ( path, hash )
}

pub fn ensure_path<T,LT,E>( base: &str, segments: T, link_type: LT ) -> AppResult<(Path, EntryHash)>
where
    T: IntoIterator,
    T::Item: std::fmt::Display,
    ScopedLinkType: TryFrom<LT, Error = E>,
    WasmError: From<E>, 
{
    let result = create_path( base, segments );
    result.0.to_owned().typed( link_type )?.ensure()?;

    Ok( result )
}


pub fn get_entities_for_path<T,LT,ET>(path: Path, link_type: LT, tag: Option<Vec<u8>> ) -> AppResult<Vec<Entity<T>>>
where
    T: TryFrom<Record, Error = WasmError> + Clone + EntryModel<ET>,
    LT: LinkTypeFilterExt + std::fmt::Debug + Clone,
    Entry: TryFrom<T, Error = WasmError>,
    ScopedEntryDefIndex: for<'a> TryFrom<&'a ET, Error = WasmError>,
{
    debug!("Getting all {:?} [{}] entities: {}", link_type, fmt_tag( &tag ), fmt_path( &path ) );

    let path_hash = path.path_entry_hash().unwrap();
    let links = get_links(
        path_hash.clone(),
	link_type,
	tag.map( |tag| LinkTag::new( tag ) ),
    )?;

    let list = links.into_iter()
	.filter_map(|link| {
	    link.target.into_action_hash()
		.and_then( |target| get_entity( &target ).ok() )
	})
	.collect();

    Ok(list)
}


pub fn get_entities_for_path_filtered<T,F,LT,ET>(path: Path, link_type: LT, tag: Option<Vec<u8>>, filter: F ) -> AppResult<Vec<Entity<T>>>
where
    T: TryFrom<Record, Error = WasmError> + Clone + EntryModel<ET>,
    F: FnOnce(Vec<Entity<T>>) -> AppResult<Vec<Entity<T>>>,
    LT: LinkTypeFilterExt + std::fmt::Debug + Clone,
    Entry: TryFrom<T, Error = WasmError>,
    ScopedEntryDefIndex: for<'a> TryFrom<&'a ET, Error = WasmError>,
{
    let collection = get_entities_for_path( path, link_type, tag )?;

    Ok(filter( collection )?)
}


pub fn get_hdk_version_entities<T,LT,ET>( link_type: LT, hdk_version: String ) -> AppResult<Vec<Entity<T>>>
where
    T: TryFrom<Record, Error = WasmError> + Clone + EntryModel<ET>,
    LT: LinkTypeFilterExt + std::fmt::Debug + Clone,
    Entry: TryFrom<T, Error = WasmError>,
    ScopedEntryDefIndex: for<'a> TryFrom<&'a ET, Error = WasmError>,
{
    let (base_path, base_hash) = create_path( ANCHOR_HDK_VERSIONS, vec![ &hdk_version ] );

    debug!("Getting entities with tag '{:?}' and HDK Version '{}': {}", link_type, hdk_version, fmt_path( &base_path ) );
    let links = get_links(
        base_hash.clone(),
	link_type,
	None
    )?;

    let list = links.into_iter()
	.filter_map(|link| {
	    link.target.into_action_hash()
		.and_then( |target| get_entity::<T,ET>( &target ).ok() )
	})
	.collect();

    Ok(list)
}


pub fn get_by_filter<T,LT,ET>( link_type: LT, filter: String, keyword: String ) -> AppResult<Vec<Entity<T>>>
where
    T: TryFrom<Record, Error = WasmError> + Clone + EntryModel<ET> + std::fmt::Debug,
    LT: LinkTypeFilterExt + std::fmt::Debug + Clone,
    Entry: TryFrom<T, Error = WasmError>,
    ScopedEntryDefIndex: for<'a> TryFrom<&'a ET, Error = WasmError>,
{
    let (base_path, base_hash) = create_path( ANCHOR_FILTERS, vec![ &filter, &keyword ] );

    debug!("Getting '{:?}' links for filter: {} => {:?}", link_type, fmt_path( &base_path ), base_path );
    let links = get_links(
        base_hash.clone(),
	link_type,
	None
    )?;

    let list = links.into_iter()
	.filter_map(|link| {
	    link.target.into_action_hash()
		.and_then( |target| {
		    let result = get_entity::<T,ET>( &target );
		    debug!("get_entity::<{},ET>( {} ) -> {:?}", T::name(), target, result );
		    result.ok()
		})
	})
	.collect();

    Ok(list)
}


pub fn get_by_tags<T,LT,ET>( link_type: LT, tags: Vec<String> ) -> AppResult<Vec<Entity<T>>>
where
    T: TryFrom<Record, Error = WasmError> + Clone + EntryModel<ET>,
    LT: LinkTypeFilterExt + std::fmt::Debug + Clone,
    Entry: TryFrom<T, Error = WasmError>,
    ScopedEntryDefIndex: for<'a> TryFrom<&'a ET, Error = WasmError>,
{
    if tags.len() == 0 {
	return Err( UserError::CustomError("Tag list cannot be empty").into() );
    }

    let tag_count = tags.len();
    let mut match_count = BTreeMap::new();

    debug!("Gathering links for tags: {:?}", tags );
    for tag_name in tags.into_iter() {
	let (base, base_hash) = create_path( ANCHOR_TAGS, vec![ &tag_name.to_lowercase() ] );

	debug!("Getting '{:?}' links for tag '{}': {} => {:?}", link_type, tag_name, fmt_path( &base ), base );
	let links = get_links(
            base_hash.clone(),
	    link_type.to_owned(),
	    None
	)?;

	for link in links {
	    if let Some((count, _)) = match_count.get_mut( &link.target ) {
		*count += 1;
	    } else {
		match_count.insert( link.target.to_owned(), (1, link) );
	    }
	}
    }

    let mut full_matches = Vec::new();
    for (count, link) in match_count.values() {
	if *count == tag_count {
	    full_matches.push( link.to_owned() );
	}
    }

    let list = full_matches.into_iter()
	.filter_map(|link| {
	    link.target.into_action_hash()
		.and_then( |target| get_entity::<T,ET>( &target ).ok() )
	})
	.collect();

    Ok( list )
}


pub fn get_hdk_versions<LT,E>( link_type: LT ) -> AppResult<Vec<String>>
where
    ScopedLinkType: TryFrom<LT, Error = E>,
    WasmError: From<E>, 
{
    let (hdkv_path, _) = create_path( ANCHOR_HDK_VERSIONS, Vec::<String>::new() );

    let hdk_versions : Vec<String> = hdkv_path.typed( link_type )?.children_paths()?.into_iter()
	.filter_map( |path| {
	    debug!("HDK Version PATH: {}", fmt_path( &path ) );
	    match std::str::from_utf8( path.as_ref().clone().last().unwrap().as_ref() ) {
		Err(_) => None,
		Ok(path_str) => Some( path_str.to_string() ),
	    }
	})
	.collect();

    Ok(hdk_versions)
}


pub fn update_tag_links<T,LT,E,ET>( prev_tags: Option<Vec<String>>, new_tags: Option<Vec<String>>, entity: &Entity<T>, link_type: LT, tag_link_type: LT, ) -> AppResult<()>
where
    T: TryFrom<Record, Error = WasmError> + Clone + EntryModel<ET>,
    LT: LinkTypeFilterExt + std::fmt::Debug + Clone,
    Entry: TryFrom<T, Error = WasmError>,
    ScopedEntryDefIndex: for<'a> TryFrom<&'a ET, Error = WasmError>,
    ScopedLinkType: TryFrom<LT, Error = E>,
    WasmError: From<E>,
{
    debug!("Update tag list for {} [{:?}] from {:?} to {:?}", entity.id, link_type, prev_tags, new_tags );
    if new_tags.is_none() {
	return Ok(());
    }
    // current.tags vs given tags
    //
    //   - create a list of removed tags
    //   - create a list of added tags
    //
    let prev_tags : BTreeSet<String> = BTreeSet::from_iter( prev_tags.unwrap_or( vec![] ).iter().cloned() );
    let new_tags : BTreeSet<String> = BTreeSet::from_iter( new_tags.unwrap_or( vec![] ).iter().cloned() );

    for rm_tag in prev_tags.difference( &new_tags ) {
	let (tag_path, tag_hash) = ensure_path( ANCHOR_TAGS, vec![ &rm_tag.to_lowercase() ], tag_link_type.to_owned() )?;

	let links = get_links(
	    tag_hash.clone(),
	    link_type.to_owned(),
	    None
	)?;

	debug!("Removing tag link: {}", fmt_path( &tag_path ) );
	if let Some(link) = links.iter().find(|link| {
	    debug!("Finding tag link match: {:?} == {:?}", link.target, entity.id );
	    link.target == entity.id.to_owned().into()
	}) {
	    delete_link( link.create_link_hash.clone() )?;
	}
	else {
	    debug!("Expected to remove tag link '{}' but it wasn't found", fmt_path( &tag_path ) );
	}
    }

    for add_tag in new_tags.difference( &prev_tags ) {
	let (tag_path, tag_hash) = ensure_path( ANCHOR_TAGS, vec![ &add_tag.to_lowercase() ], tag_link_type.to_owned() )?;
	debug!("Adding tag link: {}", fmt_path( &tag_path ) );
	entity.link_from( &tag_hash, link_type.to_owned(), None )?;
    }

    Ok(())
}


pub fn trace_action_origin_entry(action_hash: &ActionHash, depth: Option<u64>) -> ExternResult<(EntryHash,u64)> {
    let sh_action = must_get_action( action_hash.to_owned().into() )?;
    let depth : u64 = depth.unwrap_or(0);

    match sh_action.action() {
	Action::Create(create) => Ok( (create.entry_hash.to_owned(), depth) ),
	Action::Update(update) => trace_action_origin_entry( &update.original_action_address, Some(depth+1) ),
	action => Err(wasm_error!(WasmErrorInner::Guest(format!("Unexpected action type @ depth {}: {:?}", depth, action )))),
    }
}

pub fn trace_action_history_with_chain(action_hash: &ActionHash, history: Option<Vec<(ActionHash,EntryHash)>>) -> ExternResult<Vec<(ActionHash,EntryHash)>> {
    let sh_action = must_get_action( action_hash.to_owned().into() )?;
    let mut history = history.unwrap_or( Vec::new() );

    match sh_action.action() {
	Action::Create(create) => {
	    history.push( (action_hash.to_owned(), create.entry_hash.to_owned()) );

	    Ok( history )
	},
	Action::Update(update) => {
	    history.push( (action_hash.to_owned(), update.entry_hash.to_owned()) );

	    trace_action_history_with_chain( &update.original_action_address, Some(history) )
	},
	action => Err(wasm_error!(WasmErrorInner::Guest(format!("Unexpected action type @ trace depth {}: {:?}", history.len(), action )))),
    }
}

pub fn trace_action_history(action_hash: &ActionHash) -> ExternResult<Vec<(ActionHash,EntryHash)>> {
    trace_action_history_with_chain(action_hash, None)
}
