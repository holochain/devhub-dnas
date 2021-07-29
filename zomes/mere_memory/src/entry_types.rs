use hdk::prelude::*;
use crate::errors::{ ErrorKinds };

#[macro_export]
macro_rules! try_from_element {
    ( $( $struct:ident ),* ) => {
	$(
	    impl TryFrom<&Element> for $struct {
		type Error = ErrorKinds;

		fn try_from(element: &Element) -> Result<Self, Self::Error> {
		    let entry = element.entry()
			.to_app_option::<Self>().map_err( |e| ErrorKinds::HDKError(e.into()) )?
			.ok_or( ErrorKinds::DeserializationError(element.clone()) )?;

		    let entry_hash = hash_entry(&entry)
			.map_err(ErrorKinds::HDKError)?;
		    let expected_hash = element.header().entry_hash().unwrap().clone();

		    if entry_hash == expected_hash {
			Ok( entry )
		    } else {
			Err( ErrorKinds::DeserializationWrongEntryTypeError(entry_hash, expected_hash) )
		    }
		}
	    }
	)*
    };
}

//
// Memory Entry
//
#[hdk_entry(id = "memory_details", visibility="public")]
#[derive(Clone)]
pub struct MemoryEntry {
    pub author: AgentPubKey,
    pub published_at: u64,
    pub memory_size: u64,
    pub block_addresses: Vec<EntryHash>,
}
try_from_element![ MemoryEntry ];


//
// Memory Block Entry
//
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SequencePosition {
    pub position: u64,
    pub length: u64,
}

#[hdk_entry(id = "memory_block", visibility="public")]
#[derive(Clone)]
pub struct MemoryBlockEntry {
    pub sequence: SequencePosition,
    pub bytes: SerializedBytes,
}
try_from_element![ MemoryBlockEntry ];
