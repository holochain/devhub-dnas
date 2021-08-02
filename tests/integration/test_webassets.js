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
const WEBASSETS_PATH			= path.join(__dirname, "../../bundled/web_assets/web_assets.dna");
const zome				= "web_assets";


function basic_tests () {
    it("should get whoami info", async function () {
	let whoami			= await clients.alice.webassets.call( zome, "whoami" );

	log.info("Agent ID 'alice': %s", String(new HoloHash(whoami.agent_initial_pubkey)) );
    });

    it("should manage files", async function () {
	this.timeout( 10_000 );

	const alice			= clients.alice.webassets;


	const file_bytes		= fs.readFileSync( path.resolve(__dirname, "../test.gz") );
	log.debug("GZ file bytes (%s): typeof %s", file_bytes.length, typeof file_bytes );

	let chunk_size			= (2**20 /*1 megabyte*/) * 2;
	let gz_file_hash;
	{
	    let chunk_hashes		= [];
	    let chunk_count		= Math.ceil( file_bytes.length / chunk_size );
	    for (let i=0; i < chunk_count; i++) {
		let chunk		= await alice.call( zome, "create_file_chunk", {
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

	    let version			= await alice.call( zome, "create_file", {
		"file_size": file_bytes.length,
		"chunk_addresses": chunk_hashes,
	    });
	    log.normal("New GZ version: %s -> %s", String(version.$address), version.version );
	    gz_file_hash		= version.$address;
	}
    });
}

function errors_tests () {
}

describe("Web Assets", () => {

    const holochain			= new Holochain();

    before(async function () {
	this.timeout( 30_000 );

	clients				= await backdrop( holochain, {
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
