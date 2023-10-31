
import { Bytes }			from '@whi/bytes-class';
import { AgentPubKey, HoloHash,
	 ActionHash, EntryHash }	from '@spartan-hc/holo-hash';
import {
    DnaTokenStruct,
}					from '@holochain/dnahub-zomelets';
import {
    ScopedEntity,
    intoStruct,
    AnyType, OptionType,
    VecType, MapType,
}					from '@spartan-hc/caps-entities';


//
// AppEntry Handling
//
export const AppStruct = {
    "manifest": {
	"name":			String,
	"description":		String,
	"roles": VecType({
	    "name":		String,
	    "provisioning":	OptionType( Object ),
	    "dna": {
		"dna_entry":		EntryHash,
		"modifiers": {
		    "network_seed":	OptionType( AnyType ),
		    "properties":	OptionType( AnyType ),
		    "origin_time":	OptionType( AnyType ),
		    "quantum_time":	OptionType( AnyType ),
		},
		"installed_hash":	OptionType( AnyType ),
		"clone_limit":		Number,
	    }
	}),
    },
    "app_token": {
	"integrity_hash":	Bytes,
	"roles_token_hash":	Bytes,
    },
    "roles_token": VecType([
	String,
	Object.assign( {}, DnaTokenStruct, {
	    "modifiers_hash":	Bytes,
	}),
    ]),
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
// WebAppToken Struct
//
export const WebAppToken = {
    "integrity_hash":		Bytes,
    "assets_token_hash":	Bytes,
};


//
// WebAppEntry Handling
//
export const WebAppStruct = {
    "manifest": {
	"name":			String,
	"ui": {
	    "ui_entry":		EntryHash,
	},
	"happ_manifest": {
	    "app_entry":	EntryHash,
	},
    },
    "webapp_token":		WebAppToken,
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
    "webapp_token":		WebAppToken,
    "source_code_revision_url":	OptionType( String ),
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
	return await this.zome.get_webapp_package( this.for_package );
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
