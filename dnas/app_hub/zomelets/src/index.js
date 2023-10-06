
import {
    AgentPubKey,
    ActionHash, EntryHash,
}					from '@spartan-hc/holo-hash';
import {
    Zomelet,
    CellZomelets,
}					from '@spartan-hc/zomelets';
import {
    Bundle,
}					from '@spartan-hc/bundles';
import {
    DnaHubCSRZomelet,
    ZomeHubCSRZomelet,
    MereMemoryZomelet,

    DnaHubCell,
    ZomeHubCell,
}					from '@holochain/dna-hub-zomelets';
import {
    AppEntry,
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

    //
    // Virtual functions
    //
    async save_app ( bytes ) {
	const bundle			= new Bundle( bytes, "happ" );
	const resources			= {};

	for ( let [ rpath, dna_bytes ] of Object.entries( bundle.resources ) ) {
	    this.log.info("Save WASM resource '%s' (%s bytes)", rpath, dna_bytes.length );
	    resources[ rpath ]		= await this.cells.dna_hub.dna_hub_csr.save_dna( dna_bytes );
	}

	return await this.functions.create_app_entry({
	    "manifest": bundle.manifest,
	    resources,
	});
    },
}, {
    "cells": {
	"dna_hub": DnaHubCell,
    },
});


export const AppHubCell			= new CellZomelets({
    "app_hub_csr": AppHubCSRZomelet,
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
