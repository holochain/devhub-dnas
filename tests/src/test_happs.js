const path				= require('path');
const log				= require('@whi/stdlog')(path.basename( __filename ), {
    level: process.env.LOG_LEVEL || 'silly',
});


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

    log.info("Agent info 'alice': %s", json.debug(a_agent_info) );
    log.info("Agent ID 'alice': %s", a_agent_info.agent_initial_pubkey.toString("base64") );


    let happ_input			= {
	"name": "Chess",
	"description": "Play chess with friends :)",
    };

    let happ				= await alice_client( zome, "create_happ", happ_input );
    log.normal("New hApp: %s -> %s", String(happ.$addr), json.debug(happ) );

    {
	let description			= "New description";
	let update			= await alice_client( zome, "update_happ", {
	    "addr": happ.$addr,
	    "properties": {
		description,
	    },
	});
	log.normal("New hApp: %s -> %s", String(update.$addr), json.debug(update) );
    }
});

orchestrator.run();
