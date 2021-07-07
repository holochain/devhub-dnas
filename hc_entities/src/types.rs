
use hdk::prelude::*;

#[derive(Debug, Deserialize)]
pub struct GetEntityInput {
    pub id: EntryHash,
}

#[derive(Debug, Deserialize)]
pub struct UpdateEntityInput<T> {
    pub id: Option<EntryHash>,
    pub addr: EntryHash,
    pub properties: T,
}
