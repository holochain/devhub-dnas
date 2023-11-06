
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


export const WasmStruct = {
    "wasm_type":		String,
    "mere_memory_addr":		EntryHash,
    "file_size":		Number,
};

export function WasmEntry ( entry ) {
    return intoStruct( entry, WasmStruct );
}

export class Wasm extends ScopedEntity {
    static STRUCT		= WasmStruct;
}


export default {
    WasmStruct,
    WasmEntry,
    Wasm,
};
