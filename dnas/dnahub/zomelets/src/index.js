// 118kb = 114kb + this
import {
    AgentPubKey,
    ActionHash, EntryHash,
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
    async create_dna_entry ( input ) {
	this.log.trace("DNA entry manifest input:", input.manifest );
	const result			= await this.call( input );

	return new EntryHash( result );
    },
    async get_dna_entry ( input ) {
	const result			= await this.call( new EntryHash( input ) );

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

	for ( let zome_manifest of bundle.manifest.integrity.zomes ) {
	    const rpath			= zome_manifest.bundled;
	    const wasm_bytes		= bundle.resources[ rpath ];

	    this.log.debug("Save integrity resource '%s' (%s bytes)", zome_manifest.name, wasm_bytes.length );
	    const wasm_addr		= await this.cells.zomehub.zomehub_csr.save_integrity( wasm_bytes );
	    this.log.info("Created new (integrity) Wasm entry: %s", wasm_addr );

	    zome_manifest.wasm_entry	= wasm_addr;
	    delete zome_manifest.bundled;
	}

	for ( let zome_manifest of bundle.manifest.coordinator.zomes ) {
	    const rpath			= zome_manifest.bundled;
	    const wasm_bytes		= bundle.resources[ rpath ];

	    this.log.debug("Save coordinator resource '%s' (%s bytes)", zome_manifest.name, wasm_bytes.length );
	    const wasm_entry		= await this.cells.zomehub.zomehub_csr.save_coordinator( wasm_bytes );
	    this.log.info("Created new (coordinator) Wasm entry: %s", wasm_entry );

	    zome_manifest.wasm_entry	= wasm_entry;
	    delete zome_manifest.bundled;
	}

	return await this.functions.create_dna_entry({
	    "manifest": bundle.manifest,
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
export *				from './types.js';

export default {
    DnaHubCSRZomelet,
    ZomeHubCSRZomelet,
    MereMemoryZomelet,

    // CellZomelets
    DnaHubCell,
    ZomeHubCell,
};
