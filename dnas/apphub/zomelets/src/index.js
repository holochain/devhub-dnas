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

    async create_app_entry ({ manifest, resources }) {
	this.log.info("App entry input (%s resources):", Object.keys(resources).length, manifest );
	const result			= await this.call({
	    "manifest": manifest,
	    "resources": resources,
	});

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

    async create_webapp_entry ({ manifest, resources }) {
	this.log.info("WebApp entry input (%s resources):", Object.keys(resources).length, manifest );
	const result			= await this.call({
	    "manifest": manifest,
	    "resources": resources,
	});

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

	this.log.info("WebApp package entry input:", input );
	const result			= await this.call( input );

	return new WebAppPackage( result, this );
    },
    async get_webapp_package_entry ( input ) {
	const result			= await this.call( new ActionHash( input ) );

	return new WebAppPackage( result, this );
    },
    "link_webapp_package_version":	true,
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
	const resources			= {};

	for ( let [ rpath, dna_bytes ] of Object.entries( bundle.resources ) ) {
	    this.log.info("Save DNA resource '%s' (%s bytes)", rpath, dna_bytes.length );
	    resources[ rpath ]		= await this.cells.dnahub.dnahub_csr.save_dna( dna_bytes );
	}

	return await this.functions.create_app_entry({
	    "manifest": bundle.manifest,
	    resources,
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
	const resources			= {};

	{
	    const rpath			= bundle.manifest.happ_manifest.bundled;
	    const happ_bytes		= bundle.resources[ rpath ];
	    this.log.info("Save hApp resource '%s' (%s bytes)", rpath, happ_bytes.length );
	    resources[ rpath ]		= await this.functions.save_app( happ_bytes );
	}
	{
	    const rpath			= bundle.manifest.ui.bundled;
	    const ui_bytes		= bundle.resources[ rpath ];
	    this.log.info("Save UI resource '%s' (%s bytes)", rpath, ui_bytes.length );
	    resources[ rpath ]		= await this.functions.save_ui( ui_bytes );
	}

	return await this.functions.create_webapp_entry({
	    "manifest": bundle.manifest,
	    resources,
	});
    },
    async create_webapp_package_version ( input ) {
	if ( typeof input.version !== "string" )
	    throw new TypeError(`Missing 'version' input`);

	const entity			= await this.functions.create_webapp_package_version_entry( input );

	await this.functions.link_webapp_package_version({
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
