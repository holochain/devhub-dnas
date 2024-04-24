
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


export const ZomeStruct = {
    "zome_type":		String,
    "mere_memory_addr":		EntryHash,
    "file_size":		Number,
};

export function ZomeEntry ( entry ) {
    return intoStruct( entry, ZomeStruct );
}

export class Zome extends ScopedEntity {
    static STRUCT		= ZomeStruct;
}


export default {
    ZomeStruct,
    ZomeEntry,
    Zome,
};
