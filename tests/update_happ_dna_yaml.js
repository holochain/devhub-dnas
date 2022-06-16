const path				= require('path');
const log				= require('@whi/stdlog')(path.basename( __filename ), {
    level: process.env.LOG_LEVEL || 'fatal',
});

const fs				= require('fs');
const YAML				= require('yaml');

global.WebSocket			= require('ws');
const { AdminClient }			= require('@whi/holochain-client');
const { Holochain }			= require('@whi/holochain-backdrop');


const DNAREPO_PATH			= path.resolve( __dirname, "../bundled/dnarepo.dna" );
const WEBASSETS_PATH			= path.resolve( __dirname, "../bundled/web_assets.dna" );
const happs_yaml_path			= path.resolve( __dirname, "../bundled/happs/dna.yaml" );


(async function main () {
    const holochain			= new Holochain({
	"default_stdout_loggers": true,
    });
    await holochain.start();

    const admin				= new AdminClient( holochain.adminPorts()[0] );

    try {

	const config				= YAML.parse( fs.readFileSync( happs_yaml_path, "utf-8" ) );

	console.log("Before:", config );

	config.properties.dnarepo_hash		= String( await admin.registerDna( DNAREPO_PATH ) );
	config.properties.webassets_hash	= String( await admin.registerDna( WEBASSETS_PATH ) );

	console.log("After:", config );
	fs.writeFileSync( happs_yaml_path, YAML.stringify( config ) );
    } catch (err) {
	console.error( err );
    } finally {
	await admin.close();
	await holochain.destroy();
    }
})();
