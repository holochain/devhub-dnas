// 217kb = (-57kb duplicates) 43 + 120 (dnahub) + this
import {
    AgentPubKey,
    ActionHash, EntryHash,
}					from '@spartan-hc/holo-hash'; // approx. 11kb
import {
    Zomelet,
    CellZomelets,
}					from '@spartan-hc/zomelets'; // approx. 7kb
import { Bundle }			from '@spartan-hc/bundles'; // approx. 39kb
import { Entity }			from '@spartan-hc/caps-entities'; // approx. 19kb
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
    AppEntry,
    UiEntry,
    WebAppEntry,
    // WebAppPackageEntry,
    // WebAppPackageVersionEntry,

    // Entity Classes
    WebAppPackage,
    WebAppPackageVersion,
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
	this.log.trace("App entry manifest input:", input.manifest );
	const result			= await this.call( input );

	return new EntryHash( result );
    },
    async get_app_entry ( input ) {
	const result			= await this.call( new EntryHash( input ) );

	return AppEntry( result );
    },
    async get_app_entries_for_agent ( input ) {
	const entries			= await this.call(); // new AgentPubKey( input )

	return entries.map( entry => AppEntry( entry ) );
    },


    // UI

    async create_ui_entry ( input ) {
	const result			= await this.call({
	    "mere_memory_addr": new EntryHash( input.mere_memory_addr ),
	});

	return new EntryHash( result );
    },
    async get_ui_entry ( input ) {
	const result			= await this.call( new EntryHash( input ) );

	return UiEntry( result );
    },
    async get_ui_entries_for_agent ( input ) {
	const entries			= await this.call(); // new AgentPubKey( input )

	return entries.map( entry => UiEntry( entry ) );
    },


    // WebApp

    async create_webapp_entry ( input ) {
	this.log.trace("WebApp entry manifest input:", input.manifest );
	const result			= await this.call( input );

	return new EntryHash( result );
    },
    async get_webapp_entry ( input ) {
	const result			= await this.call( new EntryHash( input ) );

	return WebAppEntry( result );
    },
    async get_webapp_entries_for_agent ( input ) {
	const entries			= await this.call( input ? new AgentPubKey( input ) : input );

	return entries.map( entry => WebAppEntry( entry ) );
    },


    // WebApp Package

    async create_webapp_package_entry ( input ) {
	input.icon			= await this.zomes.mere_memory_api.save( input.icon );

	this.log.trace("WebApp package entry input:", input );
	const result			= await this.call( input );

	return new WebAppPackage( result, this );
    },
    async get_webapp_package_entry ( input ) {
	const result			= await this.call( new ActionHash( input ) );

	return new WebAppPackage( result, this );
    },
    "create_webapp_package_link_to_version":	true,
    async get_webapp_package_versions ( input ) {
	const version_map		= await this.call( input );

	for ( let key in version_map ) {
	    version_map[ key ]		= new WebAppPackageVersion( version_map[ key ], this );
	}

	return version_map;
    },
    async get_webapp_package_versions_sorted ( input ) {
	const version_map		= await this.functions.get_webapp_package_versions( input );
	const versions			= [];

	semverReverseSort(
	    Object.keys( version_map )
	).forEach( vtag => {
	    const webapp_pv		= version_map[ vtag ];
	    webapp_pv.version		= vtag;
	    versions.push( webapp_pv );
	});

	return versions;
    },
    async get_all_webapp_package_entries ( input ) {
	const entries			= await this.call(); // new AgentPubKey( input )

	return entries.map( entity => new WebAppPackage( entity, this ) );
    },


    // WebApp Package Version

    async create_webapp_package_version_entry ( input ) {
	const result			= await this.call( input );

	return new WebAppPackageVersion( result, this );
    },
    async get_webapp_package_version_entry ( input ) {
	const result			= await this.call( new ActionHash( input ) );

	return new WebAppPackageVersion( result, this );
    },


    //
    // Virtual functions
    //
    async save_app ( bytes ) {
	const bundle			= new Bundle( bytes, "happ" );
	const roles_dna_tokens		= {};

	for ( let role of bundle.manifest.roles ) {
	    let name			= role.name;
	    let rpath			= role.dna.bundled;
	    let dna_bytes		= bundle.resources[ rpath ];

	    this.log.debug("Save DNA resource '%s' (%s bytes) for role '%s'", () => [
		rpath, dna_bytes.length, name,
	    ]);
	    let dna_addr		= await this.cells.dnahub.dnahub_csr.save_dna( dna_bytes )
	    this.log.info("Created new DNA entry '%s' for role '%s'", () => [
		dna_addr, name,
	    ]);

	    let dna_entry		= await this.cells.dnahub.dnahub_csr.get_dna_entry( dna_addr );

	    role.dna.dna_entry		= dna_addr;
	    delete role.dna.bundled;

	    roles_dna_tokens[ name ]	= dna_entry.dna_token;
	}

	return await this.functions.create_app_entry({
	    "manifest": bundle.manifest,
	    roles_dna_tokens,
	});
    },
    async save_ui ( bytes ) {
	const addr			= await this.zomes.mere_memory_api.save( bytes );

	return await this.functions.create_ui_entry({
	    "mere_memory_addr": addr,
	});
    },
    async save_webapp ( bytes ) {
	const bundle			= new Bundle( bytes, "webhapp" );

	{
	    const happ_manifest		= bundle.manifest.happ_manifest;
	    const happ_bytes		= bundle.resources[ happ_manifest.bundled ];
	    this.log.debug("Save hApp resource '%s' (%s bytes)", happ_manifest.bundled, happ_bytes.length );

	    happ_manifest.app_entry	= await this.functions.save_app( happ_bytes );
	    delete happ_manifest.bundled;
	}
	{
	    const ui_manifest		= bundle.manifest.ui;
	    const ui_bytes		= bundle.resources[ ui_manifest.bundled ];
	    this.log.debug("Save UI resource '%s' (%s bytes)", ui_manifest.bundled, ui_bytes.length );

	    ui_manifest.ui_entry	= await this.functions.save_ui( ui_bytes );
	    delete ui_manifest.bundled;
	}

	return await this.functions.create_webapp_entry({
	    "manifest": bundle.manifest,
	});
    },
    async create_webapp_package_version ( input ) {
	if ( typeof input.version !== "string" )
	    throw new TypeError(`Missing 'version' input`);

	const entity			= await this.functions.create_webapp_package_version_entry( input );

	await this.functions.create_webapp_package_link_to_version({
	    "version":				input.version,
	    "webapp_package_id":		entity.for_package,
	    "webapp_package_version_id":	entity.$id,
	});

	return entity;
    },
}, {
    "zomes": {
	"mere_memory_api": MereMemoryZomelet,
    },
    "cells": {
	"dnahub": DnaHubCell,
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

export default {
    AppHubCSRZomelet,
    DnaHubCSRZomelet,
    ZomeHubCSRZomelet,
    MereMemoryZomelet,

    // CellZomelets
    AppHubCell,
    DnaHubCell,
    ZomeHubCell,
};
