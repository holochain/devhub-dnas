const path				= require('path');
const log				= require('@whi/stdlog')(path.basename( __filename ), {
    level: process.env.LOG_LEVEL || 'fatal',
});


const fs				= require('fs');
const crypto				= require('crypto');
const expect				= require('chai').expect;
const { HoloHash }			= require('@whi/holo-hash');
const { Holochain }			= require('@whi/holochain-backdrop');
const json				= require('@whi/json');

const { backdrop }			= require('./setup.js');

const delay				= (n) => new Promise(f => setTimeout(f, n));
const HAPPS_PATH			= path.join(__dirname, "../../bundled/happs.dna");


function basic_tests () {
    it("should manage happ configurations", async function () {
	this.timeout( 10_000 );

	const alice			= clients.alice;

	let happ_input			= {
	    "title": "Chess",
	    "subtitle": "Super fun board game",
	    "description": "Play chess with friends :)",
	    "tags": [ "Games", "Strategy" ],
	};

	let happ			= await alice.call( "happs", "happ_library", "create_happ", happ_input );
	log.normal("New hApp: %s -> %s", String(happ.$addr), happ.title );

	expect( happ.description	).to.equal( happ_input.description );

	{
	    let happs			= await alice.call( "happs", "happ_library", "get_my_happs" );
	    log.normal("My hApps: %s -> %s", happs.length, String(happs.$base) );

	    expect( happs		).to.have.length( 1 );
	}

	{
	    let happs			= await alice.call( "happs", "happ_library", "get_happs_by_filter", {
		"filter": "title",
		"keyword": happ_input.title.toLowerCase(),
	    });
	    log.normal("hApps by title: %s -> %s", happs.length, String(happs.$base) );

	    expect( happs		).to.have.length( 1 );
	}

	{
	    let happs			= await alice.call( "happs", "happ_library", "get_happs_by_tags", [ "Games" ] );
	    log.normal("hApps by title: %s -> %s", happs.length, String(happs.$base) );

	    expect( happs		).to.have.length( 1 );
	}
	{
	    let happs			= await alice.call( "happs", "happ_library", "get_happs_by_tags", [ "games", "strategy" ] );
	    log.normal("hApps by title: %s -> %s", happs.length, String(happs.$base) );

	    expect( happs		).to.have.length( 1 );
	}
	{
	    let happs			= await alice.call( "happs", "happ_library", "get_happs_by_tags", [ "Games", "Action" ] );
	    log.normal("hApps by title: %s -> %s", happs.length, String(happs.$base) );

	    expect( happs		).to.have.length( 0 );
	}

	let happ_addr			= happ.$addr;
	{
	    let title			= "Chess++";
	    let description		= "New description";
	    let tags			= [ "Games", "Boardgame" ];
	    let update			= await alice.call( "happs", "happ_library", "update_happ", {
		"addr": happ_addr,
		"properties": {
		    title,
		    description,
		    tags,
		},
	    });
	    log.normal("New hApp: %s -> %s", String(update.$addr), update.title );
	    happ_addr			= update.$addr;

	    expect( update.title	).to.equal( title );
	    expect( update.description	).to.equal( description );

	    happ			= await alice.call( "happs", "happ_library", "get_happ", {
		"id": happ.$id,
	    });
	    log.normal("Updated hApp: %s -> %s", String(happ.$addr), happ.title );

	    expect( happ.description	).to.equal( description );
	}

	{
	    let happs			= await alice.call( "happs", "happ_library", "get_happs_by_tags", [ "strategy" ] );
	    log.normal("hApps by title: %s -> %s", happs.length, String(happs.$base) );

	    expect( happs		).to.have.length( 0 );
	}
	{
	    let happs			= await alice.call( "happs", "happ_library", "get_happs_by_tags", [ "games", "boardgame" ] );
	    log.normal("hApps by title: %s -> %s", happs.length, String(happs.$base) );

	    expect( happs		).to.have.length( 1 );
	}

	{
	    let happs			= await alice.call( "happs", "happ_library", "get_happs_by_filter", {
		"filter": "title",
		"keyword": happ_input.title.toLowerCase(),
	    });
	    log.normal("hApps by title: %s -> %s", happs.length, String(happs.$base) );

	    expect( happs		).to.have.length( 0 );
	}

	{
	    let happs			= await alice.call( "happs", "happ_library", "get_happs_by_filter", {
		"filter": "title",
		"keyword": happ.title.toLowerCase(),
	    });
	    log.normal("hApps by title: %s -> %s", happs.length, String(happs.$base) );

	    expect( happs		).to.have.length( 1 );
	}

	{
	    let message			= "This hApp is no longer maintained";
	    let update			= await alice.call( "happs", "happ_library", "deprecate_happ", {
		"addr": happ_addr,
		"message": message,
	    });
	    log.normal("New hApp: %s -> %s", String(update.$addr), update.title );
	    happ_addr			= update.$addr;

	    expect( update.deprecation		).to.be.an( "object" );
	    expect( update.deprecation.message	).to.equal( message );

	    let _happ			= await alice.call( "happs", "happ_library", "get_happ", {
		"id": happ.$id,
	    });
	    log.normal("Deprecated hApp: %s -> %s", String(_happ.$addr), _happ.title );

	    expect( _happ.deprecation		).to.be.an( "object" );
	    expect( _happ.deprecation.message	).to.equal( message );
	}

	const dna_id			= new HoloHash("uhCEkh3HCoTRCZD2I7H-gcf5VNdqXUdT4Nq6B8WUo-pzMZ338XDlb");
	const dna_version_id		= new HoloHash("uhCEkxe-5fTSvh_WVchpAmEvMbN9aGAu_Nm3GwN03IM2kmmyPmLxy");
	const dna_wasm_hash		= "07bb7ae9898a64c69617a8dc0faf0c9449ccd0c0b2a81be29763b8a95d7bd708";
	const manifest_yaml		= fs.readFileSync( path.resolve(__dirname, "../test_happ.yaml"), "utf8" );
	let release_input		= {
	    "name": "v0.1.0",
	    "description": "The first release",
	    "for_happ": happ.$id,
	    "manifest": {
		"manifest_version": "1",
		"roles": [
		    {
			"id": "test_dna",
			"dna": {
			    "path": "./this/does/not/matter.dna",
			    "clone_limit": 0,
			},
		    },
		],
	    },
	    "hdk_version": "v0.0.120",
	    "dnas": [
		{
		    "role_id": "test_dna",
		    "dna": dna_id,
		    "version": dna_version_id,
		    "wasm_hash": dna_wasm_hash,
		}
	    ],
	};

	let release			= await alice.call( "happs", "happ_library", "create_happ_release", release_input );
	log.normal("New hApp release: %s -> %s", String(release.$addr), release.name );

	expect( release.description	).to.equal( release_input.description );

	{
	    let dna_hash_bytes		= Buffer.from( dna_wasm_hash, "hex" );
	    let hash			= crypto.createHash("sha256");
	    hash.update( dna_hash_bytes );

	    let versions		= await alice.call( "happs", "happ_library", "get_happ_releases_by_filter", {
		"filter": "uniqueness_hash",
		"keyword": hash.digest("hex"),
	    });
	    log.normal("hApp releases by hash: %s -> %s", versions.length, String(versions.$base) );

	    expect( versions		).to.have.length( 1 );
	}

	{
	    let _release		= await alice.call( "happs", "happ_library", "get_happ_release", {
		"id": release.$id,
	    });
	    log.normal("Updated release: %s -> %s", String(_release.$addr), _release.name );

	    expect( _release.description	).to.equal( release_input.description );
	}

	{
	    let releases		= await alice.call( "happs", "happ_library", "get_happ_releases", {
		"for_happ": happ.$id,
	    });
	    log.normal("hApp Releases %s -> %s", releases.length, String(releases.$base) );

	    expect( happ.$id		).to.deep.equal( releases.$base );
	    expect( releases		).to.have.length( 1 );
	}

	let happ_release_addr;
	{
	    let description		= "The first release (updated)";
	    let update			= await alice.call( "happs", "happ_library", "update_happ_release", {
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
	    let header			= await alice.call( "happs", "happ_library", "delete_happ_release", {
		"id": release.$id,
	    });
	    log.normal("Delete hApp: %s", new HoloHash( header ) );
	}

	{
	    let failed			= false;
	    try {
		await alice.call( "happs", "happ_library", "get_happ_release", {
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

    it("should make multiple asynchronous calls to get_happs_by_filter", async function () {
	await Promise.all( [1,2].map( async () => {
	    let happs			= await clients.alice.call( "happs", "happ_library", "get_happs_by_filter", {
		"filter": "title",
		"keyword": crypto.randomBytes( 10 ).toString("hex"),
	    });
	    log.normal("hApps by title: %s -> %s", happs.length, String(happs.$base) );

	    expect( happs		).to.have.length( 0 );
	}) );
    });

    it("should make multiple asynchronous calls to get_happs_releases_by_filter", async function () {
	await Promise.all( [1,2].map( async () => {
	    let versions		= await clients.alice.call( "happs", "happ_library", "get_happ_releases_by_filter", {
		"filter": "uniqueness_hash",
		"keyword": crypto.randomBytes( 10 ).toString("hex"),
	    });
	    log.normal("hApp releases by hash: %s -> %s", versions.length, String(versions.$base) );

	    expect( versions		).to.have.length( 0 );
	}) );
    });

    it("should get all hApps", async function () {
	let happs			= await clients.alice.call( "happs", "happ_library", "get_all_happs");
	log.normal("hApps by hash: %s -> %s", happs.length, String(happs.$base) );

	expect( happs			).to.have.length( 1 );
    });
}

function errors_tests () {
}

describe("hApps", () => {

    const holochain			= new Holochain({
	"default_stdout_loggers": process.env.LOG_LEVEL === "silly",
    });

    before(async function () {
	this.timeout( 30_000 );

	clients				= await backdrop( holochain, {
	    "happs": HAPPS_PATH,
	}, [
	    "alice",
	]);

	// Must call whoami on each cell to ensure that init has finished.
	{
	    let whoami			= await clients.alice.call( "happs", "happ_library", "whoami" );
	    log.normal("Alice whoami: %s", String(new HoloHash( whoami.agent_initial_pubkey )) );
	}
    });

    describe("Basic", basic_tests.bind( this, holochain ) );
    describe("Errors", errors_tests.bind( this, holochain ) );

    after(async () => {
	await holochain.destroy();
    });

});
