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


const webassets_dna			= path.join(__dirname, "../../bundled/web_assets/web_assets.dna");
const dna_list				= [ webassets_dna ];
const zome				= "files";


orchestrator.registerScenario('WebAssets::files API', async (scenario, _) => {
    const [ alice_file,
	    bobby_file,
	    carol_file ]		= await create_players( scenario, dna_list, ["alice", "bobby", "carol"] );

    const [ alice_client ]		= alice_file.cells;
    const [ bobby_client ]		= bobby_file.cells;
    const [ carol_client ]		= carol_file.cells;


    let a_agent_info			= await alice_client( zome, "whoami", null);
    log.info("Agent ID 'alice': %s", String(new HoloHash(a_agent_info.agent_initial_pubkey)) );


    const file_bytes			= fs.readFileSync( path.resolve(__dirname, "../test.gz") );
    log.debug("GZ file bytes (%s): typeof %s", file_bytes.length, typeof file_bytes );

    let chunk_size			= (2**20 /*1 megabyte*/) * 2;
    let gz_file_hash;
    {
	let chunk_hashes		= [];
	let chunk_count			= Math.ceil( file_bytes.length / chunk_size );
	for (let i=0; i < chunk_count; i++) {
	    let chunk			= await alice_client( zome, "create_file_chunk", {
		"sequence": {
		    "position": i+1,
		    "length": chunk_count,
		},
		"bytes": file_bytes.slice( i*chunk_size, (i+1)*chunk_size ),
	    });
	    log.info("Chunk %s/%s hash: %s", i+1, chunk_count, String(chunk.$address) );

	    chunk_hashes.push( chunk.$address );
	}
	log.debug("Final chunks:", json.debug(chunk_hashes) );

	let version			= await alice_client( zome, "create_file", {
	    "file_size": file_bytes.length,
	    "chunk_addresses": chunk_hashes,
	});
	log.normal("New GZ version: %s -> %s", String(version.$address), version.version );
	gz_file_hash			= version.$address;
    }
});

orchestrator.run();
