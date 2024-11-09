// 217kb = (-57kb duplicates) 43 + 120 (dnahub) + this
import {
    AnyDhtHash,
    AgentPubKey,
    ActionHash, EntryHash,
}					from '@spartan-hc/holo-hash'; // approx. 11kb
import {
    Zomelet,
    CellZomelets,
}					from '@spartan-hc/zomelets'; // approx. 7kb
import { Bundle }			from '@spartan-hc/bundles'; // approx. 39kb
import { // Relative import is causing duplicates (holo-hash, zomelets, bundles)
    DnaHubCSRZomelet,
    ZomeHubCSRZomelet,
    MereMemoryZomelet,

    DnaHubCell,
    ZomeHubCell,
}					from '@holochain/dnahub-zomelets'; // approx. 118kb
import {
    rsort as semverReverseSort
}					from 'semver'; // approx. 32kb
import {
    Link,
    WebAppPackageVersionEntry,

    // Entity Classes
    App,
    Ui,
    WebApp,
    WebAppPackage,
    WebAppPackageVersion,
    AppAsset,
    UiAsset,
    WebAppAsset,
}					from './types.js';



export const AppHubCSRZomelet		= new Zomelet({
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


    // App

    async create_app_entry ( input ) {
	this.log.trace("Create App entry input (manifest):", input.manifest );
	const result			= await this.call( input );

	return new App( result, this );
    },
    async create_app ( input ) {
	this.log.trace("Create App input (manifest):", input.manifest );
	const result			= await this.call( input );

	return new App( result, this );
    },
    async get_app_entry ( input ) {
	const result			= await this.call( new EntryHash( input ) );

	return new App( result, this );
    },
    async get_app_asset ( input ) {
	const result			= await this.call( new EntryHash( input ) );
	const app_asset			= AppAsset( result );
	const manifest			= app_asset.app_entry.manifest;

	// Run potential decompression
	for ( let dna_asset of Object.values( app_asset.dna_assets ) ) {
	    for ( let zome_asset of Object.values( dna_asset.zome_assets ) ) {
		zome_asset.bytes	= await this.zomes.mere_memory_api.decompress_memory([
		    zome_asset.memory_entry,
		    new Uint8Array( zome_asset.bytes ),
		]);
	    }
	}

	// Verify asset hashes
	for ( let role_manifest of manifest.roles ) {
	    const dna_asset		= app_asset.dna_assets[ role_manifest.name ];
	    const dna_manifest		= dna_asset.dna_entry.manifest;

	    for ( let zome_manifest of dna_manifest.integrity.zomes ) {
		const hash			= await this.zomes.mere_memory_api.calculate_hash(
		    dna_asset.zome_assets[ zome_manifest.name ].bytes
		);
		const expected_hash		= dna_asset.dna_entry.asset_hashes.integrity[ zome_manifest.name ];

		if ( hash !== expected_hash )
		    throw new Error(`Asset hash for integrity zome '${zome_manifest.name}' is invalid; ${hash} !== ${expected_hash} (expected)`);
	    }

	    for ( let zome_manifest of dna_manifest.coordinator.zomes ) {
		const hash			= await this.zomes.mere_memory_api.calculate_hash(
		    dna_asset.zome_assets[ zome_manifest.name ].bytes
		);
		const expected_hash		= dna_asset.dna_entry.asset_hashes.coordinator[ zome_manifest.name ];

		if ( hash !== expected_hash )
		    throw new Error(`Asset hash for coordinator zome '${zome_manifest.name}' is invalid; ${hash} !== ${expected_hash} (expected)`);
	    }
	}

	return app_asset;
    },
    async get_app_entries_for_agent ( input ) {
	const agent_id			= input ? new AgentPubKey( input ) : input;
	const entries			= await this.call( agent_id );

	return entries.map( entry => new App( entry, this ) );
    },
    async delete_app ( input ) {
	return new ActionHash( await this.call( new ActionHash( input ) ) );
    },


    // UI

    async create_ui_entry ( input ) {
	const result			= await this.call( input );

	return new Ui( result, this );
    },
    async create_ui ( input ) {
	input.mere_memory_addr		= new EntryHash( input.mere_memory_addr );

	const result			= await this.call( input );

	return new Ui( result, this );
    },
    async get_ui_entry ( input ) {
	const result			= await this.call( new EntryHash( input ) );

	return new Ui( result, this );
    },
    async get_ui_asset ( input ) {
	const result			= await this.call( new EntryHash( input ) );

	// Run potential decompression
	result.bytes			= await this.zomes.mere_memory_api.decompress_memory([
	    result.memory_entry,
	    new Uint8Array( result.bytes ),
	]);

	return UiAsset( result );
    },
    async get_ui ( input ) {
	const ui_entry			= await this.functions.get_ui_entry( input );

	ui_entry.bytes			= await this.zomes.mere_memory_api.remember(
	    ui_entry.mere_memory_addr
	);

	return ui_entry;
    },
    async get_ui_entry_memory ( input ) {
	const ui_entry			= await this.functions.get_ui_entry( input );

	return await this.zomes.mere_memory_api.remember( ui_entry.mere_memory_addr );
    },
    async get_ui_entries_for_agent ( input ) {
	const entries			= await this.call(); // new AgentPubKey( input )

	return entries.map( entry => new Ui( entry, this ) );
    },
    async delete_ui ( input ) {
	return new ActionHash( await this.call( new ActionHash( input ) ) );
    },


    // WebApp

    async create_webapp_entry ( input ) {
	this.log.trace("Create WebApp entry input (manifest):", input.manifest );
	const result			= await this.call( input );

	return new WebApp( result, this );
    },
    async create_webapp ( input ) {
	this.log.trace("Create WebApp input (manifest):", input.manifest );
	const result			= await this.call( input );

	return new WebApp( result, this );
    },
    async get_webapp_entry ( input ) {
	const result			= await this.call( new AnyDhtHash( input ) );

	return new WebApp( result, this );
    },
    async get_webapp_asset ( input ) {
	const result			= await this.call( new EntryHash( input ) );
	const webapp_asset		= WebAppAsset( result );
	const manifest			= webapp_asset.webapp_entry.manifest;
	const app_manifest		= webapp_asset.app_asset.app_entry.manifest;

	// Run potential decompression
	for ( let dna_asset of Object.values( webapp_asset.app_asset.dna_assets ) ) {
	    for ( let zome_asset of Object.values( dna_asset.zome_assets ) ) {
		zome_asset.bytes	= await this.zomes.mere_memory_api.decompress_memory([
		    zome_asset.memory_entry,
		    new Uint8Array( zome_asset.bytes ),
		]);
	    }
	}

	webapp_asset.ui_asset.bytes	= await this.zomes.mere_memory_api.decompress_memory([
	    webapp_asset.ui_asset.memory_entry,
	    new Uint8Array( webapp_asset.ui_asset.bytes ),
	]);

	// Verify asset hashes
	for ( let role_manifest of app_manifest.roles ) {
	    const dna_asset		= webapp_asset.app_asset.dna_assets[ role_manifest.name ];
	    const dna_manifest		= dna_asset.dna_entry.manifest;

	    for ( let zome_manifest of dna_manifest.integrity.zomes ) {
		const hash			= await this.zomes.mere_memory_api.calculate_hash(
		    dna_asset.zome_assets[ zome_manifest.name ].bytes
		);
		const expected_hash		= dna_asset.dna_entry.asset_hashes.integrity[ zome_manifest.name ];

		if ( hash !== expected_hash )
		    throw new Error(`Asset hash for integrity zome '${zome_manifest.name}' is invalid; ${hash} !== ${expected_hash} (expected)`);
	    }

	    for ( let zome_manifest of dna_manifest.coordinator.zomes ) {
		const hash			= await this.zomes.mere_memory_api.calculate_hash(
		    dna_asset.zome_assets[ zome_manifest.name ].bytes
		);
		const expected_hash		= dna_asset.dna_entry.asset_hashes.coordinator[ zome_manifest.name ];

		if ( hash !== expected_hash )
		    throw new Error(`Asset hash for coordinator zome '${zome_manifest.name}' is invalid; ${hash} !== ${expected_hash} (expected)`);
	    }
	}

	return webapp_asset;
    },
    async get_webapp_entries_for_agent ( input ) {
	const agent_id			= input ? new AgentPubKey( input ) : input;
	const entries			= await this.call( agent_id );

	return entries.map( entry => new WebApp( entry, this ) );
    },
    async delete_webapp ( input ) {
	return new ActionHash( await this.call( new ActionHash( input ) ) );
    },


    // WebApp Package

    async create_webapp_package ( input ) {
	this.log.trace("Create WebApp package input:", input );
	input.icon			= await this.zomes.mere_memory_api.save( input.icon );

	const result			= await this.call( input );

	return new WebAppPackage( result, this );
    },
    async create_webapp_package_entry ( input ) {
	this.log.trace("Create WebApp package entry input:", input );
	const result			= await this.call( input );

	return new WebAppPackage( result, this );
    },
    async get_webapp_package ( input ) {
	const result			= await this.call( new ActionHash( input ) );

	return new WebAppPackage( result, this );
    },
    async get_webapp_package_entry ( input ) {
	const result			= await this.call( new AnyDhtHash( input ) );

	return new WebAppPackage( result, this );
    },
    async get_webapp_package_versions ( input ) {
	const version_map		= await this.call( input );

	for ( let [vtag, pack_version] of Object.entries(version_map) ) {
	    version_map[ vtag ]		= new WebAppPackageVersion( pack_version, this );
	    version_map[ vtag ].version	= vtag;
	}

	return version_map;
    },
    async get_all_webapp_packages ( input ) {
	const entries			= await this.call(); // new AgentPubKey( input )

	return entries.map( entity => new WebAppPackage( entity, this ) );
    },
    async update_webapp_package ( input ) {
	if ( input.icon && input.icon.length > 39 )
	    input.icon			= await this.zomes.mere_memory_api.save( input.icon );

	this.log.trace("Update WebApp package input:", input );
	const result			= await this.call( input );

	return new WebAppPackage( result, this );
    },
    async deprecate_webapp_package ( input ) {
	return await this.call( input );
    },
    async delete_webapp_package ( input ) {
	return new ActionHash( await this.call( new ActionHash( input ) ) );
    },

    // WebApp Package Links
    async create_webapp_package_link_to_version ( input ) {
	return new ActionHash( await this.call( input ) );
    },
    async delete_webapp_package_links_to_version ( input ) {
	let deleted_links		= await this.call( input );

	return deleted_links.map( addr => new ActionHash( addr ) );
    },
    async get_webapp_package_version_links ( input ) {
	const link_map			= await this.call( input );

	for ( let [key, value] of Object.entries( link_map ) ) {
	    link_map[ key ]		= new Link( value );
	}

	return link_map;
    },
    async get_webapp_package_version_targets ( input ) {
	const link_map			= await this.call( input );
	const version_names		= Object.keys( link_map );

	for ( let [key, value] of Object.entries( link_map ) ) {
	    link_map[ key ]		= new ActionHash( value );
	}

	this.log.info("Found %s versions for WebApp Package (%s): %s", () => [
	    version_names.length, input, version_names.join(", ") ]);

	return link_map;
    },


    // WebApp Package Version

    async create_webapp_package_version ( input ) {
	if ( typeof input.version !== "string" )
	    throw new TypeError(`Missing 'version' input`);

	const version_link_map		= await this.functions.get_webapp_package_version_targets( input.for_package );

	if ( input.version in version_link_map )
	    throw new Error(`Version '${input.version}' already exists for package ${input.for_package}`);

	const result			= await this.call( input );

	result.content.version		= input.version;

	return new WebAppPackageVersion( result, this );
    },
    async create_webapp_package_version_entry ( input ) {
	const result			= await this.call( input );

	return new WebAppPackageVersion( result, this );
    },
    async update_webapp_package_version ( input ) {
	this.log.trace("Update WebApp package versioninput:", input );
	const result			= await this.call( input );

	return new WebAppPackageVersion( result, this );
    },
    async get_webapp_package_version_entry ( input ) {
	const result			= await this.call( new ActionHash( input ) );

	return WebAppPackageVersionEntry( result );
    },
    async get_webapp_package_version ( input ) {
	const result			= await this.call( new ActionHash( input ) );

	return new WebAppPackageVersion( result, this );
    },
    async move_webapp_package_version ( input ) {
	const result			= await this.call( input );

	return new WebAppPackageVersion( result, this );
    },
    async delete_webapp_package_version ( input ) {
	return new ActionHash( await this.call( new ActionHash( input ) ) );
    },


    //
    // Virtual functions
    //
    async save_app ( bytes ) {
	const claimed_file_size		= bytes.length;
	const bundle			= new Bundle( bytes, "happ" );
	const roles_dna_tokens		= {};

	for ( let role of bundle.manifest.roles ) {
	    let name			= role.name;
	    let rpath			= role.dna.bundled;
	    let dna_bytes		= bundle.resources[ rpath ];

	    this.log.debug("Save DNA resource '%s' (%s bytes) for role '%s'", () => [
		rpath, dna_bytes.length, name,
	    ]);
	    let dna			= await this.cells.dnahub.dnahub_csr.save_dna( dna_bytes )
	    this.log.info("Created new DNA entry '%s' for role '%s'", () => [
		dna.$id, name,
	    ]);

	    bundle.resources[ rpath ]	= {
		"dna": this.zome.cells.dnahub.dna,
		"target": dna.$addr,
	    };

	    roles_dna_tokens[ name ]	= dna.dna_token;
	}

	return await this.functions.create_app({
	    "manifest": bundle.manifest,
	    "resources": bundle.resources,
	    roles_dna_tokens,
	    claimed_file_size,
	});
    },
    async save_ui ( bytes ) {
	const addr			= await this.zomes.mere_memory_api.save( bytes );

	return await this.functions.create_ui({
	    "mere_memory_addr": addr,
	});
    },
    async save_webapp ( bytes ) {
	const bundle			= new Bundle( bytes, "webhapp" );

	{
	    const happ_manifest		= bundle.manifest.happ_manifest;
	    const rpath			= happ_manifest.bundled;
	    const happ_bytes		= bundle.resources[ rpath ];
	    this.log.debug("Save hApp resource '%s' (%s bytes)", rpath, happ_bytes.length );

	    let app			= await this.functions.save_app( happ_bytes );

	    bundle.resources[ rpath ]	= app.$addr;;
	}
	{
	    const ui_manifest		= bundle.manifest.ui;
	    const rpath			= ui_manifest.bundled;
	    const ui_bytes		= bundle.resources[ rpath ];
	    this.log.debug("Save UI resource '%s' (%s bytes)", rpath, ui_bytes.length );

	    let ui			= await this.functions.save_ui( ui_bytes );

	    bundle.resources[ rpath ]	= ui.$addr;;
	}

	return await this.functions.create_webapp({
	    "manifest": bundle.manifest,
	    "resources": bundle.resources,
	});
    },
    async get_webapp_package_versions_sorted ( input ) {
	const version_map		= await this.functions.get_webapp_package_versions( input );
	const versions			= [];

	semverReverseSort(
	    Object.keys( version_map )
	).forEach( vtag => {
	    versions.push( version_map[ vtag ] );
	});

	return versions;
    },

    // Might require virtual cell dependency
    async get_app_dna_entry ( input ) {
	const app_entry			= await this.functions.get_app_entry( input.app_entry );
	const role_manifest		= app_entry.manifest.roles.find(
	    role_manifest => role_manifest.name === input.name
	);

	if ( !role_manifest )
	    throw new Error(`App entry (${input.app_entry}) does not have a role named '${input.name}'`);

	const dna_hrl			= app_entry.resources[ role_manifest.dna.bundled ];
	const dnahub			= this.getCellInterface( "dnahub", dna_hrl.dna );

	return await dnahub.dnahub_csr.get_dna_entry( dna_hrl.target );
    },
    // "get_app_bundle":			"get_happ_bundle",
    async get_happ_bundle ( input ) {
	const app_entry			= await this.functions.get_app_entry( input );
	const manifest			= app_entry.manifest;
	const resources			= {};

	this.log.info("Fetch assets for App manifest:", manifest );
	for ( let role_manifest of manifest.roles ) {
	    const rpath			= role_manifest.dna.bundled;
	    const dna_hrl		= app_entry.resources[ rpath ];
	    const dnahub		= this.getCellInterface( "dnahub", dna_hrl.dna );

	    resources[ rpath ]		= await dnahub.dnahub_csr.get_dna_bundle( dna_hrl.target );
	}

	const bundle			= new Bundle({
	    "manifest":		{
		"manifest_version": "1",
		...manifest,
	    },
	    resources,
	}, "happ");

	return bundle.toBytes({ sortKeys: true });
    },
    // "get_webapp_bundle":		"get_webhapp_bundle",
    async get_webhapp_bundle ( input ) {
	const webapp_entry		= await this.functions.get_webapp_entry( input );
	const manifest			= webapp_entry.manifest;
	const resources			= {};
	this.log.info("Fetch assets for WebApp manifest:", manifest );

	{
	    const rpath			= manifest.ui.bundled;
	    resources[ rpath ]		= await this.functions.get_ui_entry_memory(
		webapp_entry.resources[ rpath ]
	    );
	}

	{
	    const rpath			= manifest.happ_manifest.bundled;
	    resources[ rpath ]		= await this.functions.get_happ_bundle(
		webapp_entry.resources[ rpath ]
	    );
	}

	const bundle			= new Bundle({
	    "manifest":		{
		"manifest_version": "1",
		...manifest,
	    },
	    resources,
	}, "webhapp");

	return bundle.toBytes({ sortKeys: true });
    },
    async bundle_from_app_asset ( app_asset ) {
	const manifest			= app_asset.app_entry.manifest;
	const resources			= {};

	for ( let role_manifest of manifest.roles ) {
	    const rpath			= role_manifest.dna.bundled;
	    const dna_bundle		= await this.cells.dnahub.dnahub_csr.bundle_from_dna_asset(
		app_asset.dna_assets[ role_manifest.name ]
	    );
	    resources[ rpath ]		= dna_bundle.toBytes({ sortKeys: true });
	}

	return new Bundle({
	    "manifest":		{
		"manifest_version": "1",
		...manifest,
	    },
	    resources,
	}, "happ");
    },
    async bundle_from_webapp_asset ( webapp_asset ) {
	const manifest			= webapp_asset.webapp_entry.manifest;
	const resources			= {};

	{
	    const app_bundle		= await this.functions.bundle_from_app_asset(
		webapp_asset.app_asset
	    );
	    const rpath			= manifest.happ_manifest.bundled;
	    resources[ rpath ]		= app_bundle.toBytes({ sortKeys: true });
	}

	{
	    const rpath			= manifest.ui.bundled;
	    resources[ rpath ]		= new Uint8Array( webapp_asset.ui_asset.bytes );
	}

	return new Bundle({
	    "manifest":		{
		"manifest_version": "1",
		...manifest,
	    },
	    resources,
	}, "webhapp");
    },
}, {
    "zomes": {
	"mere_memory_api": MereMemoryZomelet,
    },
    "cells": {
	"dnahub": DnaHubCell,
    },
    "virtual": {
	"cells": {
	    "dnahub": DnaHubCell,
	},
    },
});


export const AppHubCell			= new CellZomelets({
    "apphub_csr": AppHubCSRZomelet,
    "mere_memory_api": MereMemoryZomelet,
});

export  {
    DnaHubCSRZomelet,
    ZomeHubCSRZomelet,
    MereMemoryZomelet,
    DnaHubCell,
    ZomeHubCell,
}					from '@holochain/dnahub-zomelets';
export *				from './types.js';

export {
    Bundle,
    semverReverseSort,
};

export default {
    Bundle,
    semverReverseSort,

    // Zomelets
    AppHubCSRZomelet,
    DnaHubCSRZomelet,
    ZomeHubCSRZomelet,
    MereMemoryZomelet,

    // CellZomelets
    AppHubCell,
    DnaHubCell,
    ZomeHubCell,
};
