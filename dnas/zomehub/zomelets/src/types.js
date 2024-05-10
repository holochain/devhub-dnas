
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


export const ZomeAssetStruct = {
    "zome_entry":		ZomeStruct,
    "memory_entry":		MemoryStruct,
    "bytes":			Bytes,
};

export function ZomeAsset ( entry ) {
    return intoStruct( entry, ZomeAssetStruct );
}


export default {
    ZomeStruct,
    ZomeEntry,
    Zome,

    ZomeAssetStruct,
    ZomeAsset,
};
