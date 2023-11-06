
import { Bytes }			from '@whi/bytes-class';
import {
    AgentPubKey, HoloHash,
    ActionHash, EntryHash
}					from '@spartan-hc/holo-hash';
import {
    ScopedEntity,
    intoStruct,
    AnyType, OptionType,
    VecType, MapType,
}					from '@spartan-hc/caps-entities';


export const DnaTokenStruct = {
    "integrity_hash":		Bytes,
    "integrities_token_hash":	Bytes,
    "coordinators_token_hash":	Bytes,
};

export const DnaStruct = {
    "manifest": {
	"name":			String,
	"integrity": {
	    "network_seed":	OptionType( AnyType ),
	    "properties":	OptionType( AnyType ),
	    "origin_time":	AnyType,
	    "zomes": VecType({
		"name":		String,
		"hash":		OptionType( AnyType ),
		"wasm_entry":	EntryHash,
		"dylib":	OptionType( AnyType ),
	    }),
	},
	"coordinator": {
	    "zomes": VecType({
		"name":		String,
		"hash":		OptionType( AnyType ),
		"wasm_entry":	EntryHash,
		"dependencies": VecType({
		    "name":	String,
		}),
		"dylib":	OptionType( AnyType ),
	    }),
	},
    },
    "dna_token":		DnaTokenStruct,
    "integrities_token":	VecType([
	String, Bytes,
    ]),
    "coordinators_token":	VecType([
	String, Bytes,
    ]),
};

export function DnaEntry ( entry ) {
    return intoStruct( entry, DnaStruct );
}

export class Dna extends ScopedEntity {
    static STRUCT		= DnaStruct;
}


export default {
    DnaTokenStruct,
    DnaStruct,
    DnaEntry,
    Dna,
};
