const path				= require('path');
const log				= require('@whi/stdlog')(path.basename( __filename ), {
    level: process.env.LOG_LEVEL || 'silly',
});


const { Orchestrator,
	Config }			= require('@holochain/tryorama');
const { Schema }			= require('@holochain/devhub-entities');

const { HoloHash }			= require('@whi/holo-hash');
const { Translator }			= require('@whi/essence');
const json				= require('@whi/json');


const delay				= ms => new Promise(f => setTimeout(f,ms));
const Interpreter			= new Translator(["AppError", "UtilsError", "DNAError", "UserError"], {
    "rm_stack_lines": 2,
});


async function callZome ( zome, fn_name, args ) {
    log.normal("Calling conductor: %s->%s( %s )", zome, fn_name, Object.keys(args || {}).join(", ") )
    let response			= await this.call(zome, fn_name, args );

    log.silly("Call Zome FULL Response: %s", json.debug(response, 4, (k,v) => {
	try {
	    return (new HoloHash(v)).toString();
	} catch (err) {
	    return v;
	}
    }) );

    let pack				= Interpreter.parse( response );
    let payload				= pack.value();

    if ( payload instanceof Error ) {
	console.error("Throwing error package:", payload );
	throw payload;
    }

    let composition			= pack.metadata('composition');

    return Schema.deconstruct( composition, payload );
}


const orchestrator			= new Orchestrator({
    "mode": {
	executor: { tape: require('tape') },
	spawning: 'local',
    },
});


async function create_players ( scenario, happ_dnas, agents ) {
    const config			= Config.gen();

    log.debug("Creating scenario players with default 'Config.gen()' input");
    const [ conductor ]			= await scenario.players([ config ]);

    log.debug("hApp configuration for %s agent(s): %s", agents.length, happ_dnas );
    const agent_configs			= agents.map(_ => [ happ_dnas ]);
    const installations			= await conductor.installAgentsHapps( agent_configs );

    log.debug("Installation: %s", json.debug(installations.map(happs => {
	return {
	    "id": happs[0].hAppId,
	    "agent": happs[0].agent,
	};
    })) );
    const agents_happ			= installations.map( happs => happs[0] );

    agents_happ.forEach( (happ, i) => {
	happ.cells.forEach( (cell_client, n) => {
	    happ.cells[n]		= callZome.bind(cell_client);
	    happ.cells[n].cell		= cell_client;
	});
    });

    return agents_happ;
}


module.exports = {
    delay,
    callZome,
    orchestrator,
    create_players,
};
