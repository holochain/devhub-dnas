
import { AgentPubKey, HoloHash,
	 ActionHash, EntryHash }	from '@spartan-hc/holo-hash';
import {
    intoStruct,
    OptionType, VecType, MapType,
}					from '@whi/into-struct';


export const DnaStruct = {
    "manifest":			Object,
    "resources":		Object,
};

export function DnaEntry ( entry ) {
    return intoStruct( entry, DnaStruct );
}


export default {
    DnaStruct,
    DnaEntry,
};
