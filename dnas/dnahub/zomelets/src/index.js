// 118kb = 114kb + this
import {
    AgentPubKey,
    ActionHash,
}					from '@spartan-hc/holo-hash'; // approx. 11kb
import {
    Zomelet,
    CellZomelets,
}					from '@spartan-hc/zomelets'; // approx. 7kb
import {
    Bundle,
}					from '@spartan-hc/bundles'; // approx. 39kb
import { // Relative import is causing duplicates (holo-hash, zomelets)
    ZomeHubCSRZomelet,
    MereMemoryZomelet,

    ZomeHubCell,
}					from '@holochain/zomehub-zomelets'; // approx. 57kb
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
    async get_dna_entries_for_agent ( input ) {
	const entries			= await this.call( input ? new AgentPubKey( input ) : input );

	return entries.map( entry => DnaEntry( entry ) );
    },

    //
    // Virtual functions
    //
    async save_dna ( bytes ) {
	const bundle			= new Bundle( bytes, "dna" );
	const zomes			= bundle.zomes();
	const resources			= {};

	for ( let wasm of zomes.integrity ) {
	    const rpath			= wasm.bundled;
	    this.log.info("Save integrity resource '%s' (%s bytes)", wasm.name, wasm.bytes.length );
	    resources[ rpath ]		= await this.cells.zomehub.zomehub_csr.save_integrity( wasm.bytes );
	}

	for ( let wasm of zomes.coordinator ) {
	    const rpath			= wasm.bundled;
	    this.log.info("Save integrity resource '%s' (%s bytes)", wasm.name, wasm.bytes.length );
	    resources[ rpath ]		= await this.cells.zomehub.zomehub_csr.save_coordinator( wasm.bytes );
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
