const path			= require('path');
const log			= require('@whi/stdlog')(path.basename( __filename ), {
    level: process.env.LOG_LEVEL || 'silly',
});


const fs			= require('fs');
const expect			= require('chai').expect;
const { Orchestrator,
	Config }		= require('@holochain/tryorama');

const { json, jsonraw,
	b64 }			= require('./utils.js');

const dna			= path.join(__dirname, "../../bundled/dnas/dnas.dna");
const storage_zome		= "storage";
const devhub_bundle		= [ dna ];

const conductorConfig		= Config.gen();

const agent_0_happs		= [ devhub_bundle ];
const agent_1_happs		= [ devhub_bundle ];
const agent_2_happs		= [ devhub_bundle ];

const delay			= ms => new Promise(f => setTimeout(f,ms));


const orchestrator		= new Orchestrator({
    "mode": {
	executor: { tape: require('tape') },
	spawning: 'local',
    },
});


const dna_input			= {
    "name": "Game Turns",
    "description": "A tool for turn-based games to track the order of player actions",
};

orchestrator.registerScenario('Check uniqueness', async (scenario, _) => {
    const [a_and_b_conductor]	= await scenario.players([ conductorConfig ]);
    const [
	[ alice_devhub_happ ],
	[ bobbo_devhub_happ ],
	[ candy_devhub_happ ],
    ]				= await a_and_b_conductor.installAgentsHapps([
	agent_0_happs,
	agent_1_happs,
	agent_2_happs,
    ]);

    const alice_devhub			= alice_devhub_happ.cells[0];
    const bobbo_devhub			= bobbo_devhub_happ.cells[0];
    const candy_devhub			= candy_devhub_happ.cells[0];


    let a_agent_info			= await alice_devhub.call(storage_zome, "whoami", null);
    log.info("Agent info 'alice': %s", jsonraw(a_agent_info) );
    log.info("Agent ID 'alice': %s", a_agent_info.agent_initial_pubkey.toString("base64") );

    let [dna_hash, new_entry]		= await alice_devhub.call(storage_zome, "create_dna", dna_input );
    log.normal("New DNA (metadata): %s -> %s", b64(dna_hash), jsonraw(new_entry) );

    {
	// Check the created entry
	let dna_info			= await alice_devhub.call(storage_zome, "get_dna", {
	    "addr": dna_hash,
	});
	expect( dna_info.name		).to.equal( dna_input.name );
	expect( dna_info.description	).to.equal( dna_input.description );
    }


    const dna_bytes			= fs.readFileSync( path.resolve(__dirname, "../test.dna") );
    log.debug("DNA file bytes (%s): typeof %s", dna_bytes.length, typeof dna_bytes );

    let chunk_size			= (2**20 /*1 megabyte*/) * 2;
    {
	let chunk_hashes		= [];
	let chunk_count			= Math.ceil( dna_bytes.length / chunk_size );
	for (let i=0; i < chunk_count; i++) {
	    let [chunk_hash, chunk]	= await alice_devhub.call(storage_zome, "create_dna_chunk", {
		"sequence": {
		    "position": i+1,
		    "length": chunk_count,
		},
		"bytes": dna_bytes.slice( i*chunk_size, (i+1)*chunk_size ),
	    });
	    log.info("Chunk %s/%s hash: %s", i+1, chunk_count, jsonraw(chunk_hash) );

	    chunk_hashes.push( chunk_hash );
	}
	console.log("Final chunks:", chunk_hashes );

	let [version_hash, version]	= await alice_devhub.call(storage_zome, "create_dna_version", {
	    "for_dna": dna_hash,
	    "version": 1,
	    "file_size": dna_bytes.length,
	    "chunk_addresses": chunk_hashes,
	});
	log.normal("New DNA version: %s -> %s", b64(version_hash), jsonraw(version) );
    }

    const bigdna_bytes			= Buffer.concat( Array(5).fill(dna_bytes) );
    log.debug("Big DNA file bytes (%s): typeof %s", bigdna_bytes.length, typeof bigdna_bytes );

    {
	let chunk_hashes		= [];
	let chunk_count			= Math.ceil( bigdna_bytes.length / chunk_size );
	for (let i=0; i < chunk_count; i++) {
	    let [chunk_hash, chunk]	= await alice_devhub.call(storage_zome, "create_dna_chunk", {
		"sequence": {
		    "position": i+1,
		    "length": chunk_count,
		},
		"bytes": bigdna_bytes.slice( i*chunk_size, (i+1)*chunk_size ),
	    });
	    log.info("Chunk %s/%s hash: %s", i+1, chunk_count, jsonraw(chunk_hash) );

	    chunk_hashes.push( chunk_hash );
	}
	console.log("Final chunks:", chunk_hashes );

	let [version_hash, version]	= await alice_devhub.call(storage_zome, "create_dna_version", {
	    "for_dna": dna_hash,
	    "version": 2,
	    "file_size": dna_bytes.length,
	    "chunk_addresses": chunk_hashes,
	});
	log.normal("New DNA version: %s -> %s", b64(version_hash), jsonraw(version) );
    }

    let dna_versions			= await alice_devhub.call(storage_zome, "get_dna_versions", {
	"for_dna": dna_hash,
    });
    log.normal("Version list (%s): %s", dna_versions.length, json(dna_versions.map(v => {
	return `DnaVersion { version: ${v.version}, file_size: ${v.file_size}, chunks: (${v.chunk_addresses.length})[..] }`;
    })) );

    expect( dna_versions		).to.have.length( 2 );


    let dnas				= await alice_devhub.call(storage_zome, "get_my_dnas", null);
    log.normal("DNA list (%s): %s", dnas.length, json(dnas.map(v => {
	return `Dna { name: ${v.name}, published_at: ${v.published_at} }`;
    })) );

    expect( dnas			).to.have.length( 1 );
});

orchestrator.run();
