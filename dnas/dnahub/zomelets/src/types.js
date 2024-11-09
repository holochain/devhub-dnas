
import { Bytes }			from '@whi/bytes-class';
import {
    HoloHash,
    AnyLinkableHash, AnyDhtHash,
    AgentPubKey, DnaHash,
    ActionHash, EntryHash
}					from '@spartan-hc/holo-hash';
import {
    ScopedEntity,
    intoStruct,
    AnyType, OptionType,
    VecType, MapType,
}					from '@spartan-hc/entities';
import {
    ZomeAssetStruct,
}					from '@holochain/zomehub-zomelets';



//
// Common Structs
//
export const HRLStruct = {
    "dna":			DnaHash,
    "target":			AnyDhtHash,
}



export const DnaTokenStruct = {
    "integrity_hash":		Bytes,
    "integrities_token_hash":	Bytes,
    "coordinators_token_hash":	Bytes,
};

export const DnaStruct = {
    "manifest":                 AnyType,
    "resources":		MapType( String, HRLStruct ),
    "dna_token":		DnaTokenStruct,
    "integrities_token":	VecType([
	String, Bytes,
    ]),
    "coordinators_token":	VecType([
	String, Bytes,
    ]),
    "claimed_file_size":	Number,
    "asset_hashes": {
	"integrity":		MapType( String, String ),
	"coordinator":		MapType( String, String ),
    },
};

export function DnaEntry ( entry ) {
    return intoStruct( entry, DnaStruct );
}

export class Dna extends ScopedEntity {
    static STRUCT		= DnaStruct;
}


export const DnaAssetStruct = {
    "dna_entry":		DnaStruct,
    "zome_assets":		MapType( String, ZomeAssetStruct ),
};

export function DnaAsset ( entry ) {
    return intoStruct( entry, DnaAssetStruct );
}


export default {
    DnaTokenStruct,
    DnaStruct,
    DnaEntry,
    Dna,

    DnaAssetStruct,
    DnaAsset,
};
