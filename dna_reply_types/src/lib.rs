use hdk::prelude::*;

pub trait EntryModel {
    fn get_type(&self) -> String;
}

#[derive(Debug, Serialize)]
pub struct EssencePackage<T, M> {
    #[serde(rename = "type")]
    rtype: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<M>,
    payload: T,
}


#[derive(Debug, Serialize)]
pub struct EntityMetadata {
    composition: String,
}
type Package<T> = EssencePackage<T, EntityMetadata>;


#[derive(Debug, Serialize)]
pub struct Entity<T> {
    pub id: EntryHash,
    pub header: HeaderHash,
    pub address: EntryHash,
    #[serde(rename = "type")]
    pub ctype: String,
    pub content: T,
}

impl<T> Entity<T> {
    pub fn replace_content<M>(&self, content: M) -> Entity<M>
    where M: EntryModel {
	Entity {
	    id: self.id.to_owned(),
	    header: self.header.to_owned(),
	    address: self.address.to_owned(),
	    ctype: content.get_type(),
	    content: content,
	}
    }
}

#[derive(Debug, Serialize)]
pub struct EntityCollection<T> {
    pub base: EntryHash,
    pub items: Vec<Entity<T>>,
}



pub type ReplyWithSingle<T> = Package<Entity<T>>;

impl<T> ReplyWithSingle<T> where T: EntryModel {
    pub fn new (entity: Entity<T>) -> Self {
	EssencePackage {
	    rtype: String::from("success"),
	    metadata: Some(EntityMetadata {
		composition: String::from("single"),
	    }),
	    payload: entity,
	}
    }
}

pub type ReplyWithCollection<T> = Package<EntityCollection<T>>;

impl<T> ReplyWithCollection<T> {
    pub fn new (base: EntryHash, entities: Vec<Entity<T>>) -> Self {
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
