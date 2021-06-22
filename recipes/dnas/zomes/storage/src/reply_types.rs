use hdk::prelude::*;

use crate::entry_types::{ EntryModel };

#[derive(Debug, Serialize)]
pub struct EssencePackage<T, M> {
    #[serde(rename = "type")]
    rtype: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<M>,
    payload: T,
}
type Package<T, M> = EssencePackage<T, M>;

#[derive(Debug, Serialize)]
pub struct EntityMetadata {
    composition: String,
}


#[derive(Debug, Serialize)]
pub struct Entity<T> {
    pub id: EntryHash,
    pub address: EntryHash,
    #[serde(rename = "type")]
    pub ctype: String,
    pub content: T,
}
pub type ReplyWithSingle<T> = Package<Entity<T>, EntityMetadata>;

impl<T> ReplyWithSingle<T> where T: EntryModel {
    pub fn success (id: EntryHash, addr: EntryHash, content: T) -> Self {
	EssencePackage {
	    rtype: String::from("success"),
	    metadata: Some(EntityMetadata {
		composition: String::from("single"),
	    }),
	    payload: Entity {
		id: id,
		address: addr,
		ctype: content.get_type(),
		content: content,
	    },
	}
    }
}

#[derive(Debug, Serialize)]
pub struct EntityCollection<T> {
    pub base: EntryHash,
    pub items: Vec<Entity<T>>,
}
pub type ReplyWithCollection<T> = Package<EntityCollection<T>, EntityMetadata>;

impl<T> ReplyWithCollection<T> {
    pub fn success (base: EntryHash, entities: Vec<Entity<T>>) -> Self {
	EssencePackage {
	    rtype: String::from("success"),
	    metadata: Some(EntityMetadata {
		composition: String::from("collection"),
	    }),
	    payload: EntityCollection {
		base: base,
		items: entities,
	    },
	}
    }
}
