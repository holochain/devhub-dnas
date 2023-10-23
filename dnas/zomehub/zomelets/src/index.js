import {
    AgentPubKey,
    ActionHash, EntryHash,
}					from '@spartan-hc/holo-hash'; // approx. 11kb
import {
    Zomelet,
    CellZomelets,
}					from '@spartan-hc/zomelets'; // approx. 7kb
import { MereMemoryZomelet }		from '@spartan-hc/mere-memory-zomelets'; // approx. 33kb
import {
    WasmEntry,
}					from './types.js';

export const ZomeHubCSRZomelet		= new Zomelet({
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
    async create_wasm_entry ( input ) {
	const result			= await this.call({
	    "mere_memory_addr": new EntryHash( input.mere_memory_addr ),
	});

	return new ActionHash( result );
    },
    async get_wasm_entry ( input ) {
	const result			= await this.call( new ActionHash( input ) );

	return WasmEntry( result );
    },
    async get_wasm_entries_for_agent ( input ) {
	const entries			= await this.call(); // new AgentPubKey( input )

	return entries.map( entry => WasmEntry( entry ) );
    },

    //
    // Virtual functions
    //
    async save_wasm ( bytes ) {
	const addr			= await this.zomes.mere_memory_api.save( bytes, {
	    "compress": true,
	});

	return await this.functions.create_wasm_entry({
	    "mere_memory_addr": addr,
	});
    },
}, {
    "zomes": {
	"mere_memory_api": MereMemoryZomelet,
    },
});


export const ZomeHubCell		= new CellZomelets({
    "zomehub_csr": ZomeHubCSRZomelet,
    "mere_memory_api": MereMemoryZomelet,
});


export { MereMemoryZomelet }		from '@spartan-hc/mere-memory-zomelets';

export default {
    // Zomelets
    ZomeHubCSRZomelet,
    MereMemoryZomelet,

    // CellZomelets
    ZomeHubCell,
};
