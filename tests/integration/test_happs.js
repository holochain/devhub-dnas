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
const HAPPS_PATH			= path.join(__dirname, "../../bundled/happs/happs.dna");
const zome				= "store";


function basic_tests () {
    it("should get whoami info", async function () {
	let whoami			= await clients.alice.happs.call( zome, "whoami" );

	log.info("Agent ID 'alice': %s", String(new HoloHash(whoami.agent_initial_pubkey)) );
    });

    it("should manage happ configurations", async function () {
	this.timeout( 10_000 );

	const alice			= clients.alice.happs;
	const bobby			= clients.bobby.happs;
	const carol			= clients.carol.happs;


	let happ_input			= {
	    "title": "Chess",
	    "subtitle": "Super fun board game",
	    "description": "Play chess with friends :)",
	};

	let happ			= await alice.call( zome, "create_happ", happ_input );
	log.normal("New hApp: %s -> %s", String(happ.$addr), happ.title );

	expect( happ.description	).to.equal( happ_input.description );

	let happ_addr			= happ.$addr;
	{
	    let description		= "New description";
	    let update			= await alice.call( zome, "update_happ", {
		"addr": happ_addr,
		"properties": {
		    description,
		},
	    });
	    log.normal("New hApp: %s -> %s", String(update.$addr), update.title );
	    happ_addr			= update.$addr;

	    expect( update.description	).to.equal( description );

	    let _happ			= await alice.call( zome, "get_happ", {
		"id": happ.$id,
	    });
	    log.normal("Updated hApp: %s -> %s", String(_happ.$addr), _happ.title );

	    expect( _happ.description	).to.equal( description );
	}

	{
	    let message			= "This hApp is no longer maintained";
	    let update			= await alice.call( zome, "deprecate_happ", {
		"addr": happ_addr,
		"message": message,
	    });
	    log.normal("New hApp: %s -> %s", String(update.$addr), update.title );
	    happ_addr			= update.$addr;

	    expect( update.deprecation		).to.be.an( "object" );
	    expect( update.deprecation.message	).to.equal( message );

	    let _happ			= await alice.call( zome, "get_happ", {
		"id": happ.$id,
	    });
	    log.normal("Deprecated hApp: %s -> %s", String(_happ.$addr), _happ.title );

	    expect( _happ.deprecation		).to.be.an( "object" );
	    expect( _happ.deprecation.message	).to.equal( message );
	}

	const manifest_yaml		= fs.readFileSync( path.resolve(__dirname, "../test_happ.yaml"), "utf8" );
	let release_input		= {
	    "name": "v0.1.0",
	    "description": "The first release",
	    "for_happ": happ.$id,
	    manifest_yaml,
	    "resources": {
		"test_dna": new HoloHash("uhCEkNBaVvGRYmJUqsGNrfO8jC9Ij-t77QcmnAk3E3B8qh6TU09QN"),
	    },
	};

	let release			= await alice.call( zome, "create_happ_release", release_input );
	log.normal("New hApp release: %s -> %s", String(release.$addr), release.name );

	expect( release.description	).to.equal( release_input.description );

	{
	    let _release		= await alice.call( zome, "get_happ_release", {
		"id": release.$id,
	    });
	    log.normal("Updated release: %s -> %s", String(_release.$addr), _release.name );

	    expect( _release.description	).to.equal( release_input.description );
	}

	let happ_release_addr;
	{
	    let description		= "The first release (updated)";
	    let update			= await alice.call( zome, "update_happ_release", {
		"addr": release.$addr,
		"properties": {
		    description,
		},
	    });
	    log.normal("New hApp: %s -> %s", String(update.$addr), update.name );
	    happ_release_addr		= update.$addr;

	    expect( update.description	).to.equal( description );
	}

	{
	    let header			= await alice.call( zome, "delete_happ_release", {
		"id": release.$id,
	    });
	    log.normal("Delete hApp: %s", header );
	}

	{
	    let failed			= false;
	    try {
		await alice.call( zome, "get_happ_release", {
		    "id": release.$id,
		});
	    } catch (err) {
		expect( err.kind	).to.equal( "UserError" );
		expect( err.name	).to.equal( "EntryNotFoundError" );
		expect( err.message	).to.have.string( "Entry not found for address: " );

		failed			= true;
	    }

	    expect( failed		).to.be.true;
	}
    });
}

function errors_tests () {
}

describe("hApps", () => {

    const holochain			= new Holochain();

    before(async function () {
	this.timeout( 5_000 );

	clients				= await backdrop( holochain, {
	    "happs": HAPPS_PATH,
	}, [
	    "alice",
	    "bobby",
	    "carol",
	]);
    });

    describe("Basic", basic_tests.bind( this, holochain ) );
    describe("Errors", errors_tests.bind( this, holochain ) );

    after(async () => {
	await holochain.stop();
	await holochain.destroy();
    });

});
