const path				= require('path');
const log				= require('@whi/stdlog')(path.basename( __filename ), {
    level: process.env.LOG_LEVEL || 'fatal',
});

global.WebSocket			= require('ws');
const { AgentClient }			= require('@whi/holochain-client');
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
    await holochain.start( 5_000 );

    const app_id			= "test";
    const app_port			= 44910;
    const clients			= {};

    log.debug("Waiting for DNAs and actors to be set up...");
    const agents			= await holochain.backdrop( app_id, app_port, dnas, actors );
    const crux_config			= new CruxConfig();

    log.debug("Creating clients actors: %s", actors.join(", ") );
    await Promise.all( Object.entries( agents ).map( async ([ actor, happ ]) => {
	const dna_map			= {};
	await Promise.all( Object.entries( happ.cells ).map( async ([ role_id, cell ]) => {
	    dna_map[role_id]		= cell.dna.hash;
	    log.info("Established a new cell for '%s': %s => [ %s :: %s ]", actor, role_id, String(cell.dna.hash), String(happ.agent) );
	}) );

	const client			= new AgentClient( happ.agent, dna_map, app_port, client_options );
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
