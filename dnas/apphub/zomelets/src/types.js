
import { Bytes }			from '@whi/bytes-class';
import {
    HoloHash,
    AnyLinkableHash, AnyDhtHash,
    AgentPubKey, DnaHash,
    ActionHash, EntryHash
}					from '@spartan-hc/holo-hash';
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
// Common Structs
//
export const HRLStruct = {
    "dna":			DnaHash,
    "target":			AnyDhtHash,
}

export const LinkStruct = {
    "author":			AgentPubKey,
    "target":			AnyLinkableHash,
    "timestamp":		Number,
    "zome_index":		Number,
    "link_type":		Number,
    "tag":			Uint8Array,
    "create_link_hash":		ActionHash,
}

export class Link {
    constructor ( data ) {
	Object.assign( this, intoStruct( data, LinkStruct ) );
    }

    tagString () {
	return this.tag;
    }

    toJSON () {
	const decoder		= new TextDecoder();
	const data		= Object.assign( {}, this );
	try {
	    data.tag		= decoder.decode( data.tag );
	} catch (_) {
	    // Tag doesn't need to be a string
	}
	return data;
    }
}


//
// AppToken Struct
//
export const AppTokenStruct = {
    "integrity_hash":		Bytes,
    "roles_token_hash":		Bytes,
    "roles_token": VecType([
	String,
	Object.assign( {}, DnaTokenStruct, {
	    "modifiers_hash":	Bytes,
	}),
    ]),
};


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
		"dna_hrl":		HRLStruct,
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
    "app_token":		AppTokenStruct,
    "claimed_file_size":	Number,
};

export function AppEntry ( entry ) {
    return intoStruct( entry, AppStruct );
}

export class App extends ScopedEntity {
    static STRUCT		= AppStruct;
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

export class Ui extends ScopedEntity {
    static STRUCT		= UiStruct;
}


//
// WebAppToken Struct
//
export const WebAppTokenStruct = {
    "ui_hash":			Bytes,
    "app_token":		AppTokenStruct,
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
    "webapp_token":		WebAppTokenStruct,
};

export function WebAppEntry ( entry ) {
    return intoStruct( entry, WebAppStruct );
}

export class WebApp extends ScopedEntity {
    static STRUCT		= WebAppStruct;
}


//
// WebAppPackageEntry Handling
//
export const MaintainerType	= Object;

export const WebAppPackageStruct = {
    "title":			String,
    "subtitle":			String,
    "description":		String,
    "icon":			EntryHash,
    "source_code_uri":		OptionType( String ),
    "maintainer": {
	"type":			MaintainerType,
	"content":		AgentPubKey, // [ ActionHash, ActionHash ]
    },
    "deprecation":		OptionType( Object ),
    "metadata":			Object,
};

export function WebAppPackageEntry ( entry ) {
    // entry.maintainer.type	= Object.keys( entry.maintainer.type )[0];
    return intoStruct( entry, WebAppPackageStruct );
}

export class WebAppPackage extends ScopedEntity {
    static STRUCT		= WebAppPackageStruct;

    async $versions () {
	return await this.zome.get_webapp_package_versions_sorted( this.$id );
    }

    async $update ( changes ) {
	const result		= await this.zome.update_webapp_package({
	    "base": this.$action,
	    "properties": changes,
	});

	super.$update( result );

	return this;
    }

    async $deprecate ( message, recommended_alternatives = [] ) {
	const result		= await this.zome.deprecate_webapp_package({
	    "base": this.$action,
	    "properties": {
		message,
		recommended_alternatives,
	    },
	});

	super.$update( result );

	return this;
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
    "changelog":		OptionType( String ),
    "webapp":			EntryHash,
    "webapp_token":		WebAppTokenStruct,
    "source_code_revision_uri":	OptionType( String ),
    "maintainer": {
	"type":			MaintainerType,
	"content":		AgentPubKey, // [ ActionHash, ActionHash ]
    },
    "metadata":			Object,
};

export function WebAppPackageVersionEntry ( entry ) {
    return intoStruct( entry, WebAppPackageVersionStruct );
}

export class WebAppPackageVersion extends ScopedEntity {
    static STRUCT		= WebAppPackageVersionStruct;

    async $update ( changes ) {
	const result		= await this.zome.update_webapp_package_version({
	    "base": this.$action,
	    "properties": changes,
	});

	super.$update( result );

	return this;
    }

    async $getWebAppPackage () {
	return await this.zome.get_webapp_package( this.for_package );
    }
}



export default {
    LinkStruct,
    Link,

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
