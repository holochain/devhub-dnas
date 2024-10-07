import {
    AnyDhtHash,
    AgentPubKey,
    ActionHash, EntryHash,
}					from '@spartan-hc/holo-hash'; // approx. 11kb
import {
    Zomelet,
    CellZomelets,
}					from '@spartan-hc/zomelets'; // approx. 7kb
import { CoopContentZomelet }		from '@spartan-hc/coop-content-zomelets';
import { MereMemoryZomelet }		from '@spartan-hc/mere-memory-zomelets'; // approx. 33kb
import {
    rsort as semverReverseSort
}					from 'semver'; // approx. 32kb
import {
    Link,
    ZomePackageVersionEntry,

    // Entity Classes
    Zome,
    ZomeAsset,
    ZomePackage,
    ZomePackageVersion,
}					from './types.js';


export const ZOME_TYPES			= {
    "INTEGRITY": "integrity",
    "COORDINATOR": "coordinator",
};
export const ZOME_TYPE_NAMES		= Object.values( ZOME_TYPES );


export const ZomeHubCSRZomelet		= new Zomelet({
    "whoami": {
	output ( response ) {
	    // Struct - https://docs.rs/hdk/*/hdk/prelude/struct.AgentInfo.html
	    return {
		"pubkey": {
		    "initial":		new AgentPubKey( response.agent_initial_pubkey ),
		    "latest":		new AgentPubKey( response.agent_latest_pubkey ),
		},
		"chain_head": {
		    "action":		new ActionHash( response.chain_head[0] ),
		    "sequence":		response.chain_head[1],
		    "timestamp":	response.chain_head[2],
		},
	    };
	},
    },

    //
    // Zome entry
    //
    async create_zome_entry ( input ) {
	const result			= await this.call( input );

	return new Zome( result, this );
    },
    async create_zome ( input ) {
	if ( !ZOME_TYPE_NAMES.includes( input.zome_type ) )
	    throw new TypeError(`Invalid 'zome_type' input '${input.zome_type}'; expected ${ZOME_TYPE_NAMES.join(", ")}`);

	input.mere_memory_addr		= new EntryHash( input.mere_memory_addr );

	const result			= await this.call( input );

	return new Zome( result, this );
    },
    async get_zome_entry ( input ) {
	const result			= await this.call( new AnyDhtHash( input ) );

	return new Zome( result, this );
    },
    async get_zome_asset ( input ) {
	const result			= await this.call( new EntryHash( input ) );

	// Run potential decompression
	result.bytes			= await this.zomes.mere_memory_api.decompress_memory([
	    result.memory_entry,
	    new Uint8Array( result.bytes ),
	]);

	return ZomeAsset( result );
    },
    async get_zome_entries_for_agent ( input ) {
	const entries			= await this.call( input ? new AgentPubKey( input ) : input );

	return Object.fromEntries(
	    entries.map( entry => {
		const zome		= new Zome( entry, this );
		return [
		    zome.$addr,
		    zome,
		];
	    })
	);
    },
    async delete_zome ( input ) {
	return new ActionHash( await this.call( new ActionHash( input ) ) );
    },


    //
    // Zome package entry
    //
    async create_zome_package_entry ( input ) {
	const result			= await this.call( input );
        const zome_package              = new ZomePackage( result, this );

	return zome_package;
    },
    async create_zome_package ( input ) {
	const result			= await this.call( input );
        const zome_package              = new ZomePackage( result, this );

        if ( zome_package.maintainer.type === "group" ) {
            await this.zomes.coop_content_csr.create_content_link({
                "group_id": zome_package.maintainer.content[0],
                "content_target": zome_package.$id,
                "content_type": "zome_package",
            });
        }

	return zome_package;
    },
    async update_zome_package ( input ) {
	const result			= await this.call( input );
        const zome_package              = new ZomePackage( result, this );

        if ( zome_package.maintainer.type === "group" ) {
            await this.zomes.coop_content_csr.create_content_update_link({
                "group_id": zome_package.maintainer.content[0],
                "content_id": zome_package.$id,
                "content_prev": input.base,
                "content_next": zome_package.$action,
            });
        }

	return zome_package;
    },
    async get_zome_package ( input ) {
	const result			= await this.call( new ActionHash( input ) );

	return new ZomePackage( result, this );
    },
    async get_zome_package_by_name ( input ) {
	const result			= await this.call( input );

	return new ZomePackage( result, this );
    },
    async get_zome_package_entry ( input ) {
	const result			= await this.call( new AnyDhtHash( input ) );

	return new ZomePackage( result, this );
    },
    async get_zome_packages_for_agent ( input ) {
	const entries			= await this.call( input ? new AgentPubKey( input ) : input );

	return Object.fromEntries(
	    entries.map( entry => {
		const zome_pack		= new ZomePackage( entry, this );
		return [
		    zome_pack.$id,
		    zome_pack,
		];
	    })
	);
    },
    async get_zome_package_versions ( input ) {
	const version_map		= await this.call( input );

	for ( let [vtag, pack_version] of Object.entries(version_map) ) {
	    version_map[ vtag ]		= new ZomePackageVersion( pack_version, this );
	    version_map[ vtag ].version	= vtag;
	}

	return version_map;
    },

    // Zome Package Links
    async create_zome_package_link_to_version ( input ) {
	return new ActionHash( await this.call( input ) );
    },
    async delete_zome_package_links_to_version ( input ) {
	let deleted_links		= await this.call( input );

	return deleted_links.map( addr => new ActionHash( addr ) );
    },
    async get_zome_package_version_links ( input ) {
	const link_map			= await this.call( input );

	for ( let [key, value] of Object.entries( link_map ) ) {
	    link_map[ key ]		= new Link( value );
	}

	return link_map;
    },
    async get_zome_package_version_targets ( input ) {
	const link_map			= await this.call( input );
	const version_names		= Object.keys( link_map );

	for ( let [key, value] of Object.entries( link_map ) ) {
	    link_map[ key ]		= new ActionHash( value );
	}

	this.log.info("Found %s versions for Zome Package (%s): %s", () => [
	    version_names.length, input, version_names.join(", ") ]);

	return link_map;
    },


    //
    // Zome package Version entry
    //
    async create_zome_package_version ( input ) {
	if ( typeof input.version !== "string" )
	    throw new TypeError(`Missing 'version' input`);

	const version_link_map		= await this.functions.get_zome_package_version_targets( input.for_package );

	if ( input.version in version_link_map )
	    throw new Error(`Version '${input.version}' already exists for package ${input.for_package}`);

	const result			= await this.call( input );

	result.content.version		= input.version;

        const zome_package              = await this.functions.get_zome_package( result.content.for_package );

        if ( zome_package.maintainer.type === "group" ) {
            await this.zomes.coop_content_csr.create_content_link({
                "group_id": zome_package.maintainer.content[0],
                "content_target": result.id,
                "content_type": "zome_package_version",
            });
        }

	return new ZomePackageVersion( result, this );
    },
    async create_zome_package_version_entry ( input ) {
	const result			= await this.call( input );

	return new ZomePackageVersion( result, this );
    },
    async get_zome_package_version_entry ( input ) {
	const result			= await this.call( new ActionHash( input ) );

	return new ZomePackageVersion( result, this );
    },
    async get_zome_package_version ( input ) {
	const result			= await this.call( new ActionHash( input ) );

	return new ZomePackageVersion( result, this );
    },
    async update_zome_package_version ( input ) {
        if ( input.properties.maintainer === undefined ) {
            const prev_zome_pack_vers   = await this.functions.get_zome_package_version_entry( input.base );
            const zome_package          = await this.functions.get_zome_package( prev_zome_pack_vers.for_package );
            input.properties.maintainer = zome_package.maintainer;

            if ( input.properties.maintainer.type === "group" ) {
                const group             = await this.zomes.coop_content_csr.get_group( input.properties.maintainer.content[0] );
                input.properties.maintainer.content[1] = group.$action;
            }
        }

	const result			= await this.call( input );

        const zome_pack_version         = new ZomePackageVersion( result, this );
        const zome_package              = await this.functions.get_zome_package( zome_pack_version.for_package );

        if ( zome_package.maintainer.type === "group" ) {
            await this.zomes.coop_content_csr.create_content_update_link({
                "group_id": zome_package.maintainer.content[0],
                "content_id": zome_pack_version.$id,
                "content_prev": input.base,
                "content_next": zome_pack_version.$action,
            });
        }

	return zome_package;
    },


    //
    // Virtual functions
    //
    async form_zome_entry ( bytes ) {
        return {
	    "zome_type":        ZOME_TYPES.INTEGRITY,
	    "mere_memory_addr": new EntryHash( input.mere_memory_addr ),
            "file_size":        bytes.length,
            "hash":             await this.zomes.mere_memory_api.calculate_hash( bytes ),
	};
    },
    async save_integrity ( bytes ) {
	const addr			= await this.zomes.mere_memory_api.save( bytes );

	return await this.functions.create_zome({
	    "zome_type": ZOME_TYPES.INTEGRITY,
	    "mere_memory_addr": addr,
	});
    },
    async save_coordinator ( bytes ) {
	const addr			= await this.zomes.mere_memory_api.save( bytes );

	return await this.functions.create_zome({
	    "zome_type": ZOME_TYPES.COORDINATOR,
	    "mere_memory_addr": addr,
	});
    },
    async get_zome ( input ) {
	const zome_entry		= await this.functions.get_zome_entry( input );

	zome_entry.bytes		= await this.zomes.mere_memory_api.remember(
	    zome_entry.mere_memory_addr
	);

	return zome_entry;
    },
    async get_zome_entry_memory ( input ) {
	const zome_entry		= await this.functions.get_zome_entry( input );

	return await this.zomes.mere_memory_api.remember( zome_entry.mere_memory_addr );
    },
    async get_zome_package_versions_sorted ( input ) {
	const version_map		= await this.functions.get_zome_package_versions( input );
	const versions			= [];

	semverReverseSort(
	    Object.keys( version_map )
	).forEach( vtag => {
	    versions.push( version_map[ vtag ] );
	});

	return versions;
    },
    async download_zome_package ( input ) {
	const zome_package		= await this.functions.get_zome_package_by_name( input );
	const versions			= await this.functions.get_zome_package_versions_sorted( zome_package.$id );
        const latest_version            = versions[0];
        const zome                      = await this.functions.get_zome( latest_version.zome_entry );

	return [
            zome_package,
            latest_version,
            zome,
        ];
    },
    async get_zome_packages_for_group ( group_id ) {
        const targets                   = await this.zomes.coop_content_csr.get_all_group_content_targets({
            group_id,
            "content_type":     "zome_package",
        });

        return await Promise.all(
            targets.map( async ([ _id, latest ]) => {
                return await this.functions.get_zome_package_entry( latest );
            })
        );
    },
}, {
    "zomes": {
	"mere_memory_api": MereMemoryZomelet,
        "coop_content_csr": CoopContentZomelet,
    },
});


export const ZomeHubCell		= new CellZomelets({
    "zomehub_csr": ZomeHubCSRZomelet,
    "mere_memory_api": MereMemoryZomelet,
    "coop_content_csr": CoopContentZomelet,
});


export *				from '@spartan-hc/mere-memory-zomelets';
export *				from '@spartan-hc/coop-content-zomelets';
export *				from './types.js';

export default {
    ZOME_TYPES,
    ZOME_TYPE_NAMES,

    // Zomelets
    ZomeHubCSRZomelet,
    MereMemoryZomelet,
    CoopContentZomelet,

    // CellZomelets
    ZomeHubCell,
};
