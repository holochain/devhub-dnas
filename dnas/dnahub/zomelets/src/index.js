// 118kb = 114kb + this
import {
    AnyDhtHash,
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
import {
    ZomeHubCSRZomelet,
    MereMemoryZomelet,

    ZomeHubCell,
}					from '@holochain/zomehub-zomelets'; // approx. 57kb
import {
    DnaEntry,
    Dna,
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
    async create_dna ( input ) {
	this.log.trace("DNA entry manifest input:", input.manifest );
	const result			= await this.call( input );

	return new Dna( result, this );
    },
    async create_dna_entry ( input ) {
	this.log.trace("DNA entry input:", input );
	const result			= await this.call( input );

	return new Dna( result, this );
    },
    "derive_dna_token":			true,
    "derive_integrities_token":		true,
    "derive_coordinators_token":	true,
    async get_dna_entry ( input ) {
	const result			= await this.call( new AnyDhtHash( input ) );

	return new Dna( result, this );
    },
    async get_dna_entries_for_agent ( input ) {
	const entries			= await this.call( input ? new AgentPubKey( input ) : input );

	return entries.map( entry => new Dna( entry ) );
    },
    async delete_dna ( input ) {
	return new ActionHash( await this.call( new ActionHash( input ) ) );
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
	    const wasm			= await this.cells.zomehub.zomehub_csr.save_integrity( wasm_bytes );
	    this.log.info("Created new (integrity) Wasm entry: %s", wasm.$addr );

	    zome_manifest.wasm_hrl	= {
		"dna": this.zome.cells.zomehub.dna,
		"target": wasm.$addr,
	    };

	    delete zome_manifest.bundled;
	}

	for ( let zome_manifest of bundle.manifest.coordinator.zomes ) {
	    const rpath			= zome_manifest.bundled;
	    const wasm_bytes		= bundle.resources[ rpath ];

	    this.log.debug("Save coordinator resource '%s' (%s bytes)", zome_manifest.name, wasm_bytes.length );
	    const wasm			= await this.cells.zomehub.zomehub_csr.save_coordinator( wasm_bytes );
	    this.log.info("Created new (coordinator) Wasm entry: %s", wasm.$addr );

	    zome_manifest.wasm_hrl	= {
		"dna": this.zome.cells.zomehub.dna,
		"target": wasm.$addr,
	    };

	    delete zome_manifest.bundled;
	}

	return await this.functions.create_dna({
	    "manifest": bundle.manifest,
	});
    },

    // Might require virtual cell dependency
    async get_integrity_wasm ( input ) {
	const dna_entry			= await this.functions.get_dna_entry( input.dna_entry );
	const zome_manifest		= dna_entry.manifest.integrity.zomes.find(
	    zome_manifest => zome_manifest.name === input.name
	);

	if ( !zome_manifest )
	    throw new Error(`DNA entry (${input.dna_entry}) does not have an integrity zome named '${input.name}'`);

	const wasm_hrl			= zome_manifest.wasm_hrl;
	const zomehub			= this.getCellInterface( "zomehub", wasm_hrl.dna );

	return await zomehub.zomehub_csr.get_wasm( wasm_hrl.target );
    },
    async get_coordinator_wasm ( input ) {
	const dna_entry			= await this.functions.get_dna_entry( input.dna_entry );
	const zome_manifest		= dna_entry.manifest.coordinator.zomes.find(
	    zome_manifest => zome_manifest.name === input.name
	);

	if ( !zome_manifest )
	    throw new Error(`DNA entry (${input.dna_entry}) does not have an coordinator zome named '${input.name}'`);

	const wasm_hrl			= zome_manifest.wasm_hrl;
	const zomehub			= this.getCellInterface( "zomehub", wasm_hrl.dna );

	return await zomehub.zomehub_csr.get_wasm( wasm_hrl.target );
    },
    async get_dna_bundle ( input ) {
	const dna_entry			= await this.functions.get_dna_entry( input );

	this.log.normal("Fetch assests for DNA manifest:", dna_entry.manifest );
	for ( let zome_manifest of dna_entry.manifest.integrity.zomes ) {
	    const wasm_hrl		= zome_manifest.wasm_hrl;
	    const zomehub		= this.getCellInterface( "zomehub", wasm_hrl.dna );

	    const wasm			= await zomehub.zomehub_csr.get_wasm( wasm_hrl.target );
	    zome_manifest.bytes		= wasm.bytes;

	    delete zome_manifest.wasm_hrl;
	}

	for ( let zome_manifest of dna_entry.manifest.coordinator.zomes ) {
	    const wasm_hrl		= zome_manifest.wasm_hrl;
	    const zomehub		= this.getCellInterface( "zomehub", wasm_hrl.dna );

	    const wasm			= await zomehub.zomehub_csr.get_wasm( wasm_hrl.target );
	    zome_manifest.bytes		= wasm.bytes;

	    delete zome_manifest.wasm_hrl;
	}

	const bundle			= Bundle.createDna( dna_entry.manifest );

	return bundle.toBytes();
    },
}, {
    "cells": {
	"zomehub": ZomeHubCell,
    },
    // Virtual cells don't require ?
    "virtual": {
	"cells": {
	    "zomehub": ZomeHubCell,
	},
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
