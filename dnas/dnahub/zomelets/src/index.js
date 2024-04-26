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
    DnaAsset,
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
    async get_dna_asset ( input ) {
	const result			= await this.call( new EntryHash( input ) );
	const dna_asset			= DnaAsset( result );
	const manifest			= dna_asset.dna_entry.manifest;

	for ( let zome_manifest of manifest.integrity.zomes ) {
	    delete zome_manifest.zome_hrl;

	    const compressed_bytes	= new Uint8Array(
		dna_asset.zome_assets[ zome_manifest.name ].bytes
	    );

	    zome_manifest.bytes		= await this.cells.zomehub.mere_memory_api.gzip_uncompress(
		compressed_bytes
	    );

	    // Verify asset hash
	    const hash			= await this.cells.zomehub.mere_memory_api.calculate_hash( zome_manifest.bytes );
	    const expected_hash		= dna_asset.dna_entry.asset_hashes.integrity[ zome_manifest.name ];

	    if ( hash !== expected_hash )
		throw new Error(`Asset hash for integrity zome '${zome_manifest.name}' is invalid; ${hash} !== ${expected_hash} (expected)`);
	}

	for ( let zome_manifest of manifest.coordinator.zomes ) {
	    delete zome_manifest.zome_hrl;

	    const compressed_bytes	= new Uint8Array(
		dna_asset.zome_assets[ zome_manifest.name ].bytes
	    );

	    zome_manifest.bytes		= await this.cells.zomehub.mere_memory_api.gzip_uncompress(
		compressed_bytes
	    );

	    // Verify asset hash
	    const hash			= await this.cells.zomehub.mere_memory_api.calculate_hash( zome_manifest.bytes );
	    const expected_hash		= dna_asset.dna_entry.asset_hashes.coordinator[ zome_manifest.name ];

	    if ( hash !== expected_hash )
		throw new Error(`Asset hash for coordinator zome '${zome_manifest.name}' is invalid; ${hash} !== ${expected_hash} (expected)`);
	}

	return dna_asset;
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
	const claimed_file_size		= bytes.length;
	const dna_asset_hashes		= {
	    "integrity": {},
	    "coordinator": {},
	};
	const bundle			= new Bundle( bytes, "dna" );

	for ( let zome_manifest of bundle.manifest.integrity.zomes ) {
	    const rpath			= zome_manifest.bundled;
	    const zome_bytes		= bundle.resources[ rpath ];

	    this.log.debug("Save integrity resource '%s' (%s bytes)", zome_manifest.name, zome_bytes.length );
	    const zome			= await this.cells.zomehub.zomehub_csr.save_integrity( zome_bytes );
	    this.log.info("Created new (integrity) Zome entry: %s", zome.$addr );

	    zome_manifest.zome_hrl	= {
		"dna": this.zome.cells.zomehub.dna,
		"target": zome.$addr,
	    };

	    dna_asset_hashes.integrity[ zome_manifest.name ] = zome.hash;

	    delete zome_manifest.bundled;
	}

	for ( let zome_manifest of bundle.manifest.coordinator.zomes ) {
	    const rpath			= zome_manifest.bundled;
	    const zome_bytes		= bundle.resources[ rpath ];

	    this.log.debug("Save coordinator resource '%s' (%s bytes)", zome_manifest.name, zome_bytes.length );
	    const zome			= await this.cells.zomehub.zomehub_csr.save_coordinator( zome_bytes );
	    this.log.info("Created new (coordinator) Zome entry: %s", zome.$addr );

	    zome_manifest.zome_hrl	= {
		"dna": this.zome.cells.zomehub.dna,
		"target": zome.$addr,
	    };

	    dna_asset_hashes.coordinator[ zome_manifest.name ] = zome.hash;

	    delete zome_manifest.bundled;
	}

	return await this.functions.create_dna({
	    "manifest": bundle.manifest,
	    claimed_file_size,
	    "asset_hashes": dna_asset_hashes,
	});
    },

    // Might require virtual cell dependency
    async get_integrity_zome ( input ) {
	const dna_entry			= await this.functions.get_dna_entry( input.dna_entry );
	const zome_manifest		= dna_entry.manifest.integrity.zomes.find(
	    zome_manifest => zome_manifest.name === input.name
	);

	if ( !zome_manifest )
	    throw new Error(`DNA entry (${input.dna_entry}) does not have an integrity zome named '${input.name}'`);

	const zome_hrl			= zome_manifest.zome_hrl;
	const zomehub			= this.getCellInterface( "zomehub", zome_hrl.dna );

	return await zomehub.zomehub_csr.get_zome( zome_hrl.target );
    },
    async get_coordinator_zome ( input ) {
	const dna_entry			= await this.functions.get_dna_entry( input.dna_entry );
	const zome_manifest		= dna_entry.manifest.coordinator.zomes.find(
	    zome_manifest => zome_manifest.name === input.name
	);

	if ( !zome_manifest )
	    throw new Error(`DNA entry (${input.dna_entry}) does not have an coordinator zome named '${input.name}'`);

	const zome_hrl			= zome_manifest.zome_hrl;
	const zomehub			= this.getCellInterface( "zomehub", zome_hrl.dna );

	return await zomehub.zomehub_csr.get_zome( zome_hrl.target );
    },
    async get_dna_bundle ( input ) {
	const dna_entry			= await this.functions.get_dna_entry( input );

	this.log.normal("Fetch assests for DNA manifest:", dna_entry.manifest );
	for ( let zome_manifest of dna_entry.manifest.integrity.zomes ) {
	    const zome_hrl		= zome_manifest.zome_hrl;
	    const zomehub		= this.getCellInterface( "zomehub", zome_hrl.dna );

	    const zome			= await zomehub.zomehub_csr.get_zome( zome_hrl.target );
	    zome_manifest.bytes		= zome.bytes;

	    delete zome_manifest.zome_hrl;
	}

	for ( let zome_manifest of dna_entry.manifest.coordinator.zomes ) {
	    const zome_hrl		= zome_manifest.zome_hrl;
	    const zomehub		= this.getCellInterface( "zomehub", zome_hrl.dna );

	    const zome			= await zomehub.zomehub_csr.get_zome( zome_hrl.target );
	    zome_manifest.bytes		= zome.bytes;

	    delete zome_manifest.zome_hrl;
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
