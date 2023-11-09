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
    Link,
    WebAppPackageVersionEntry,

    // Entity Classes
    App,
    Ui,
    WebApp,
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

	    role.dna.dna_entry		= dna.$addr;
	    delete role.dna.bundled;

	    roles_dna_tokens[ name ]	= dna.dna_token;
	}

	return await this.functions.create_app({
	    "manifest": bundle.manifest,
	    roles_dna_tokens,
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
	    const happ_bytes		= bundle.resources[ happ_manifest.bundled ];
	    this.log.debug("Save hApp resource '%s' (%s bytes)", happ_manifest.bundled, happ_bytes.length );

	    let app			= await this.functions.save_app( happ_bytes );

	    happ_manifest.app_entry	= app.$addr;
	    delete happ_manifest.bundled;
	}
	{
	    const ui_manifest		= bundle.manifest.ui;
	    const ui_bytes		= bundle.resources[ ui_manifest.bundled ];
	    this.log.debug("Save UI resource '%s' (%s bytes)", ui_manifest.bundled, ui_bytes.length );

	    let ui			= await this.functions.save_ui( ui_bytes );

	    ui_manifest.ui_entry	= ui.$addr;
	    delete ui_manifest.bundled;
	}

	return await this.functions.create_webapp({
	    "manifest": bundle.manifest,
	});
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
