
import { AgentPubKey, HoloHash,
	 ActionHash, EntryHash }	from '@spartan-hc/holo-hash';
import {
    intoStruct,
    OptionType, VecType, MapType,
}					from '@whi/into-struct';


export const WasmStruct = {
    "author":			AgentPubKey,
    "mere_memory_addr":		EntryHash,

    // Common fields
    "published_at":		Number, // Date,
    "last_updated":		Number, // Date,
    "metadata":			Object,
};

export function WasmEntry ( entry ) {
    return intoStruct( entry, WasmStruct );
}


export default {
    WasmStruct,
    WasmEntry,
};
