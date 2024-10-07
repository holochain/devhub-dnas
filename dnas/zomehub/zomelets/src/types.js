
import { Bytes }			from '@whi/bytes-class';
import {
    HoloHash,
    AnyLinkableHash, AnyDhtHash,
    AgentPubKey, DnaHash,
    ActionHash, EntryHash
}					from '@spartan-hc/holo-hash';
import { MemoryStruct }			from '@spartan-hc/mere-memory-zomelets';
import {
    ScopedEntity,
    intoStruct,
    AnyType, OptionType,
    VecType, MapType,
}					from '@spartan-hc/entities';


//
// ZomeEntry Handling
//
export const ZomeStruct = {
    "zome_type":		String,
    "mere_memory_addr":		EntryHash,
    "file_size":		Number,
    "hash":			String,
};

export function ZomeEntry ( entry ) {
    return intoStruct( entry, ZomeStruct );
}

export class Zome extends ScopedEntity {
    static STRUCT		= ZomeStruct;
}


//
// ZomeAsset Handling
//
export const ZomeAssetStruct = {
    "zome_entry":		ZomeStruct,
    "memory_entry":		MemoryStruct,
    "bytes":			Bytes,
};

export function ZomeAsset ( entry ) {
    return intoStruct( entry, ZomeAssetStruct );
}


export function Authority ( data ) {
    if ( data.type === "agent" )
        data.content            = new AgentPubKey( data.content );
    else if ( data.type === "group" )
        data.content            = intoStruct( data.content, [ ActionHash, ActionHash ] );

    return data;
}

//
// ZomePackageEntry Handling
//
export const ZomePackageStruct = {
    "name":			String,
    "title":			String,
    "description":		String,
    "zome_type":		String,
    "maintainer":               Authority,
    "tags":			OptionType( VecType( String ) ),
    "metadata":			Object,
};

export function ZomePackageEntry ( entry ) {
    return intoStruct( entry, ZomePackageStruct );
}

export class ZomePackage extends ScopedEntity {
    static STRUCT		= ZomePackageStruct;

    async $versions () {
	return await this.zome.get_zome_package_versions_sorted( this.$id );
    }
}



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
// ZomePackageVersionEntry Handling
//
export const ZomePackageVersionStruct = {
    // The version value comes from the link tag (not the entry) so it will only be present when
    // fetched in the context of a 'get_links'
    "version":			OptionType( String ),

    "for_package":		ActionHash,
    "zome_entry":		EntryHash,
    "maintainer":               Authority,
    "changelog":		OptionType( String ),
    "source_code_revision_uri":	OptionType( String ),
    "api_compatibility": {
        "build_with": {
            "hdi_version":      String,
            "hdk_version":      OptionType( String ),
        },
        "tested_with":          String,
    },
    "metadata":			Object,
};

export function ZomePackageVersionEntry ( entry ) {
    return intoStruct( entry, ZomePackageVersionStruct );
}

export class ZomePackageVersion extends ScopedEntity {
    static STRUCT		= ZomePackageVersionStruct;

    async $getZomePackage () {
	return await this.zome.get_zome_package( this.for_package );
    }
}


export default {
    ZomeStruct,
    ZomeEntry,
    Zome,

    ZomePackageStruct,
    ZomePackageEntry,
    ZomePackage,

    ZomePackageVersionStruct,
    ZomePackageVersionEntry,
    ZomePackageVersion,

    ZomeAssetStruct,
    ZomeAsset,
};
