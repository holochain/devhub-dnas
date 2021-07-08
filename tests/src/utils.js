const path				= require('path');
const log				= require('@whi/stdlog')(path.basename( __filename ), {
    level: process.env.LOG_LEVEL || 'fatal',
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
    log.normal("Calling conductor: %s->%s({ %s })", zome, fn_name, Object.keys(args || {}).join(", ") )
    let response;
    try {
	response			= await this.call(zome, fn_name, args );
    } catch ( err ) {
	log.error("Conductor returned error: %s", err );
	if ( err instanceof Error )
	    console.error( err );
	throw err;
    }

    log.silly("Call Zome FULL Response: %s", json.debug(response, 4, (k,v) => {
	try {
	    return (new HoloHash(v)).toString();
	} catch (err) {
	    return v;
	}
    }) );

    let pack;
    try {
	pack				= Interpreter.parse( response );
    } catch ( err ) {
	log.error("Failed to interpret Essence package: %s", String(err) );
	console.error( err.stack );
	throw err;
    }

    let payload				= pack.value();

    if ( payload instanceof Error ) {
	log.warn("Throwing error package: %s::%s( %s )", payload.kind, payload.name, payload.message );
	throw payload;
    }

    let composition			= pack.metadata('composition');

    try {
	return Schema.deconstruct( composition, payload );
    } catch ( err ) {
	log.error("Failed to deconstruct payload: %s", String(err) );
	console.error( err.stack );
	throw err;
    }
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

    log.silly("Installation: %s", json.debug(installations.map(happs => {
	return {
	    "id": happs[0].hAppId,
	    "agent": happs[0].agent,
	};
    })) );
    const agents_happ			= installations.map( happs => happs[0] );

    agents_happ.forEach( (happ, i) => {
	happ.cells.forEach( (cell_client, n) => {
	    let fn			= callZome.bind(cell_client);

	    happ.cells[n]		= Object.assign( fn, {
		"cell":		cell_client,
		"nick":		cell_client.cellNick,
		"id": {
		    "dna":	new HoloHash( cell_client.cellId[0] ),
		    "agent":	new HoloHash( cell_client.cellId[1] ),
		},
	    });
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
