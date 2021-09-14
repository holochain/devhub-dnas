
use hdk::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetEntityInput {
    pub id: EntryHash,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateEntityInput<T> {
    pub id: Option<EntryHash>,
    pub addr: EntryHash,
    pub properties: T,
}
