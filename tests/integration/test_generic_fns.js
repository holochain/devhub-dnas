const path				= require('path');
const log				= require('@whi/stdlog')(path.basename( __filename ), {
    level: process.env.LOG_LEVEL || 'fatal',
});


const fs				= require('fs');
const crypto				= require('crypto');
const expect				= require('chai').expect;
const Identicon				= require('identicon.js');
const msgpack				= require('@msgpack/msgpack');
const { EntryHash,
	HoloHash }			= require('@whi/holo-hash');
const { Holochain }			= require('@whi/holochain-backdrop');
const json				= require('@whi/json');
const why				= require('why-is-node-running');

// setTimeout(() => {
//     console.log( why() );
// }, 6000 );

const { backdrop }			= require('./setup.js');

const delay				= (n) => new Promise(f => setTimeout(f, n));
const DNAREPO_PATH			= path.join( __dirname, "../../bundled/dnarepo.dna" );

let clients;
let zome_version_1;
let dna_1;

function basic_tests () {
    const zome_bytes			= fs.readFileSync( path.resolve(__dirname, "../../zomes/mere_memory.wasm") );

    it("should CRUD Zome and ZomeVersion", async function () {
	this.timeout( 30_000 );

	const alice			= clients.alice;

	let zome_input			= {
	    "name": "file_storage",
	    "display_name": "File Storage",
	    "description": "A generic API for fs-like data management",
	    "tags": [ "Storage", "General-use" ],
	};

	let zome			= await alice.call( "dnarepo", "dna_library", "create_zome", zome_input );;
	log.normal("New ZOME (metadata): %s -> %s", String(zome.$id), zome.name );

	{
	    log.debug("ZOME file bytes (%s): typeof %s", zome_bytes.length, typeof zome_bytes );
	    let version			= await alice.call( "dnarepo", "dna_library", "create_zome_version", {
		"for_zome": zome.$id,
		"version": "v0.1.0",
		"ordering": 1,
		"zome_bytes": zome_bytes,
		"hdk_version": "v0.0.120",
	    });
	    log.normal("New ZOME version: %s -> %s", String(version.$address), version.version );

	    zome_version_1		= version;
	}

	{
	    // Make another zome version for list tests
	    await alice.call( "dnarepo", "dna_library", "create_zome_version", {
		"for_zome": zome.$id,
		"version": "v0.2.0",
		"ordering": 2,
		"zome_bytes": [ 1, 2, 3 ],
		"hdk_version": "v0.0.120",
	    });
	}
    });

    it("should CRUD Dna and DnaVersion", async function () {
	this.timeout( 30_000 );

	const alice			= clients.alice;

	let dna_input			= {
	    "name": "game_turns",
	    "display_name": "Game Turns",
	    "description": "A tool for turn-based games to track the order of player actions",
	    "tags": [ "Games", "Turn-based" ],
	    "metadata": {
		"color": "blue",
	    },
	};

	let dna				= dna_1 = await alice.call( "dnarepo", "dna_library", "create_dna", dna_input );
	log.normal("New DNA (metadata): %s -> %s", String(dna.$id), dna.name );

	{
	    // Update DNA
	    const dna_name		= "game_turns_new";
	    const tags			= [ "Games", "Turns" ];
	    dna				= await alice.call( "dnarepo", "dna_library", "update_dna", {
		"addr": dna.$action,
		"properties": {
		    "name": dna_name,
		    tags,
		}
	    });
	}

	{
	    let version			= await alice.call( "dnarepo", "dna_library", "create_dna_version", {
		"for_dna": dna.$id,
		"version": "v0.1.0",
		"ordering": 1,
		"hdk_version": "v0.0.120",
		"integrity_zomes": [{
		    "name": "mere_memory",
		    "zome": new EntryHash( zome_version_1.for_zome ),
		    "version": zome_version_1.$id,
		    "resource": new EntryHash( zome_version_1.mere_memory_addr ),
		    "resource_hash": zome_version_1.mere_memory_hash,
		}],
		"zomes": [],
		"origin_time": "2022-02-11T23:05:19.470323Z",
	    });
	    log.normal("New DNA version: %s -> %s", String(version.$address), version.version );
	}
    });

    it("should test exposed base functions", async function () {
	{
	    let record			= await clients.alice.call( "dnarepo", "dna_library", "get_record", dna_1.$id );
	    let entry			= msgpack.decode( record.entry.Present.entry );
	    expect( entry.name		).to.be.a("string");
	}

	let record			= await clients.alice.call( "dnarepo", "dna_library", "get_record_latest", dna_1.$id );
	let dna				= msgpack.decode( record.entry.Present.entry );
	expect( dna.name		).to.not.equal( dna_1.name );

	// {
	//     let links			= await clients.alice.call( "dnarepo", "dna_library", "get_links", {
	// 	"base":	dna_1.$id,
	// 	"tag":	"dna_version",
	//     });
	//     expect( links		).to.have.length( 1 );
	// }

	// {
	//     let path_id			= await clients.alice.call( "dnarepo", "dna_library", "path", [ "dnas" ] );
	//     let links			= await clients.alice.call( "dnarepo", "dna_library", "get_links", {
	// 	"base":	path_id,
	// 	"tag":	"dna",
	//     });
	//     expect( links		).to.have.length( 1 );
	// }

	// {
	//     let path_id			= await clients.alice.call( "dnarepo", "dna_library", "path", [ "filter_by", "name", dna.name ] );
	//     let links			= await clients.alice.call( "dnarepo", "dna_library", "get_links", {
	// 	"base":	path_id,
	// 	"tag":	"dna",
	//     });
	//     expect( links		).to.have.length( 1 );
	// }
    });
}

function errors_tests () {
}

describe("Generic", () => {

    const holochain			= new Holochain({
	"default_stdout_loggers": process.env.LOG_LEVEL === "silly",
    });

    before(async function () {
	this.timeout( 30_000 );

	clients				= await backdrop( holochain, {
	    "dnarepo": DNAREPO_PATH,
	}, [
	    "alice",
	]);

	// Must call whoami on each cell to ensure that init has finished.
	{
	    let whoami			= await clients.alice.call( "dnarepo", "dna_library", "whoami" );
	    log.normal("Alice whoami: %s", String(new HoloHash( whoami.agent_initial_pubkey )) );
	}
    });

    describe("Basic", basic_tests.bind( this, holochain ) );
    describe("Errors", errors_tests.bind( this, holochain ) );

    after(async () => {
	await holochain.destroy();
    });

});
