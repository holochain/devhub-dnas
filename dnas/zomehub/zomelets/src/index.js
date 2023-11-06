import {
    AnyDhtHash,
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
    Wasm,
}					from './types.js';


export const WASM_TYPES			= {
    "INTEGRITY": "integrity",
    "COORDINATOR": "coordinator",
};
export const WASM_TYPE_NAMES		= Object.values( WASM_TYPES );


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
	const result			= await this.call( input );

	return new Wasm( result, this );
    },
    async create_wasm ( input ) {
	if ( !WASM_TYPE_NAMES.includes( input.wasm_type ) )
	    throw new TypeError(`Invalid 'wasm_type' input '${input.wasm_type}'; expected ${WASM_TYPE_NAMES.join(", ")}`);

	input.mere_memory_addr		= new EntryHash( input.mere_memory_addr );

	const result			= await this.call( input );

	return new Wasm( result, this );
    },
    async get_wasm_entry ( input ) {
	const result			= await this.call( new AnyDhtHash( input ) );

	return new Wasm( result, this );
    },
    async get_wasm_entries_for_agent ( input ) {
	const entries			= await this.call( input ? new AgentPubKey( input ) : input );

	return entries.map( entry => new Wasm( entry, this ) );
    },
    async delete_wasm ( input ) {
	return new ActionHash( await this.call( new ActionHash( input ) ) );
    },


    //
    // Virtual functions
    //
    async save_integrity ( bytes ) {
	const addr			= await this.zomes.mere_memory_api.save( bytes );

	return await this.functions.create_wasm({
	    "wasm_type": WASM_TYPES.INTEGRITY,
	    "mere_memory_addr": addr,
	});
    },
    async save_coordinator ( bytes ) {
	const addr			= await this.zomes.mere_memory_api.save( bytes );

	return await this.functions.create_wasm({
	    "wasm_type": WASM_TYPES.COORDINATOR,
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
export *				from './types.js';

export default {
    WASM_TYPES,
    WASM_TYPE_NAMES,

    // Zomelets
    ZomeHubCSRZomelet,
    MereMemoryZomelet,

    // CellZomelets
    ZomeHubCell,
};
