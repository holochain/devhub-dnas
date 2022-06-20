const path				= require('path');
const log				= require('@whi/stdlog')(path.basename( __filename ), {
    level: process.env.LOG_LEVEL || 'fatal',
});


const fs				= require('fs');
const crypto				= require('crypto');
const expect				= require('chai').expect;
const { EntryHash,
	HoloHash }			= require('@whi/holo-hash');
const { Holochain }			= require('@whi/holochain-backdrop');
const json				= require('@whi/json');

const { backdrop }			= require('./setup.js');
const delay				= (n) => new Promise(f => setTimeout(f, n));


const DNAREPO_PATH			= path.join(__dirname, "../../bundled/dnarepo.dna");
const HAPPS_PATH			= path.join(__dirname, "../../bundled/happs.dna");
const WEBASSETS_PATH			= path.join(__dirname, "../../bundled/web_assets.dna");

const chunk_size			= (2**20 /*1 megabyte*/) * 2;

let clients;


function basic_tests () {
    const zome_bytes			= fs.readFileSync( path.resolve(__dirname, "../../zomes/mere_memory.wasm") );

    it("should get whoami info", async function () {
	const alice			= clients.alice;

	let a_agent_info		= await alice.call( "webassets", "web_assets", "whoami", null);
	log.info("Agent ID 'alice': %s", String(new HoloHash(a_agent_info.agent_initial_pubkey)) );
    });


    it("should assemble hApp bundle", async function () {
	this.timeout( 30_000 );

	const alice			= clients.alice;

	const file_bytes		= fs.readFileSync( path.resolve(__dirname, "../test.zip") );
	log.debug("Zip file bytes (%s): typeof %s", file_bytes.length, typeof file_bytes );

	let file_addr;
	{
	    let file			= await alice.call( "webassets", "web_assets", "create_file", {
		"file_bytes": file_bytes,
	    });
	    log.normal("New webasset file: %s -> %s", String(file.$address), file.version );
	    file_addr			= file.$address;
	}

	{
	    let gui			= await alice.call( "happs", "happ_library", "get_gui", {
		// "dna_hash": alice._app_schema._dnas.webassets._hash,
		"id": file_addr,
	    });
	    log.normal("Updated hApp UI: %s", gui.file_size );

	    expect( gui.file_size	).to.be.a("number");
	    expect( gui.file_size	).to.be.gt( 0 );
	}


	let zome_input			= {
	    "name": "File Storage",
	    "description": "A generic API for fs-like data management",
	};
	let zome_1			= await alice.call( "dnarepo", "dna_library", "create_zome", zome_input );
	log.normal("New ZOME (metadata): %s -> %s", String(zome_1.$id), zome_1.name );

	log.debug("ZOME file bytes (%s): typeof %s", zome_bytes.length, typeof zome_bytes );
	let zome_version_1		= await alice.call( "dnarepo", "dna_library", "create_zome_version", {
	    "for_zome": zome_1.$id,
	    "version": 1,
	    "zome_bytes": zome_bytes,
	    "hdk_version": "v0.0.120",
	});
	log.normal("New ZOME version: %s -> %s", String(zome_version_1.$address), zome_version_1.version );


	const dna_input			= {
	    "name": "Game Turns",
	    "description": "A tool for turn-based games to track the order of player actions",
	};
	let dna				= await alice.call( "dnarepo", "dna_library", "create_dna", dna_input );
	log.normal("New DNA (metadata): %s -> %s", String(dna.$id), dna.name );

	const dna_bytes			= fs.readFileSync( path.resolve(__dirname, "../test.dna") );
	log.debug("DNA file bytes (%s): typeof %s", dna_bytes.length, typeof dna_bytes );

	let version			= await alice.call( "dnarepo", "dna_library", "create_dna_version", {
	    "for_dna": dna.$id,
	    "version": 1,
	    "hdk_version": "v0.0.120",
	    "zomes": [{
		"name": "mere_memory",
		"zome": new EntryHash( zome_version_1.for_zome ),
		"version": zome_version_1.$id,
		"resource": new EntryHash( zome_version_1.mere_memory_addr ),
		"resource_hash": zome_version_1.mere_memory_hash,
	    }],
	});
	log.normal("New DNA version: %s -> %s", String(version.$address), version.version );


	let happ_input			= {
	    "title": "Chess",
	    "subtitle": "Super fun board game",
	    "description": "Play chess with friends :)",
	};

	let happ			= await alice.call( "happs", "happ_library", "create_happ", happ_input );
	log.normal("New hApp: %s", String(happ.$addr) );

	expect( happ.description	).to.equal( happ_input.description );


	const manifest_yaml		= fs.readFileSync( path.resolve(__dirname, "../test_happ.yaml"), "utf8" );
	let release_input		= {
	    "name": "v0.1.0",
	    "description": "The first release",
	    "for_happ": happ.$id,
	    "gui": {
		"asset_group_id": file_addr,
		"uses_web_sdk": false,
	    },
	    "manifest": {
		"manifest_version": "1",
		"roles": [
		    {
			"id": "test_dna",
			"dna": {
			    "path": "./this/does/not/matter.dna",
			},
			"clone_limit": 0,
		    },
		],
	    },
	    "hdk_version": "v0.0.120",
	    "dnas": [
		{
		    "role_id": "test_dna",
		    "dna": dna.$id,
		    "version": version.$id,
		    "wasm_hash": version.wasm_hash,
		}
	    ],
	};

	let release			= await alice.call( "happs", "happ_library", "create_happ_release", release_input );
	log.normal("New hApp release: %s -> %s", String(release.$addr), release.name );

	expect( release.description	).to.equal( release_input.description );


	{
	    let happ_package		= await alice.call( "happs", "happ_library", "get_release_package", {
		"id": release.$id,
	    });
	    log.normal("hApp release package bytes: (%s) %s", happ_package.constructor.name, happ_package.length );

	    expect( happ_package.constructor.name	).to.equal("Array");

	    fs.writeFileSync( path.resolve(__dirname, "../multitesting.happ"), Buffer.from(happ_package) );
	}

	{
	    let webhapp_package		= await alice.call( "happs", "happ_library", "get_webhapp_package", {
		"name": "Test Web hApp Package",
		"id": release.$id,
	    });
	    log.normal("Web hApp package bytes: (%s) %s", webhapp_package.constructor.name, webhapp_package.length );

	    expect( webhapp_package.constructor.name	).to.equal("Array");

	    fs.writeFileSync( path.resolve(__dirname, "../multitesting.webhapp"), Buffer.from(webhapp_package) );
	}
    });
}


