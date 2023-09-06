
import { AgentPubKey, HoloHash,
	 ActionHash, EntryHash }	from '@spartan-hc/holo-hash';
import {
    intoStruct,
    OptionType, VecType, MapType,
}					from '@whi/into-struct';


export const WasmStruct = {
    "wasm_type":		String,
    "mere_memory_addr":		EntryHash,
    "file_size":		Number,
};

export function WasmEntry ( entry ) {
    return intoStruct( entry, WasmStruct );
}


export default {
    WasmStruct,
    WasmEntry,
};
