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


const dnarepo_dna			= path.join(__dirname, "../../bundled/dnarepo/dnarepo.dna");
const happs_dna				= path.join(__dirname, "../../bundled/happs/happs.dna");
const webassets_dna			= path.join(__dirname, "../../bundled/web_assets/web_assets.dna");

const dna_list				= [ dnarepo_dna, happs_dna, webassets_dna ];

const storage				= "storage";
const store				= "store";
const files				= "files";

const chunk_size			= (2**20 /*1 megabyte*/) * 2;

orchestrator.registerScenario('WebAssets::files API', async (scenario, _) => {
    const [ alice ]			= await create_players( scenario, dna_list, ["alice"] );
    const [ dnarepo_client,
	    happ_client,
	    asset_client ]		= alice.cells;

    log.warn("dnarepo    DNA hash: %s", String(dnarepo_client.id.dna) );
    log.warn("happs      DNA hash: %s", String(happ_client.id.dna) );
    log.warn("web_assets DNA hash: %s", String(asset_client.id.dna) );


    let a_agent_info			= await asset_client( files, "whoami", null);
    log.info("Agent ID 'alice': %s", String(new HoloHash(a_agent_info.agent_initial_pubkey)) );


    const file_bytes			= fs.readFileSync( path.resolve(__dirname, "../test.gz") );
    log.debug("GZ file bytes (%s): typeof %s", file_bytes.length, typeof file_bytes );

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
	let gui				= await happ_client( store, "get_gui", {
	    "dna_hash": asset_client.id.dna,
	    "id": gz_file_hash,
	});
	log.normal("Updated hApp UI: %s", gui.file_size );

	expect( gui.file_size		).to.equal( 802067 );
    }


    const dna_input			= {
	"name": "Game Turns",
	"description": "A tool for turn-based games to track the order of player actions",
    };
    let dna				= await dnarepo_client( storage, "create_dna", dna_input );
    log.normal("New DNA (metadata): %s -> %s", String(dna.$id), dna.name );

    const dna_bytes			= fs.readFileSync( path.resolve(__dirname, "../test.dna") );
    log.debug("DNA file bytes (%s): typeof %s", dna_bytes.length, typeof dna_bytes );

    let chunk_hashes			= [];
    {
	let chunk_count			= Math.ceil( dna_bytes.length / chunk_size );
	for (let i=0; i < chunk_count; i++) {
	    let chunk			= await dnarepo_client( storage, "create_dna_chunk", {
		"sequence": {
		    "position": i+1,
		    "length": chunk_count,
		},
		"bytes": dna_bytes.slice( i*chunk_size, (i+1)*chunk_size ),
	    });
	    log.info("Chunk %s/%s hash: %s", i+1, chunk_count, String(chunk.$address) );

	    chunk_hashes.push( chunk.$address );
	}
	log.debug("Final chunks:", json.debug(chunk_hashes) );
    }

    let version			= await dnarepo_client( storage, "create_dna_version", {
	"for_dna": dna.$id,
	"version": 1,
	"file_size": dna_bytes.length,
	"chunk_addresses": chunk_hashes,
    });
    log.normal("New DNA version: %s -> %s", String(version.$address), version.version );


    let happ_input			= {
	"title": "Chess",
	"subtitle": "Super fun board game",
	"description": "Play chess with friends :)",
	"gui": {
	    "asset_group_id": gz_file_hash,
	    "uses_web_sdk": false,
	}
    };

    let happ				= await happ_client( store, "create_happ", happ_input );
    log.normal("New hApp: %s", String(happ.$addr) );

    expect( happ.description		).to.equal( happ_input.description );


    const manifest_yaml			= fs.readFileSync( path.resolve(__dirname, "../test_happ.yaml"), "utf8" );
    let release_input			= {
	"name": "v0.1.0",
	"description": "The first release",
	"for_happ": happ.$id,
	manifest_yaml,
	"resources": {
	    "test_dna": version.$id,
	},
    };

    let release				= await happ_client( store, "create_happ_release", release_input );
    log.normal("New hApp release: %s -> %s", String(release.$addr), release.name );

    expect( release.description		).to.equal( release_input.description );


    {
	let happ_package		= await happ_client( store, "get_release_package", {
	    "id": release.$id,
	    "dnarepo_dna_hash": dnarepo_client.id.dna,
	});
	log.normal("hApp release package bytes: (%s) %s", happ_package.constructor.name, happ_package.length );

	expect( happ_package.length	).to.equal( 899277 );

	fs.writeFileSync( path.resolve(__dirname, "../multitesting.happ"), Buffer.from(happ_package) );
    }
});

orchestrator.run();