function errors_tests () {
}

describe("All DNAs", () => {

    const holochain			= new Holochain({
	"default_stdout_loggers": process.env.LOG_LEVEL === "silly",
    });

    before(async function () {
	this.timeout( 30_000 );

	clients				= await backdrop( holochain, {
	    "dnarepo": DNAREPO_PATH,
	    "happs": HAPPS_PATH,
	    "webassets": WEBASSETS_PATH,
	}, [
	    "alice",
	]);

	{
	    let dna_info		= await clients.alice.call( "dnarepo", "dna_library", "dna_info" );
	    log.info("Alice dnarepo dna_info zomes: %s", dna_info.zome_names );
	}
	{
	    let dna_info		= await clients.alice.call( "happs", "happ_library", "dna_info" );
	    log.info("Alice happs dna_info zomes: %s", dna_info.zome_names );
	}
	{
	    let dna_info		= await clients.alice.call( "webassets", "web_assets", "dna_info" );
	    log.info("Alice webassets dna_info zomes: %s", dna_info.zome_names );
	}

	// Must call whoami on each cell to ensure that init has finished.
	{
	    let whoami			= await clients.alice.call( "dnarepo", "dna_library", "whoami" );
	    log.normal("Alice whoami: %s", String(new HoloHash( whoami.agent_initial_pubkey )) );
	}
	{
	    let whoami			= await clients.alice.call( "happs", "happ_library", "whoami" );
	    log.normal("Alice whoami: %s", String(new HoloHash( whoami.agent_initial_pubkey )) );
	}
	{
	    let whoami			= await clients.alice.call( "webassets", "web_assets", "whoami" );
	    log.normal("Alice whoami: %s", String(new HoloHash( whoami.agent_initial_pubkey )) );
	}
    });

    describe("Basic", basic_tests.bind( this, holochain ) );
    describe("Errors", errors_tests.bind( this, holochain ) );

    after(async () => {
	await holochain.destroy();
    });

});
