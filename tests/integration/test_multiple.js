const path				= require('path');
const log				= require('@whi/stdlog')(path.basename( __filename ), {
    level: process.env.LOG_LEVEL || 'fatal',
});


const fs				= require('fs');
const expect				= require('chai').expect;
const { HoloHash }			= require('@whi/holo-hash');
const { Holochain }			= require('@whi/holochain-backdrop');
const json				= require('@whi/json');

const { backdrop }			= require('./setup.js');
const delay				= (n) => new Promise(f => setTimeout(f, n));


const DNAREPO_PATH			= path.join(__dirname, "../../bundled/dnarepo/dnarepo.dna");
const HAPPS_PATH			= path.join(__dirname, "../../bundled/happs/happs.dna");
const WEBASSETS_PATH			= path.join(__dirname, "../../bundled/web_assets/web_assets.dna");

const storage				= "storage";
const store				= "store";
const files				= "files";

const chunk_size			= (2**20 /*1 megabyte*/) * 2;

let clients;


function basic_tests () {

    it("should get whoami info", async function () {
	const asset_client		= clients.alice.webassets;

	let a_agent_info		= await asset_client.call( files, "whoami", null);
	log.info("Agent ID 'alice': %s", String(new HoloHash(a_agent_info.agent_initial_pubkey)) );
    });


    it("should assemble hApp bundle", async function () {
	this.timeout( 30_000 );

	const dnarepo_client		= clients.alice.dnarepo;
	const happ_client		= clients.alice.happs;
	const asset_client		= clients.alice.webassets;

	const file_bytes		= fs.readFileSync( path.resolve(__dirname, "../test.gz") );
	log.debug("GZ file bytes (%s): typeof %s", file_bytes.length, typeof file_bytes );

	let gz_file_hash;
	{
	    let chunk_hashes		= [];
	    let chunk_count		= Math.ceil( file_bytes.length / chunk_size );
	    for (let i=0; i < chunk_count; i++) {
		let chunk		= await asset_client.call( files, "create_file_chunk", {
		    "sequence": {
			"position": i+1,
			"length": chunk_count,
		    },
		    "bytes": file_bytes.slice( i*chunk_size, (i+1)*chunk_size ),
		});
		log.info("Chunk %s/%s hash: %s", i+1, chunk_count, String(chunk.$address) );

		chunk_hashes.push( chunk.$address );
	    }
	    log.debug("Final chunks:", json.debug(chunk_hashes) );

	    let file			= await asset_client.call( files, "create_file", {
		"file_size": file_bytes.length,
		"chunk_addresses": chunk_hashes,
	    });
	    log.normal("New GZ file: %s -> %s", String(file.$address), file.file_size );
	    gz_file_hash		= file.$address;
	}

	{
	    let gui			= await happ_client.call( store, "get_gui", {
		"dna_hash": asset_client.dna_hash,
		"id": gz_file_hash,
	    });
	    log.normal("Updated hApp UI: %s", gui.file_size );

	    expect( gui.file_size	).to.equal( 802067 );
	}


	const dna_input			= {
	    "name": "Game Turns",
	    "description": "A tool for turn-based games to track the order of player actions",
	};
	let dna				= await dnarepo_client.call( storage, "create_dna", dna_input );
	log.normal("New DNA (metadata): %s -> %s", String(dna.$id), dna.name );

	const dna_bytes			= fs.readFileSync( path.resolve(__dirname, "../test.dna") );
	log.debug("DNA file bytes (%s): typeof %s", dna_bytes.length, typeof dna_bytes );

	let version			= await dnarepo_client.call( storage, "create_dna_version", {
	    "for_dna": dna.$id,
	    "version": 1,
	    "file_size": dna_bytes.length,
	    "dna_bytes": dna_bytes,
	});
	log.normal("New DNA version: %s -> %s", String(version.$address), version.version );


	let happ_input			= {
	    "title": "Chess",
	    "subtitle": "Super fun board game",
	    "description": "Play chess with friends :)",
	    "gui": {
		"asset_group_id": gz_file_hash,
		"uses_web_sdk": false,
	    }
	};

	let happ			= await happ_client.call( store, "create_happ", happ_input );
	log.normal("New hApp: %s", String(happ.$addr) );

	expect( happ.description	).to.equal( happ_input.description );


	const manifest_yaml		= fs.readFileSync( path.resolve(__dirname, "../test_happ.yaml"), "utf8" );
	let release_input		= {
	    "name": "v0.1.0",
	    "description": "The first release",
	    "for_happ": happ.$id,
	    manifest_yaml,
	    "resources": {
		"test_dna": version.$id,
	    },
	};

	let release			= await happ_client.call( store, "create_happ_release", release_input );
	log.normal("New hApp release: %s -> %s", String(release.$addr), release.name );

	expect( release.description	).to.equal( release_input.description );


	{
	    let happ_package		= await happ_client.call( store, "get_release_package", {
		"id": release.$id,
		"dnarepo_dna_hash": dnarepo_client.dna_hash,
	    });
	    log.normal("hApp release package bytes: (%s) %s", happ_package.constructor.name, happ_package.length );

	    expect( happ_package.length	).to.equal( 899277 );

	    fs.writeFileSync( path.resolve(__dirname, "../multitesting.happ"), Buffer.from(happ_package) );
	}
    });
}


function errors_tests () {
}

describe("All DNAs", () => {

    const holochain			= new Holochain();

    before(async function () {
	this.timeout( 30_000 );

	clients				= await backdrop( holochain, {
	    "dnarepo": DNAREPO_PATH,
	    "happs": HAPPS_PATH,
	    "webassets": WEBASSETS_PATH,
	}, [
	    "alice",
	]);
    });

    describe("Basic", basic_tests.bind( this, holochain ) );
    describe("Errors", errors_tests.bind( this, holochain ) );

    after(async () => {
	await holochain.stop();
	await holochain.destroy();
    });

});
