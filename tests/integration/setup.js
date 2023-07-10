const path				= require('path');
const log				= require('@whi/stdlog')(path.basename( __filename ), {
    level: process.env.LOG_LEVEL || 'fatal',
});

global.WebSocket			= require('ws');
const { CruxConfig, ...crux }		= require('@whi/crux-payload-parser');

// crux.log.setLevel("trace");

const all_clients			= [];
function exit_cleanup () {
    all_clients.forEach( client => client.close() );
}
process.once("exit", exit_cleanup );


async function backdrop ( holochain, dnas, actors, client_options ) {
    log.normal("Setting up backdrop with %s DNAs and %s Agents", Object.keys(dnas).length, actors.length );

    log.debug("Waiting for holochain to start...");
    await holochain.start( 30_000 );

    const app_id			= "test";
    const app_port			= 44910;
    const clients			= {};

    log.debug("Waiting for DNAs and actors to be set up...");
    const agents			= await holochain.backdrop({ "test_happ": dnas, }, {
	"timeout": 30_000,
	actors,
    });
    const crux_config			= new CruxConfig();

    log.debug("Creating clients actors: %s", actors.join(", ") );
    await Promise.all( Object.entries( agents ).map( async ([ actor, happs ]) => {
	const happ			= happs.test_happ;
	const dna_map			= {};
	await Promise.all( Object.entries( happ.cells ).map( async ([ role_name, cell ]) => {
	    dna_map[role_name]		= cell.dna;
	    log.info("Established a new cell for '%s': %s => [ %s :: %s ]", actor, role_name, String(cell.dna), String(happ.agent) );
	}) );

	const client			= happ.client;
	crux_config.upgrade( client );
	clients[actor]			= client

	all_clients.push( client );
    }) );
    log.info("Finished backdrop setup: %s", Object.keys(clients) );

    return clients;
}


module.exports = {
    backdrop,
};
