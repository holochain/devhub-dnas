mod types;

use hdk::prelude::*;

pub use types::{ UpdateEntityInput, GetEntityInput };


#[derive(Debug, Serialize, Deserialize)]
pub struct EntityType {
    pub name: String,
    pub model: String,
}
pub trait EntryModel {
    fn get_type(&self) -> EntityType;
}

impl EntityType {
    pub fn new(name: &'static str, model: &'static str) -> Self {
	EntityType {
	    name: name.into(),
	    model: model.into(),
	}
    }
}


#[derive(Debug, Serialize)]
pub struct Metadata {
    pub composition: &'static str,
}



#[derive(Debug, Serialize, Deserialize)]
pub struct Entity<T> {
    pub id: EntryHash,
    pub header: HeaderHash,
    pub address: EntryHash,
    #[serde(rename = "type")]
    pub ctype: EntityType,
    pub content: T,
}

impl<T> Entity<T> {
    pub fn new_content<M>(&self, content: M) -> Entity<M>
    where M: EntryModel {
	Entity {
	    id: self.id.to_owned(),
	    header: self.header.to_owned(),
	    address: self.address.to_owned(),
	    ctype: content.get_type(),
	    content: content,
	}
    }

    pub fn update_header(mut self, hash: HeaderHash) -> Self {
	self.header = hash;

	self
    }

    pub fn update_address(mut self, hash: EntryHash) -> Self {
	self.address = hash;

	self
    }
}



#[derive(Debug, Serialize)]
pub struct Collection<T> {
    pub base: EntryHash,
    pub items: Vec<T>,
}



#[cfg(test)]
pub mod tests {
    use super::*;

    use rand::Rng;

    #[test]
    ///
    fn entity_test() {
	let bytes = rand::thread_rng().gen::<[u8; 32]>();
	let ehash = EntryHash::from_raw_32( bytes.to_vec() );
	let hhash = HeaderHash::from_raw_32( bytes.to_vec() );

	let item = Entity {
	    id: ehash.clone(),
	    header: hhash,
	    address: ehash,
	    ctype: EntityType::new( "boolean", "primitive" ),
	    content: true,
	};

	assert_eq!( item.ctype.name, "boolean" );
	assert_eq!( item.ctype.model, "primitive" );
    }
}
