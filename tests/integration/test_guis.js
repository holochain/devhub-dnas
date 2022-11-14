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
const { ...hc_client }			= require('@whi/holochain-client');

const { expect_reject }			= require('./utils.js');
const { backdrop }			= require('./setup.js');

const delay				= (n) => new Promise(f => setTimeout(f, n));
const HAPPS_PATH			= path.join(__dirname, "../../bundled/happs.dna");
const WEBASSETS_PATH			= path.join(__dirname, "../../bundled/web_assets.dna");

let gui_1;
let gui_release_1;
let gui_release_2;

function basic_tests () {
    it("should manage gui configurations", async function () {
	this.timeout( 10_000 );

	const alice			= clients.alice;

	let gui_input			= {
	    "name": "Web UI for Chess",
	    "description": "Play chess with friends :)",
	    "tags": [ "Games", "Strategy" ],
	};

	let gui			= gui_1 = await alice.call( "happs", "happ_library", "create_gui", gui_input );
	log.normal("New gui: %s -> %s", String(gui.$addr), gui.name );

	expect( gui.description	).to.equal( gui_input.description );

	{
	    let guis			= await alice.call( "happs", "happ_library", "get_my_guis" );
	    log.normal("My guis: %s", guis.length );

	    expect( guis		).to.have.length( 1 );
	}

	{
	    let guis			= await alice.call( "happs", "happ_library", "get_guis_by_tags", [ "Games" ] );
	    log.normal("guis by name: %s", guis.length );

	    expect( guis		).to.have.length( 1 );
	}
	{
	    let guis			= await alice.call( "happs", "happ_library", "get_guis_by_tags", [ "games", "strategy" ] );
	    log.normal("guis by name: %s", guis.length );

	    expect( guis		).to.have.length( 1 );
	}
	{
	    let guis			= await alice.call( "happs", "happ_library", "get_guis_by_tags", [ "Games", "Action" ] );
	    log.normal("guis by name: %s", guis.length );

	    expect( guis		).to.have.length( 0 );
	}

	let gui_addr			= gui.$action;
	{
	    let name			= "Chess++";
	    let description		= "New description";
	    let tags			= [ "Games", "Boardgame" ];
	    let update			= gui_1 = await alice.call( "happs", "happ_library", "update_gui", {
		"addr": gui_addr,
		"properties": {
		    name,
		    description,
		    tags,
		},
	    });
	    log.normal("New gui: %s -> %s", String(update.$addr), update.name );
	    gui_addr			= update.$action;

	    expect( update.name	).to.equal( name );
	    expect( update.description	).to.equal( description );

	    gui			= await alice.call( "happs", "happ_library", "get_gui", {
		"id": gui.$id,
	    });
	    log.normal("Updated gui: %s -> %s", String(gui.$addr), gui.name );

	    expect( gui.description	).to.equal( description );
	}

	{
	    let guis			= await alice.call( "happs", "happ_library", "get_guis_by_tags", [ "strategy" ] );
	    log.normal("guis by name: %s", guis.length );

	    expect( guis		).to.have.length( 0 );
	}

	{
	    let guis			= await alice.call( "happs", "happ_library", "get_guis_by_tags", [ "games", "boardgame" ] );
	    log.normal("guis by name: %s", guis.length );

	    expect( guis		).to.have.length( 1 );
	}

	{
	    let guis			= await clients.alice.call( "happs", "happ_library", "get_all_guis");
	    log.normal("guis by hash: %s", guis.length );

	    expect( guis		).to.have.length( 1 );
	}

	{
	    let message			= "This gui is no longer maintained";
	    let update			= gui_1 = await alice.call( "happs", "happ_library", "deprecate_gui", {
		"addr": gui_addr,
		"message": message,
	    });
	    log.normal("New gui: %s -> %s", String(update.$addr), update.name );
	    gui_addr			= update.$action;

	    expect( update.deprecation		).to.be.an( "object" );
	    expect( update.deprecation.message	).to.equal( message );

	    let _gui			= await alice.call( "happs", "happ_library", "get_gui", {
		"id": gui.$id,
	    });
	    log.normal("Deprecated gui: %s -> %s", String(_gui.$addr), _gui.name );

	    expect( _gui.deprecation		).to.be.an( "object" );
	    expect( _gui.deprecation.message	).to.equal( message );
	}

	{
	    let guis			= await alice.call( "happs", "happ_library", "get_guis_by_tags", [ "games", "boardgame" ] );
	    log.normal("guis by name: %s", guis.length );

	    expect( guis		).to.have.length( 0 );
	}

	const webasset_bytes		= fs.readFileSync( path.resolve(__dirname, "../test.zip") );
	log.debug("Zip file bytes (%s): typeof %s", webasset_bytes.length, typeof webasset_bytes );

	let webasset_addr;
	{
	    let webasset			= await alice.call( "web_assets", "web_assets", "create_file", {
		"file_bytes": webasset_bytes,
	    });
	    log.normal("New webasset file: %s -> %s", String(webasset.$address), webasset.version );
	    webasset_addr			= webasset.$address;
	}

	const happ_release_id		= new HoloHash("uhCEkxe-5fTSvh_WVchpAmEvMbN9aGAu_Nm3GwN03IM2kmmyPmLxy");

	let release_input		= {
	    "version": "v0.1.0",
	    "changelog": "The first release",
	    "for_gui": gui.$id,
	    "for_happ_releases": [ happ_release_id ],
	    "web_asset_id": webasset_addr,
	};

	let release			= gui_release_1 = await alice.call( "happs", "happ_library", "create_gui_release", release_input );
	log.normal("New gui release: %s -> %s", String(release.$addr), release.name );

	expect( release.changelog	).to.equal( release_input.changelog );

	{
	    gui_release_2		= await alice.call( "happs", "happ_library", "create_gui_release", release_input );
	}

	{
	    let _release		= await alice.call( "happs", "happ_library", "get_gui_release", {
		"id": release.$id,
	    });
	    log.normal("Updated release: %s -> %s", String(_release.$addr), _release.name );

	    expect( _release.changelog	).to.equal( release_input.changelog );
	}

	{
	    let releases		= await alice.call( "happs", "happ_library", "get_gui_releases", {
		"for_gui": gui.$id,
	    });
	    log.normal("gui Releases %s", releases.length );

	    expect( releases		).to.have.length( 2 );
	}

	let gui_release_addr;
	{
	    let changelog		= "The first release (updated)";
	    let update			= gui_release_1 = await alice.call( "happs", "happ_library", "update_gui_release", {
		"addr": release.$action,
		"properties": {
		    changelog,
		},
	    });
	    log.normal("New gui: %s -> %s", String(update.$addr), update.name );
	    gui_release_addr		= update.$addr;

	    expect( update.changelog	).to.equal( changelog );
	}

	{
	    let action			= await alice.call( "happs", "happ_library", "delete_gui_release", {
		"id": release.$id,
	    });
	    log.normal("Delete gui: %s", new HoloHash( action ) );
	}

	{
	    let failed			= false;
	    try {
		await alice.call( "happs", "happ_library", "get_gui_release", {
		    "id": release.$id,
		});
	    } catch (err) {
		expect( err.kind	).to.equal( "UserError" );
		expect( err.name	).to.equal( "EntryNotFoundError" );
		expect( err.message	).to.have.string( "Record not found for Entry address" );

		failed			= true;
	    }

	    expect( failed		).to.be.true;
	}
    });

    it("should get all guis", async function () {
	let guis			= await clients.alice.call( "happs", "happ_library", "get_all_guis");
	log.normal("guis by hash: %s", guis.length );

	expect( guis			).to.have.length( 0 );
    });
}

function errors_tests () {
}

describe("GUIs", () => {

    const holochain			= new Holochain({
	"default_stdout_loggers": process.env.LOG_LEVEL === "silly",
    });

    before(async function () {
	this.timeout( 30_000 );

	clients				= await backdrop( holochain, {
	    "happs": HAPPS_PATH,
	    "web_assets": WEBASSETS_PATH,
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
