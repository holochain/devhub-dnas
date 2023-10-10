
import { AgentPubKey, HoloHash,
	 ActionHash, EntryHash }	from '@spartan-hc/holo-hash';
import {
    ScopedEntity,
    intoStruct,
    OptionType, VecType, MapType,
}					from '@spartan-hc/caps-entities';



//
// AppEntry Handling
//
export const AppStruct = {
    "manifest":			Object,
    "resources":		Object,
};

export function AppEntry ( entry ) {
    return intoStruct( entry, AppStruct );
}


//
// WebAppEntry Handling
//
export const WebAppStruct = {
    "manifest":			Object,
    "resources":		Object,
};

export function WebAppEntry ( entry ) {
    return intoStruct( entry, WebAppStruct );
}


//
// WebAppPackageEntry Handling
//
export const WebAppPackageStruct = {
    "title":			String,
    "subtitle":			String,
    "description":		String,
    "icon":			EntryHash,
    "source_code_url":		OptionType( String ),
    "maintainer":		{
	"group":		OptionType([ ActionHash, ActionHash ]),
	"agent":		OptionType( AgentPubKey ),
    },
    "deprecation":		OptionType( Object ),
    "metadata":			Object,
};

export function WebAppPackageEntry ( entry ) {
    return intoStruct( entry, WebAppPackageStruct );
}

export class WebAppPackage extends ScopedEntity {
    static STRUCT		= WebAppPackageStruct;

    async versions () {
	return await this.zome.get_webapp_package_versions( this.$id );
    }
}


//
// WebAppPackageVersionEntry Handling
//
export const WebAppPackageVersionStruct = {
    "for_package":		ActionHash,
    "webapp":			ActionHash,
    "source_code_url":		OptionType( String ),
    "maintainer":		{
	"group":		OptionType([ ActionHash, ActionHash ]),
	"agent":		OptionType( AgentPubKey ),
    },
};

export function WebAppPackageVersionEntry ( entry ) {
    return intoStruct( entry, WebAppPackageVersionStruct );
}

export class WebAppPackageVersion extends ScopedEntity {
    static STRUCT		= WebAppPackageVersionStruct;
}



export default {
    AppStruct,
    AppEntry,

    WebAppStruct,
    WebAppEntry,

    WebAppPackageStruct,
    WebAppPackageEntry,
    WebAppPackage,

    WebAppPackageVersionStruct,
    WebAppPackageVersionEntry,
    WebAppPackageVersion,
};
