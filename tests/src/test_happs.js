const path				= require('path');
const log				= require('@whi/stdlog')(path.basename( __filename ), {
    level: process.env.LOG_LEVEL || 'fatal',
});


const fs				= require('fs');
const expect				= require('chai').expect;
const { HoloHash }			= require('@whi/holo-hash');
const json				= require('@whi/json');

const { delay, callZome,
	orchestrator,
	create_players }		= require('./utils.js');


const happs_dna				= path.join(__dirname, "../../bundled/happs/happs.dna");
const dna_list				= [ happs_dna ];
const zome				= "store";


orchestrator.registerScenario('hApps::store API', async (scenario, _) => {
    const [ alice_happ,
	    bobby_happ,
	    carol_happ ]		= await create_players( scenario, dna_list, ["alice", "bobby", "carol"] );

    const [ alice_client ]		= alice_happ.cells;
    const [ bobby_client ]		= bobby_happ.cells;
    const [ carol_client ]		= carol_happ.cells;


    let a_agent_info			= await alice_client( zome, "whoami", null);
    let b_agent_info			= await bobby_client( zome, "whoami", null);
    let c_agent_info			= await carol_client( zome, "whoami", null);

    log.info("Agent ID 'alice': %s", String(new HoloHash(a_agent_info.agent_initial_pubkey)) );


    let happ_input			= {
	"title": "Chess",
	"subtitle": "Super fun board game",
	"description": "Play chess with friends :)",
    };

    let happ				= await alice_client( zome, "create_happ", happ_input );
    log.normal("New hApp: %s -> %s", String(happ.$addr), happ.title );

    expect( happ.description		).to.equal( happ_input.description );

    let happ_addr			= happ.$addr;
    {
	let description			= "New description";
	let update			= await alice_client( zome, "update_happ", {
	    "addr": happ_addr,
	    "properties": {
		description,
	    },
	});
	log.normal("New hApp: %s -> %s", String(update.$addr), update.title );
	happ_addr			= update.$addr;

	expect( update.description	).to.equal( description );

	let _happ			= await alice_client( zome, "get_happ", {
	    "id": happ.$id,
	});
	log.normal("Updated hApp: %s -> %s", String(_happ.$addr), _happ.title );

	expect( _happ.description	).to.equal( description );
    }

    {
	let message			= "This hApp is no longer maintained";
	let update			= await alice_client( zome, "deprecate_happ", {
	    "addr": happ_addr,
	    "message": message,
	});
	log.normal("New hApp: %s -> %s", String(update.$addr), update.title );
	happ_addr			= update.$addr;

	expect( update.deprecation		).to.be.an( "object" );
	expect( update.deprecation.message	).to.equal( message );

	let _happ			= await alice_client( zome, "get_happ", {
	    "id": happ.$id,
	});
	log.normal("Deprecated hApp: %s -> %s", String(_happ.$addr), _happ.title );

	expect( _happ.deprecation		).to.be.an( "object" );
	expect( _happ.deprecation.message	).to.equal( message );
    }

    const manifest_yaml			= fs.readFileSync( path.resolve(__dirname, "../test_happ.yaml"), "utf8" );
    let release_input			= {
	"name": "v0.1.0",
	"description": "The first release",
	"for_happ": happ_addr,
	manifest_yaml,
	"resources": {
	    "test_dna": new HoloHash("uhCEkNBaVvGRYmJUqsGNrfO8jC9Ij-t77QcmnAk3E3B8qh6TU09QN"),
	},
    };

    let release				= await alice_client( zome, "create_happ_release", release_input );
    log.normal("New hApp release: %s -> %s", String(release.$addr), release.name );

    expect( release.description		).to.equal( release_input.description );

    {
	let _release			= await alice_client( zome, "get_happ_release", {
	    "id": release.$id,
	});
	log.normal("Updated release: %s -> %s", String(_release.$addr), _release.name );

	expect( _release.description	).to.equal( release_input.description );
    }

    {
	let failed			= false;
	try {
	    await alice_client( zome, "get_happ", {
		"id": new HoloHash("uhCEkNBaVvGRYmJUqsGNrfO8jC9Ij-t77QcmnAk3E3B8qh6TU09QN"),
	    });
	} catch (err) {
	    expect( err.kind		).to.equal( "UserError" );
	    expect( err.name		).to.equal( "EntryNotFoundError" );
	    expect( err.message		).to.have.string( "Entry not found for address: " );

	    failed			= true;
	}

	expect( failed			).to.be.true;
    }
});

orchestrator.run();
