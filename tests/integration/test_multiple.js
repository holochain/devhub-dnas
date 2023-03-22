const path				= require('path');
const log				= require('@whi/stdlog')(path.basename( __filename ), {
    level: process.env.LOG_LEVEL || 'fatal',
});


const fs				= require('fs');
const crypto				= require('crypto');
const expect				= require('chai').expect;
const YAML				= require('yaml');
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

	let a_agent_info		= await alice.call( "web_assets", "web_assets", "whoami", null);
	log.info("Agent ID 'alice': %s", String(new HoloHash(a_agent_info.agent_initial_pubkey)) );
    });


    it("should assemble hApp bundle", async function () {
	this.timeout( 120_000 );

	const alice			= clients.alice;

	let zome_input			= {
	    "name": "file_storage",
	    "zome_type": 0,
	    "display_name": "File Storage",
	    "description": "A generic API for fs-like data management",
	};
	let zome_1			= await alice.call( "dnarepo", "dna_library", "create_zome", zome_input );
	log.normal("New ZOME (metadata): %s -> %s", String(zome_1.$id), zome_1.name );

	log.debug("ZOME file bytes (%s): typeof %s", zome_bytes.length, typeof zome_bytes );
	let zome_version_1		= await alice.call( "dnarepo", "dna_library", "create_zome_version", {
	    "for_zome": zome_1.$id,
	    "version": "v0.1.0",
	    "ordering": 1,
	    "zome_bytes": zome_bytes,
	    "hdk_version": "v0.0.120",
	});
	log.normal("New ZOME version: %s -> %s", String(zome_version_1.$address), zome_version_1.version );


	const dna_input			= {
	    "name": "game_turns",
	    "display_name": "Game Turns",
	    "description": "A tool for turn-based games to track the order of player actions",
	};
	let dna				= await alice.call( "dnarepo", "dna_library", "create_dna", dna_input );
	log.normal("New DNA (metadata): %s -> %s", String(dna.$id), dna.name );

	let version			= await alice.call( "dnarepo", "dna_library", "create_dna_version", {
	    "for_dna": dna.$id,
	    "version": "v0.1.0",
	    "ordering": 1,
	    "hdk_version": "v0.0.120",
	    "integrity_zomes": [{
		"name": "mere_memory_api",
		"zome": new EntryHash( zome_version_1.for_zome ),
		"version": zome_version_1.$id,
		"resource": new EntryHash( zome_version_1.mere_memory_addr ),
		"resource_hash": zome_version_1.mere_memory_hash,
	    }],
	    "zomes": [],
	    "origin_time": "2022-02-11T23:05:19.470323Z",
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

	const yaml_text			= fs.readFileSync( path.resolve(__dirname, "../../bundled/happ.yaml"), "utf-8" );
	const manifest			= YAML.parse( yaml_text );

	manifest.roles			= manifest.roles.slice( 0, 1 );
	manifest.roles[0].name		= "test_dna";

	let release_input		= {
	    "name": "v0.1.0",
	    "description": "The first release",
	    "for_happ": happ.$id,
	    "ordering": 1,
	    "manifest": manifest,
	    "hdk_version": "v0.0.120",
	    "dnas": [
		{
		    "role_name": "test_dna",
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


	const file_bytes		= fs.readFileSync( path.resolve(__dirname, "../test.zip") );
	log.debug("Zip file bytes (%s): typeof %s", file_bytes.length, typeof file_bytes );

	let webasset_addr;
	{
	    let file			= await alice.call( "web_assets", "web_assets", "create_file", {
		"file_bytes": file_bytes,
	    });
	    log.normal("New webasset file: %s -> %s", String(file.$address), file.version );
	    webasset_addr		= file.$address;
	}

	{
	    let webasset		= await alice.call( "happs", "happ_library", "get_webasset", {
		"id": webasset_addr,
	    });
	    log.normal("Updated hApp UI: %s", webasset.file_size );

	    expect( webasset.file_size	).to.be.a("number");
	    expect( webasset.file_size	).to.be.gt( 0 );
	}


	let gui_input			= {
	    "name": "Gecko",
	    "description": "Web UI for Chess",
	};

	let gui				= gui_1 = await alice.call( "happs", "happ_library", "create_gui", gui_input );
	log.normal("New Gui release: %s -> %s", String(gui.$addr), gui.name );

	expect( gui.description	).to.equal( gui_input.description );

	let gui_release_input		= {
	    "version": "Gecko",
	    "changelog": "Web UI for Chess",
	    "for_gui": gui.$id,
	    "for_happ_releases": [ release.$id ],
	    "web_asset_id": webasset_addr,
	};

	let gui_release		= gui_release_1 = await alice.call( "happs", "happ_library", "create_gui_release", gui_release_input );
	log.normal("New Gui release: %s -> %s", String(gui_release.$addr), gui_release.name );

	expect( gui_release.description	).to.equal( gui_release_input.description );

	{
	    let gui_package		= await alice.call( "happs", "happ_library", "get_webhapp_package", {
		"name": "Test Web hApp Package",
		"happ_release_id": release.$id,
		"gui_release_id": gui_release.$id,
	    });
	    log.normal("Web hApp package bytes: (%s) %s", gui_package.constructor.name, gui_package.length );

	    expect( gui_package.constructor.name	).to.equal("Array");

	    fs.writeFileSync( path.resolve(__dirname, "../multitesting.webhapp"), Buffer.from(gui_package) );
	}

	release				= await alice.call( "happs", "happ_library", "update_happ_release", {
	    "addr": release.$action,
	    "properties": {
		"official_gui": gui_release.$id,
	    },
	});
	log.normal("Updated hApp release: %s -> %s", String(release.$addr), release.name );

	expect( release.official_gui	).to.deep.equal( gui_release.$id.bytes() );
    });
}


function errors_tests () {
}

describe("All DNAs", () => {

    const holochain			= new Holochain({
	"default_stdout_loggers": process.env.LOG_LEVEL === "silly",
    });

    before(async function () {
	this.timeout( 60_000 );

	clients				= await backdrop( holochain, {
	    "dnarepo": DNAREPO_PATH,
	    "happs": HAPPS_PATH,
	    "web_assets": WEBASSETS_PATH,
	}, [
	    "alice",
	]);

	// Must call whoami on each cell to ensure that init has finished.
	{
	    let whoami			= await clients.alice.call( "dnarepo", "dna_library", "whoami", null, 30_000 );
	    log.normal("Alice whoami: %s", String(new HoloHash( whoami.agent_initial_pubkey )) );
	}
	{
	    let whoami			= await clients.alice.call( "happs", "happ_library", "whoami", null, 30_000 );
	    log.normal("Alice whoami: %s", String(new HoloHash( whoami.agent_initial_pubkey )) );
	}
	{
	    let whoami			= await clients.alice.call( "web_assets", "web_assets", "whoami", null, 30_000 );
	    log.normal("Alice whoami: %s", String(new HoloHash( whoami.agent_initial_pubkey )) );
	}
    });

    describe("Basic", basic_tests.bind( this, holochain ) );
    describe("Errors", errors_tests.bind( this, holochain ) );

    after(async () => {
	await holochain.destroy();
    });

});
