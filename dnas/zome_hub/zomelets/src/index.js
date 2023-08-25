
import {
    AgentPubKey,
    ActionHash, EntryHash,
}					from '@spartan-hc/holo-hash';
import {
    Zomelet,
}					from '@spartan-hc/zomelets';
import { MereMemoryZomelet }		from '@spartan-hc/mere-memory-sdk';
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

    //
    // Virtual functions
    //
    async save_wasm ( bytes ) {
	const mere_memory_addr		= await this.peers.mere_memory_api.save( bytes );

	return await this.functions.create_wasm_entry({
	    mere_memory_addr,
	});
    },
}, {
    "peers": {
	"mere_memory_api": MereMemoryZomelet,
    },
});

export { MereMemoryZomelet }		from '@spartan-hc/mere-memory-sdk';

export default {
    ZomeHubCSRZomelet,
    MereMemoryZomelet,
};
