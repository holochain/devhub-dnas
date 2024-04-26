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
    Zome,
    ZomeAsset,
}					from './types.js';


export const ZOME_TYPES			= {
    "INTEGRITY": "integrity",
    "COORDINATOR": "coordinator",
};
export const ZOME_TYPE_NAMES		= Object.values( ZOME_TYPES );


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
    async create_zome_entry ( input ) {
	const result			= await this.call( input );

	return new Zome( result, this );
    },
    async create_zome ( input ) {
	if ( !ZOME_TYPE_NAMES.includes( input.zome_type ) )
	    throw new TypeError(`Invalid 'zome_type' input '${input.zome_type}'; expected ${ZOME_TYPE_NAMES.join(", ")}`);

	input.mere_memory_addr		= new EntryHash( input.mere_memory_addr );

	const result			= await this.call( input );

	return new Zome( result, this );
    },
    async get_zome_entry ( input ) {
	const result			= await this.call( new AnyDhtHash( input ) );

	return new Zome( result, this );
    },
    async get_zome_asset ( input ) {
	const result			= await this.call( new EntryHash( input ) );

	return ZomeAsset( result );
    },
    async get_zome_entries_for_agent ( input ) {
	const entries			= await this.call( input ? new AgentPubKey( input ) : input );

	return entries.map( entry => new Zome( entry, this ) );
    },
    async delete_zome ( input ) {
	return new ActionHash( await this.call( new ActionHash( input ) ) );
    },


    //
    // Virtual functions
    //
    async save_integrity ( bytes ) {
	const addr			= await this.zomes.mere_memory_api.save( bytes );

	return await this.functions.create_zome({
	    "zome_type": ZOME_TYPES.INTEGRITY,
	    "mere_memory_addr": addr,
	});
    },
    async save_coordinator ( bytes ) {
	const addr			= await this.zomes.mere_memory_api.save( bytes );

	return await this.functions.create_zome({
	    "zome_type": ZOME_TYPES.COORDINATOR,
	    "mere_memory_addr": addr,
	});
    },
    async get_zome ( input ) {
	const zome_entry		= await this.functions.get_zome_entry( input );

	zome_entry.bytes		= await this.zomes.mere_memory_api.remember(
	    zome_entry.mere_memory_addr
	);

	return zome_entry;
    },
    async get_zome_entry_memory ( input ) {
	const zome_entry		= await this.functions.get_zome_entry( input );

	return await this.zomes.mere_memory_api.remember( zome_entry.mere_memory_addr );
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
    ZOME_TYPES,
    ZOME_TYPE_NAMES,

    // Zomelets
    ZomeHubCSRZomelet,
    MereMemoryZomelet,

    // CellZomelets
    ZomeHubCell,
};
