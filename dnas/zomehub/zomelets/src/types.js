
import { Bytes }			from '@whi/bytes-class';
import {
    AgentPubKey, HoloHash,
    ActionHash, EntryHash
}					from '@spartan-hc/holo-hash';
import { MemoryStruct }			from '@spartan-hc/mere-memory-zomelets';
import {
    ScopedEntity,
    intoStruct,
    AnyType, OptionType,
    VecType, MapType,
}					from '@spartan-hc/entities';


//
// ZomeEntry Handling
//
export const ZomeStruct = {
    "zome_type":		String,
    "mere_memory_addr":		EntryHash,
    "file_size":		Number,
    "hash":			String,
};

export function ZomeEntry ( entry ) {
    return intoStruct( entry, ZomeStruct );
}

export class Zome extends ScopedEntity {
    static STRUCT		= ZomeStruct;
}


//
// ZomeAsset Handling
//
export const ZomeAssetStruct = {
    "zome_entry":		ZomeStruct,
    "memory_entry":		MemoryStruct,
    "bytes":			Bytes,
};

export function ZomeAsset ( entry ) {
    return intoStruct( entry, ZomeAssetStruct );
}


//
// ZomePackageEntry Handling
//
export const MaintainerType	= String;

export const ZomePackageStruct = {
    "name":			String,
    "description":		String,
    "zome_type":		String,
    "maintainer": {
	"type":			MaintainerType,
	"content":		AgentPubKey,
    },
    "tags":			OptionType( VecType( String ) ),
    "metadata":			Object,
};

export function ZomePackageEntry ( entry ) {
    return intoStruct( entry, ZomePackageStruct );
}

export class ZomePackage extends ScopedEntity {
    static STRUCT		= ZomePackageStruct;
}


//
// ZomePackageVersionEntry Handling
//
export const ZomePackageVersionStruct = {
    "for_package":		ActionHash,
    "zome_entry":		EntryHash,
};

export function ZomePackageVersionEntry ( entry ) {
    return intoStruct( entry, ZomePackageVersionStruct );
}

export class ZomePackageVersion extends ScopedEntity {
    static STRUCT		= ZomePackageVersionStruct;
}


export default {
    ZomeStruct,
    ZomeEntry,
    Zome,

    ZomePackageStruct,
    ZomePackageEntry,
    ZomePackage,

    ZomePackageVersionStruct,
    ZomePackageVersionEntry,
    ZomePackageVersion,

    ZomeAssetStruct,
    ZomeAsset,
};
