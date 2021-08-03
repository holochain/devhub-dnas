use hdk::prelude::*;
use crate::errors::{ ErrorKinds };

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
/// An Entry that represents a full byte-set by grouping a set of MemoryBlockEntry
///
/// Example values
/// ```
/// MemoryEntry {
///     author: AgentPubKey::try_from("uhCAkNBaVvGRYmJUqsGNrfO8jC9Ij-t77QcmnAk3E3B8qh6TU09QN").unwrap(),
///     published_at: 1628013738224,
///     memory_size: 712837,
///     block_addresses: vec![
///         EntryHash::try_from()
///     ]
/// }
/// ```
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
/// Indicates where something fits in a byte-set
///
/// Example (indicating block 1 of a 2 block set)
/// ```
/// SequencePosition {
///     pub position: 1, // Indexing is intended to start at 1, not 0
///     pub length: 2,
/// }
/// ```
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SequencePosition {
    pub position: u64,
    pub length: u64,
}

/// An Entry that contains 1 part of a MemoryEntry byte-set
#[hdk_entry(id = "memory_block", visibility="public")]
#[derive(Clone)]
pub struct MemoryBlockEntry {
    pub sequence: SequencePosition,
    pub bytes: SerializedBytes,
}
try_from_element![ MemoryBlockEntry ];
