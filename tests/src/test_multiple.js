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
const webassets_dna			= path.join(__dirname, "../../bundled/web_assets/web_assets.dna");

const dna_list				= [ happs_dna, webassets_dna ];

const store				= "store";
const files				= "files";


orchestrator.registerScenario('WebAssets::files API', async (scenario, _) => {
    const [ alice ]			= await create_players( scenario, dna_list, ["alice"] );
    const [ happ_client,
	    asset_client ]		= alice.cells;

    log.warn("happs      DNA hash: %s", String(happ_client.id.dna) );
    log.warn("web_assets DNA hash: %s", String(asset_client.id.dna) );


    let a_agent_info			= await asset_client( files, "whoami", null);
    log.info("Agent ID 'alice': %s", String(new HoloHash(a_agent_info.agent_initial_pubkey)) );


    const file_bytes			= fs.readFileSync( path.resolve(__dirname, "../test.gz") );
    log.debug("GZ file bytes (%s): typeof %s", file_bytes.length, typeof file_bytes );

    let chunk_size			= (2**20 /*1 megabyte*/) * 2;
    let gz_file_hash;
    {
	let chunk_hashes		= [];
	let chunk_count			= Math.ceil( file_bytes.length / chunk_size );
	for (let i=0; i < chunk_count; i++) {
	    let chunk			= await asset_client( files, "create_file_chunk", {
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

	let file			= await asset_client( files, "create_file", {
	    "file_size": file_bytes.length,
	    "chunk_addresses": chunk_hashes,
	});
	log.normal("New GZ file: %s -> %s", String(file.$address), file.file_size );
	gz_file_hash			= file.$address;
    }

    {
	let ui				= await happ_client( store, "get_ui", {
	    "id": gz_file_hash,
	});
	log.normal("Updated hApp UI: %s", ui.file_size );

	expect( ui.file_size		).to.equal( 802067 );
    }
});

orchestrator.run();
