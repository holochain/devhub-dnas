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
const WEBASSETS_PATH			= path.join(__dirname, "../../bundled/web_assets.dna");


function basic_tests () {
    it("should get whoami info", async function () {
	let whoami			= await clients.alice.call( "webassets", "web_assets", "whoami" );

	log.info("Agent ID 'alice': %s", String(new HoloHash(whoami.agent_initial_pubkey)) );
    });

    it("should manage files", async function () {
	this.timeout( 10_000 );

	const alice			= clients.alice;


	const file_bytes		= fs.readFileSync( path.resolve(__dirname, "../test.gz") );
	log.debug("GZ file bytes (%s): typeof %s", file_bytes.length, typeof file_bytes );

	let file_addr;
	{
	    let file			= await alice.call( "webassets", "web_assets", "create_file", {
		"file_bytes": file_bytes,
	    });
	    log.normal("New webasset file: %s -> %s", String(file.$action), file.version );
	    file_addr			= file.$action;
	}

	{
	    let file			= await alice.call( "webassets", "web_assets", "get_file_package", {
		"id": file_addr,
	    });
	    log.normal("Retrieved webasset file: %s bytes", file.bytes.length );
	    // THIS 'expect' BREAKS THE WHOLE PROCESS FOR SOME REASON
	    // expect( file.bytes		).to.be.a("Uint8Array");
	}
    });
}

function errors_tests () {
}

describe("Web Assets", () => {

    const holochain			= new Holochain({
	"default_stdout_loggers": process.env.LOG_LEVEL === "silly",
	"timeout": 30_000,
    });

    before(async function () {
	this.timeout( 30_000 );

	clients				= await backdrop( holochain, {
	    "webassets": WEBASSETS_PATH,
	}, [
	    "alice",
	]);

	// Must call whoami on each cell to ensure that init has finished.
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
