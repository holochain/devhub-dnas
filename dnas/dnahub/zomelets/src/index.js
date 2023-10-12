
import {
    AgentPubKey,
    ActionHash,
}					from '@spartan-hc/holo-hash';
import {
    Zomelet,
    CellZomelets,
}					from '@spartan-hc/zomelets';
import {
    Bundle,
}					from '@spartan-hc/bundles';
import {
    ZomeHubCSRZomelet,
    MereMemoryZomelet,

    ZomeHubCell,
}					from '@holochain/zomehub-zomelets';
import {
    DnaEntry,
}					from './types.js';

export const DnaHubCSRZomelet		= new Zomelet({
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
    async create_dna_entry ({ manifest, resources }) {
	this.log.info("DNA entry input (%s resources):", Object.keys(resources).length, manifest );
	const result			= await this.call({
	    "manifest": manifest,
	    "resources": resources,
	});

	return new ActionHash( result );
    },
    async get_dna_entry ( input ) {
	const result			= await this.call( new ActionHash( input ) );

	return DnaEntry( result );
    },

    //
    // Virtual functions
    //
    async save_dna ( bytes ) {
	const bundle			= new Bundle( bytes, "dna" );
	const resources			= {};

	for ( let [ rpath, zome_bytes ] of Object.entries( bundle.resources ) ) {
	    this.log.info("Save WASM resource '%s' (%s bytes)", rpath, zome_bytes.length );
	    resources[ rpath ]		= await this.cells.zomehub.zomehub_csr.save_wasm( zome_bytes );
	}

	return await this.functions.create_dna_entry({
	    "manifest": bundle.manifest,
	    resources,
	});
    },
}, {
    "cells": {
	"zomehub": ZomeHubCell,
    },
});


export const DnaHubCell			= new CellZomelets({
    "dnahub_csr": DnaHubCSRZomelet,
});


export  {
    ZomeHubCSRZomelet,
    MereMemoryZomelet,
    ZomeHubCell,
}					from '@holochain/zomehub-zomelets';

export default {
    DnaHubCSRZomelet,
    ZomeHubCSRZomelet,
    MereMemoryZomelet,

    // CellZomelets
    DnaHubCell,
    ZomeHubCell,
};
