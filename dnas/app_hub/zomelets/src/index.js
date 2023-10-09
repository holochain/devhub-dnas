
import {
    AgentPubKey,
    ActionHash, EntryHash,
}					from '@spartan-hc/holo-hash';
import {
    Zomelet,
    CellZomelets,
}					from '@spartan-hc/zomelets';
import { Bundle }			from '@spartan-hc/bundles';
import { Entity }			from '@spartan-hc/caps-entities';
import {
    DnaHubCSRZomelet,
    ZomeHubCSRZomelet,
    MereMemoryZomelet,

    DnaHubCell,
    ZomeHubCell,
}					from '@holochain/dna-hub-zomelets';
import {
    AppEntry,
    WebAppEntry,
    WebAppPackageEntry,

    // Entity Classes
    WebAppPackage,
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
    async create_app_entry ({ manifest, resources }) {
	this.log.info("App entry input (%s resources):", Object.keys(resources).length, manifest );
	const result			= await this.call({
	    "manifest": manifest,
	    "resources": resources,
	});

	return new ActionHash( result );
    },
    async get_app_entry ( input ) {
	const result			= await this.call( new ActionHash( input ) );

	return AppEntry( result );
    },

    async create_ui_entry ( input ) {
	const result			= await this.call({
	    "mere_memory_addr": new EntryHash( input.mere_memory_addr ),
	});

	return new ActionHash( result );
    },
    async get_ui_entry ( input ) {
	const result			= await this.call( new ActionHash( input ) );

	return AppEntry( result );
    },

    async create_webapp_entry ({ manifest, resources }) {
	this.log.info("WebApp entry input (%s resources):", Object.keys(resources).length, manifest );
	const result			= await this.call({
	    "manifest": manifest,
	    "resources": resources,
	});

	return new ActionHash( result );
    },
    async get_webapp_entry ( input ) {
	const result			= await this.call( new ActionHash( input ) );

	return WebAppEntry( result );
    },

    async create_webapp_package_entry ( input ) {
	input.icon			= await this.zomes.mere_memory_api.save( input.icon );

	this.log.info("WebApp package entry input:", input );
	const result			= await this.call( input );

	return new WebAppPackage( result );
    },
    async get_webapp_package_entry ( input ) {
	const result			= await this.call( new ActionHash( input ) );

	return new WebAppPackage( result );
    },


    //
    // Virtual functions
    //
    async save_app ( bytes ) {
	const bundle			= new Bundle( bytes, "happ" );
	const resources			= {};

	for ( let [ rpath, dna_bytes ] of Object.entries( bundle.resources ) ) {
	    this.log.info("Save DNA resource '%s' (%s bytes)", rpath, dna_bytes.length );
	    resources[ rpath ]		= await this.cells.dna_hub.dna_hub_csr.save_dna( dna_bytes );
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

	// const happ_bundle		= bundle.happ();
	// const ui_bytes			= bundle.ui();

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
}, {
    "zomes": {
	"mere_memory_api": MereMemoryZomelet,
    },
    "cells": {
	"dna_hub": DnaHubCell,
    },
});


export const AppHubCell			= new CellZomelets({
    "app_hub_csr": AppHubCSRZomelet,
    "mere_memory_api": MereMemoryZomelet,
});

export  {
    DnaHubCSRZomelet,
    ZomeHubCSRZomelet,
    MereMemoryZomelet,
    DnaHubCell,
    ZomeHubCell,
}					from '@holochain/dna-hub-zomelets';

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
