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

const encoder                           = new TextEncoder();

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
    async list_all_agents () {
	const result			= await this.call();

	return result.map( agent => new AgentPubKey(agent) );
    },

    //
    // Group links
    //
    async create_named_group_link ([ name, group_id ]) {
        if ( name.startsWith("@") )
            name                        = name.slice(1);

	const result			= await this.call([ name, group_id ]);

	return [
            new ActionHash(result[0]),
            result[1]
                ? new ActionHash( result[1] )
                : null,
        ];
    },
    async remove_named_group_link ( name ) {
        if ( name.startsWith("@") )
            name                        = name.slice(1);

	const result			= await this.call( name );

        return result.map( addr => new ActionHash( addr ) );
    },
    async get_my_group_links () {
	const result			= await this.call();

        return result.map( data => new Link(data) );
    },
    async get_org_group_links ( input ) {
        if ( input.startsWith("@") )
            input                       = input.slice(1);

	const result			= await this.call( input );

        return result.map( data => new Link(data) );
    },
    async get_all_org_group_links () {
	const result			= await this.call();

        return result.map( data => new Link(data) );
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
        if ( input.properties.maintainer === undefined ) {
            const prev_zome_pack        = await this.functions.get_zome_package_entry( input.base );
            input.properties.maintainer = prev_zome_pack.maintainer;

            if ( input.properties.maintainer.type === "group" ) {
                const group             = await this.zomes.coop_content_csr.get_group( input.properties.maintainer.content[0] );
                input.properties.maintainer.content[1] = group.$action;
            }
        }

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
    async get_all_zome_package_links () {
	const links			= await this.call();

        return links.map( data => new Link(data) );
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
    async delete_zome_package ( input ) {
	return await this.call( input );
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
	if ( await this.functions.zome_package_version_exists( input ) )
	    throw new Error(`Version '${input.version}' already exists for package ${input.for_package}`);

        if ( input.maintainer === undefined ) {
            const zome_package          = await this.functions.get_zome_package( input.for_package );
            input.maintainer            = zome_package.maintainer;

            if ( input.maintainer.type === "group" ) {
                const group             = await this.zomes.coop_content_csr.get_group( input.maintainer.content[0] );
                input.maintainer.content[1] = group.$action;
            }
        }

        // Convert readme contents to memory hash
        if ( input.readme && input.readme.length !== 39 )
	    input.readme		= await this.zomes.mere_memory_api.save( input.readme );

        // Convert changelog contents to memory hash
        if ( input.changelog && input.changelog.length !== 39 )
	    input.changelog		= await this.zomes.mere_memory_api.save( input.changelog );

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

        if ( typeof input.properties.readme === "string" )
            input.properties.readme     = encoder.encode( input.properties.readme );
        if ( typeof input.properties.changelog === "string" )
            input.properties.changelog     = encoder.encode( input.properties.changelog );

        // Convert readme contents to memory hash
        if ( input.properties.readme && input.properties.readme.length !== 39 )
	    input.properties.readme     = await this.zomes.mere_memory_api.save( input.properties.readme );

        // Convert changelog contents to memory hash
        if ( input.properties.changelog && input.properties.changelog.length !== 39 )
	    input.properties.changelog  = await this.zomes.mere_memory_api.save( input.properties.changelog );

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
    async delete_zome_package_version ( input ) {
	return await this.call( input );
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

    // Virtual functions related to zomes
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
    async get_zome_by_wasm_hash ( input ) {
        const my_zomes                  = await this.functions.get_zome_entries_for_agent();
        const matching_zome             = Object.values( my_zomes )
            .find( zome => zome.hash === input.hash );

        if ( input.zome_type && matching_zome && matching_zome.zome_type !== input.zome_type )
            throw new TypeError(`Existing zome with WASM hash '${input.hash}' has a different zome type; found '${matching_zome.zome_type}' but expected '${input.zome_type}'`);

        return matching_zome;
    },

    // Virtual functions related to zome packages
    async get_existing_zome_package ( input ) {
        const all_links                 = await this.functions.get_all_zome_package_links();
	const existing_link             = all_links.find( link => {
	    return link.tagString() === input.name;
	});

        if ( !existing_link )
            return null;

        const existing_package          = await this.functions.get_zome_package( existing_link.target );

        if ( existing_package.zome_type !== input.zome_type )
            throw new TypeError(`Existing zome package for '${input.name}' has a different zome type; found '${existing_package?.zome_type}' but expected '${input.zome_type}'`);

        return existing_package;
    },

    // Virtual functions related to zome package versions
    async get_existing_zome_package_version ( input ) {
	const version_target_map	= await this.functions.get_zome_package_version_targets( input.for_package );
        const zome_version_id           = version_target_map[ input.version ];

	if ( !zome_version_id )
            return null;

        const zome_version              = await this.functions.get_zome_package_version( zome_version_id );

        zome_version.version            = input.version;

        return zome_version;
    },
    async zome_package_version_exists ( input ) {
	const versions_target_map	= await this.functions.get_zome_package_version_targets( input.for_package );
	this.log.info("Versions for zome package '%s': %s", () => [
	    input.for_package, Object.keys(versions_target_map).join(", ") ]);

	return input.version in versions_target_map;
    },
    async zome_package_version_with_hash_exists ( input ) {
	const pack_versions		= await this.functions.get_zome_package_versions_sorted( input.for_package );
	const versions_with_zomes       = await Promise.all(
            pack_versions.map( async (zome_version) => {
                return [
                    zome_version,
                    await this.functions.get_zome_entry( zome_version.zome_entry ),
                ];
            })
        );

        return versions_with_zomes.find( ([zome_version, zome]) => zome.hash === input.hash );
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
    async download_zome_package ({ name, version }) {
	const zome_package		= await this.functions.get_zome_package_by_name( name );
	const versions			= await this.functions.get_zome_package_versions_sorted( zome_package.$id );

        if ( versions.length === 0 )
            throw new Error(`No versions for zome package '${name}'`);

        let package_version;

        if ( version ) {
            package_version             = versions.find( pack_version => pack_version.version === version );

            if ( !package_version )
                throw new Error(`Version '${version}' not found; available versions are: ${versions.map(pv => pv.version).join(", ")}`);
        }
        else {
            package_version             = versions[0];
	    this.log.info("Using latest version: %s", package_version.version );
        }

        const zome                      = await this.functions.get_zome( package_version.zome_entry );

	return [
            zome_package,
            package_version,
            zome,
        ];
    },

    // Virtual functions related to org groups
    async create_org ( input ) {
        const group                     = await this.zomes.coop_content_csr.create_group({
            "admins":           input.admins || [],
            "members":          input.members || [],
            "published_at":     Date.now(),
            "last_updated":     Date.now(),
            "metadata":         {},
        });
        await this.functions.create_named_group_link([
            input.name, group.$id
        ]);

        return group;
    },
    async get_my_orgs ( input ) {
        const org_group_links           = await this.functions.get_my_group_links();

        return await Promise.all(
            org_group_links.map( async (link) => {
                return {
                    "name":     link.tagString(),
                    "group":    await this.zomes.coop_content_csr.get_group( link.target ),
                };
            })
        );
    },
    async get_my_org_invites () {
        const invites                   = await this.zomes.coop_content_csr.get_my_invites();
        const all_org_group_links       = await this.functions.get_all_org_group_links();
        all_org_group_links.reverse();
        const org_name_map              = {};

        for ( let org_link of all_org_group_links ) {
            const group_id              = String( org_link.target );
            org_name_map[ group_id ]    = org_link.tagString();
        }

        // Map invite group ID to an org name
        return invites.map( ({ link, group }) => {
            return {
                "name":     org_name_map[ group.$id ],
                "invite":   {
                    "link":     new Link( link ),
                    group,
                },
            };
        });
    },
    async accept_invitation_to_group ( org_name ) {
        if ( !org_name.startsWith("@") )
            org_name            = `@${org_name}`;

        const orgs              = await this.functions.get_my_org_invites();
        const org_invite        = orgs.find( org => org.name === org_name.slice(1) );

        if ( !org_invite )
            throw new Error(`You have no invites from org '${org_name}'`);

        const group_id          = org_invite.invite.group.$id;

        const accepted          = await this.zomes.coop_content_csr.accept_invitation_to_group( group_id );

        // Add to my orgs
        await this.functions.create_named_group_link([
            org_name, group_id
        ]);

        return accepted;
    },
    async get_my_groups ( input ) {
        const org_group_links           = await this.functions.get_my_group_links();

        return await Promise.all(
            org_group_links.map( async (link) => await this.zomes.coop_content_csr.get_group( link.target ) )
        );
    },
    async get_groups_by_name ( input ) {
        const org_group_links           = await this.functions.get_org_group_links( input );

        return await Promise.all(
            org_group_links.map( async (link) => await this.zomes.coop_content_csr.get_group( link.target ) )
        );
    },
    async get_group_by_name ( input ) {
        const org_group_links           = await this.functions.get_org_group_links( input );

        if ( org_group_links.length === 0 )
            throw new Error(`Found 0 groups named '${input}'`);

        return await this.zomes.coop_content_csr.get_group( org_group_links[0].target );
    },
    async get_zome_packages_for_group ( group_id ) {
        const targets                   = await this.zomes.coop_content_csr.get_all_group_content_targets({
            group_id,
            "content_type":     "zome_package",
        });

        return (await Promise.all(
            targets.map( async ([ _id, latest ]) => {
                try {
                    return await this.functions.get_zome_package_entry( latest );
                } catch (err) {
	            this.log.warn("Failed to fetch zome package %s: %s", latest, String(err) );
                }
            })
        )).filter(Boolean);
    },
    async get_zome_package_targets_for_org ( org_name ) {
        const links                     = await this.functions.get_org_group_links( org_name );

        return await this.zomes.coop_content_csr.get_all_group_content_targets({
            "group_id":         new ActionHash( links[0].target ),
            "content_type":     "zome_package",
        });
    },
    async get_zome_packages_for_org ( org_name ) {
        const targets                   = await this.functions.get_zome_package_targets_for_org( org_name );

        return await Promise.all(
            targets.map( async ([ _id, latest ]) => {
                return await this.functions.get_zome_package_entry( latest );
            })
        );
    },

    // Virtual functions related to Mere Memory
    async remember_memory ( input, options ) {
        return await this.zomes.mere_memory_api.remember( input, options );
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
