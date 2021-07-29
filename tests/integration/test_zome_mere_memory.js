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
const MEMORY_PATH			= path.join(__dirname, "../dnas/memory/memory.dna");
const zome				= "mere_memory";


function basic_tests () {
    it("should create a memory manually", async function () {
	this.timeout( 10_000 );

	const alice			= clients.alice.memory;


	const memory_bytes		= fs.readFileSync( path.resolve(__dirname, "../test.gz") );
	log.debug("GZ memory bytes (%s): typeof %s", memory_bytes.length, typeof memory_bytes );

	let block_size			= (2**20 /*1 megabyte*/) / 2;
	{
	    let block_hashes		= [];
	    let block_count		= Math.ceil( memory_bytes.length / block_size );
	    for (let i=0; i < block_count; i++) {
		let addr		= new HoloHash( await alice.call( zome, "create_memory_block", {
		    "sequence": {
			"position": i+1,
			"length": block_count,
		    },
		    "bytes": memory_bytes.slice( i*block_size, (i+1)*block_size ),
		}) );
		log.info("Block %s/%s address: %s", i+1, block_count, String(addr) );

		block_hashes.push( addr );
	    }
	    log.debug("Final blocks:", json.debug(block_hashes) );

	    let addr			= new HoloHash( await alice.call( zome, "create_memory", {
		"memory_size": memory_bytes.length,
		"block_addresses": block_hashes,
	    }) );
	    log.normal("New GZ address: %s", String(addr) );
	}
    });

    let memory_addr;
    it("should create a memory using 'save_bytes'", async function () {
	this.timeout( 10_000 );

	const alice			= clients.alice.memory;


	const memory_bytes		= fs.readFileSync( path.resolve(__dirname, "../test.gz") );
	log.debug("GZ memory bytes (%s): typeof %s", memory_bytes.length, typeof memory_bytes );

	let addr			= new HoloHash( await alice.call( zome, "save_bytes", memory_bytes ) );
	log.normal("New GZ address: %s", String(addr) );

	memory_addr			= addr;
    });

    it("should get a memory using 'retrieve_bytes'", async function () {
	this.timeout( 10_000 );

	const alice			= clients.alice.memory;

	const memory_bytes		= fs.readFileSync( path.resolve(__dirname, "../test.gz") );
	let memory			= Buffer.from( await alice.call( zome, "retrieve_bytes", memory_addr ) );

	expect( memory			).to.have.length( memory_bytes.length );

	let cmp_result			= Buffer.compare( memory, memory_bytes );
	expect( cmp_result		).to.equal( 0 );
    });
}

function errors_tests () {
}

describe("Zome: Mere Memory", () => {

    const holochain			= new Holochain();

    before(async function () {
	this.timeout( 10_000 );

	clients				= await backdrop( holochain, {
	    "memory": MEMORY_PATH,
	}, [
	    "alice",
	], {
	    "parse_entities": false,
	});
    });

    describe("Basic", basic_tests.bind( this, holochain ) );
    describe("Errors", errors_tests.bind( this, holochain ) );

    after(async () => {
	await holochain.stop();
	await holochain.destroy();
    });

});
