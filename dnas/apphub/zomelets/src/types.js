
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
// UiEntry Handling
//
export const UiStruct = {
    "mere_memory_addr":		EntryHash,
    "file_size":		Number,
};

export function UiEntry ( entry ) {
    return intoStruct( entry, UiStruct );
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
    "maintainer": {
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
	return await this.zome.get_webapp_package_versions_sorted( this.$id );
    }
}


//
// WebAppPackageVersionEntry Handling
//
export const WebAppPackageVersionStruct = {
    // The version value comes from the link tag (not the entry) so it will only be present when
    // fetched in the context of a 'get_links'
    "version":			OptionType( String ),

    "for_package":		ActionHash,
    "webapp":			EntryHash,
    "source_code_url":		OptionType( String ),
    "maintainer": {
	"group":		OptionType([ ActionHash, ActionHash ]),
	"agent":		OptionType( AgentPubKey ),
    },
};

export function WebAppPackageVersionEntry ( entry ) {
    return intoStruct( entry, WebAppPackageVersionStruct );
}

export class WebAppPackageVersion extends ScopedEntity {
    static STRUCT		= WebAppPackageVersionStruct;

    async getWebAppPackage () {
	return await this.zome.get_webapp_package_entry( this.for_package );
    }
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
