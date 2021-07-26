const path				= require('path');
const log				= require('@whi/stdlog')(path.basename( __filename ), {
    level: process.env.LOG_LEVEL || 'fatal',
});

const { Client, logging }		= require('@holochain/devhub-entities');

if ( process.env.LOG_LEVEL )
    logging();

const all_clients			= [];
function exit_cleanup () {
    all_clients.forEach( client => client.destroy() );
}
process.once("exit", exit_cleanup );


async function backdrop ( holochain, dnas, actors ) {
    holochain.on("lair:stdout", (line, parts) => {
	log.debug( "\x1b[39;1m     Lair STDOUT:\x1b[22;37m %s", line );
    });

    holochain.on("lair:stderr", (line, parts) => {
	log.debug( "\x1b[31;1m     Lair STDERR:\x1b[22m %s", line );
    });

    holochain.on("conductor:stdout", (line, parts) => {
	log.debug( "\x1b[39;1mConductor STDOUT:\x1b[22;37m %s", line );
    });

    holochain.on("conductor:stderr", (line, parts) => {
	log.debug( "\x1b[31;1mConductor STDERR:\x1b[22m %s", line );
    });

    await holochain.start();

    const app_id			= "test";
    const app_port			= 44910;
    const clients			= {};

    const agents			= await holochain.backdrop( app_id, app_port, dnas, actors );

    await Promise.all( Object.entries( agents ).map( async ([ actor, happ ]) => {
	clients[actor]		= happ.agent;

	await Promise.all( Object.entries( happ.cells ).map( async ([ nick, cell ]) => {
	    const client		= new Client( app_port, cell.dna.hash, cell.agent );
	    await client.connect();

	    log.info("Established a new cell for '%s': %s => [ %s :: %s ]", actor, nick, String(cell.dna.hash), String(happ.agent) );
	    clients[actor][nick]	= client;

	    all_clients.push( client );
	}) );
    }) );
    log.info("Finished backdrop setup: %s", Object.keys(clients) );

    return clients;
}


module.exports = {
    backdrop,
};
