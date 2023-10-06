
import { AgentPubKey, HoloHash,
	 ActionHash, EntryHash }	from '@spartan-hc/holo-hash';
import {
    intoStruct,
    OptionType, VecType, MapType,
}					from '@whi/into-struct';


export const AppStruct = {
    "manifest":			Object,
    "resources":		Object,
};

export function AppEntry ( entry ) {
    return intoStruct( entry, AppStruct );
}


export default {
    AppStruct,
    AppEntry,
};
