const path				= require('path');
const log				= require('@whi/stdlog')(path.basename( __filename ), {
    level: process.env.LOG_LEVEL || 'silly',
});


const fs				= require('fs');
const expect				= require('chai').expect;
const { HoloHash }			= require('@whi/holo-hash');
const json				= require('@whi/json');
const Identicon				= require('identicon.js');

const { delay, callZome,
	orchestrator,
	create_players }		= require('./utils.js');


const dna				= path.join(__dirname, "../../bundled/dnas/dnas.dna");
const dna_list				= [ dna ];
const zome				= "storage";



const dna_input				= {
    "name": "Game Turns",
    "description": "A tool for turn-based games to track the order of player actions",
};

orchestrator.registerScenario('dnas::storage API', async (scenario, _) => {
    const [ alice_happ,
	    bobby_happ,
	    carol_happ ]		= await create_players( scenario, dna_list, ["alice", "bobby", "carol"] );

    const [ alice_client ]		= alice_happ.cells;
    const [ bobby_client ]		= bobby_happ.cells;
    const [ carol_client ]		= carol_happ.cells;


    let a_agent_info			= await alice_client( zome, "whoami", null);
    log.info("Agent info 'alice': %s", json.debug(a_agent_info) );
    log.info("Agent ID 'alice': %s", a_agent_info.agent_initial_pubkey.toString("base64") );
    let b_agent_info			= await bobby_client( zome, "whoami", null);
    let c_agent_info			= await carol_client( zome, "whoami", null);

    let profile_hash;
    let profile_input			= {
	"name": "Zed Shaw",
	"email": "zed.shaw@example.com",
	"avatar_image": Buffer.from( (new Identicon(a_agent_info.agent_initial_pubkey.toString("hex"), 10)).toString(), "base64"),
    };

    {
	let profile_info		= await alice_client( zome, "create_profile", profile_input );
	log.normal("Set Developer profile: %s -> %s", String(profile_info.$addr), json.debug(profile_info) );

	expect( profile_info.name	).to.equal( profile_input.name );

	profile_hash			= profile_info.$id;
    }

    {
	let a_profile			= await alice_client( zome, "get_profile", {} );
	log.normal("Alice profile: %s", json.debug(a_profile) );

	let failed			= false;
	try {
	    let b_profile		= await bobby_client( zome, "get_profile", {} );
	    log.normal("Bobby profile: %s", json.debug(b_profile) );
	} catch (err) {
	    failed			= true;

	    expect( String(err)		).to.have.string("CustomError: Agent Profile has not been created yet");
	}
	expect( failed			).to.be.true;
    }

    {
	let header_hash			= await alice_client( zome, "follow_developer", {
	    "agent": b_agent_info.agent_initial_pubkey,
	});
	log.normal("Following link hash: %s", String(header_hash) );

	await alice_client( zome, "follow_developer", {
	    "agent": c_agent_info.agent_initial_pubkey,
	});

	let following			= await alice_client( zome, "get_following", null );
	log.normal("Following developers: %s", json.debug(following) );

	expect( following		).to.have.length( 2 );

	let delete_hash			= await alice_client( zome, "unfollow_developer", {
	    "agent": c_agent_info.agent_initial_pubkey,
	});
	log.normal("Unfollowing link hash: %s", String(delete_hash) );

	await delay(100);

	let updated_following		= await alice_client( zome, "get_following", null );
	log.normal("Following developers: %s", json.debug(following) );

	expect( updated_following	).to.have.length( 1 );
    }

    {
	let profile_update_input	= {
	    "email": "zed.shaw@example.com",
	    "website": "zedshaw.example.com",
	};
	let profile_info		= await alice_client( zome, "update_profile", {
	    "addr": profile_hash,
	    "properties": profile_update_input,
	});
	log.normal("Updated Developer profile: %s -> %s", String(profile_info.$addr), json.debug(profile_info) );

	expect( profile_info.name	).to.equal( profile_input.name );
	expect( profile_info.email	).to.equal( profile_update_input.email );
    }

    let new_entry			= await alice_client( zome, "create_dna", dna_input );
    let main_dna			= new_entry;
    log.normal("New DNA (metadata): %s -> %s", String(main_dna.$id), json.debug(new_entry) );

    let first_header_hash;
    {
	// Check the created entry
	let dna_info			= await alice_client( zome, "get_dna", {
	    "addr": main_dna.$id,
	});
	log.info("DNA: %s", json.debug(dna_info) );

	expect( dna_info.name		).to.equal( dna_input.name );
	expect( dna_info.description	).to.equal( dna_input.description );

	first_header_hash		= dna_info.$header;
    }


    const dna_bytes			= fs.readFileSync( path.resolve(__dirname, "../tiny.dna") );
    log.debug("DNA file bytes (%s): typeof %s", dna_bytes.length, typeof dna_bytes );

    let chunk_size			= (2**20 /*1 megabyte*/) * 2;
    let dna_version_hash;
    {
	let chunk_hashes		= [];
	let chunk_count			= Math.ceil( dna_bytes.length / chunk_size );
	for (let i=0; i < chunk_count; i++) {
	    let chunk			= await alice_client( zome, "create_dna_chunk", {
		"sequence": {
		    "position": i+1,
		    "length": chunk_count,
		},
		"bytes": dna_bytes.slice( i*chunk_size, (i+1)*chunk_size ),
	    });
	    log.info("Chunk %s/%s hash: %s", i+1, chunk_count, String(chunk.$address) );

	    chunk_hashes.push( chunk.$address );
	}
	console.log("Final chunks:", chunk_hashes );

	let version			= await alice_client( zome, "create_dna_version", {
	    "for_dna": main_dna.$id,
	    "version": 1,
	    "file_size": dna_bytes.length,
	    "chunk_addresses": chunk_hashes,
	});
	log.normal("New DNA version: %s -> %s", String(version.$address), json.debug(version) );
	dna_version_hash		= version.$address;
    }

    const bigdna_bytes			= Buffer.concat( Array(5).fill(dna_bytes) );
    log.debug("Big DNA file bytes (%s): typeof %s", bigdna_bytes.length, typeof bigdna_bytes );

    {
	let chunk_hashes		= [];
	let chunk_count			= Math.ceil( bigdna_bytes.length / chunk_size );
	for (let i=0; i < chunk_count; i++) {
	    let chunk			= await alice_client( zome, "create_dna_chunk", {
		"sequence": {
		    "position": i+1,
		    "length": chunk_count,
		},
		"bytes": bigdna_bytes.slice( i*chunk_size, (i+1)*chunk_size ),
	    });
	    log.info("Chunk %s/%s hash: %s", i+1, chunk_count, String(chunk.$address) );

	    chunk_hashes.push( chunk.$address );
	}
	console.log("Final chunks:", chunk_hashes );

	let version			= await alice_client( zome, "create_dna_version", {
	    "for_dna": main_dna.$id,
	    "version": 2,
	    "file_size": dna_bytes.length,
	    "chunk_addresses": chunk_hashes,
	});
	log.normal("New DNA version: %s -> %s", String(version.$address), json.debug(version) );
    }

    {
	let dna_versions		= await alice_client( zome, "get_dna_versions", {
	    "for_dna": main_dna.$id,
	});
	log.info("DNA Versions: %s", json.debug(dna_versions) );

	log.normal("Version list (%s):", dna_versions.length,  );
	dna_versions.forEach( v => {
	    log.normal("  - DnaVersion { version: %s, file_size: %s, published_at: %s }", v.version, v.file_size, v.published_at );
	});

	expect( dna_versions		).to.have.length( 2 );
    }

    {
	let dnas			= await alice_client( zome, "get_my_dnas", null);
	log.info("My DNAs: %s", json.debug(dnas) );

	log.normal("DNA list (%s):", dnas.length,  );
	dnas.forEach( v => {
	    log.normal("  - Dna { name: %s, published_at: %s }", v.name, v.published_at );
	});

	expect( dnas			).to.have.length( 1 );

	let b_dnas			= await alice_client( zome, "get_dnas", {
	    "agent": b_agent_info.agent_initial_pubkey,
	});
	log.normal("Bobby DNAs: %s", json.debug(b_dnas) );
	expect( b_dnas			).to.have.length( 0 );
    }

    let second_header_hash;
    {
	// Update DNA
	const dna_name			= "Game Turns (new)";
	let dna				= await alice_client( zome, "update_dna", {
	    "id": main_dna.$id,
	    "addr": main_dna.$addr,
	    "properties": {
		"name": dna_name,
	    }
	});
	expect( dna.$header		).to.not.deep.equal( first_header_hash );
	log.normal("Updated DNA (metadata): %s -> %s", String(dna.$addr), json.debug(dna) );

	let dna_info			= await alice_client( zome, "get_dna", {
	    "addr": main_dna.$id,
	});
	log.info("DNA post update: %s", json.debug(dna_info) );

	expect( dna_info.name		).to.equal( dna_name );
	expect( dna_info.$header	).to.not.deep.equal( first_header_hash );

	second_header_hash		= dna.$header;
    }

    {
	// Update DNA Version
	const properties		= {
	    "changelog": "# Changelog\nFeatures\n...",
            "contributors": [
		[ "kevin@open-games.example", null ],
		[ "stuart@open-games.example", b_agent_info.agent_initial_pubkey ],
		[ "bob@open-games.example", c_agent_info.agent_initial_pubkey ],
            ],
	};
	let dna_version			= await alice_client( zome, "update_dna_version", {
	    "addr": dna_version_hash,
	    "properties": properties,
	});
	log.normal("Updated DNA Version (metadata): %s -> %s", String(dna_version.$address), json.debug(dna_version) );

	let dna_version_info		= await alice_client( zome, "get_dna_version", {
	    "addr": dna_version_hash,
	});
	log.info("DNA Version post update: %s", json.debug(dna_version_info) );
	expect( dna_version_info.changelog	).to.equal( properties.changelog );
	expect( dna_version_info.contributors	).to.have.length( 3 );
    }

    {
	// Unpublish DNA Version
	let deleted_dna_version_hash	= await alice_client( zome, "delete_dna_version", {
	    "addr": dna_version_hash,
	});
	log.normal("Deleted DNA Version hash: %s", String(deleted_dna_version_hash) );

	let dna_versions		= await alice_client( zome, "get_dna_versions", {
	    "for_dna": main_dna.$id,
	});
	expect( dna_versions		).to.have.length( 1 );
    }

    {
	// Deprecate DNA
	let deprecation_notice		= "No longer maintained";
	let dna				= await alice_client( zome, "deprecate_dna", {
	    "addr": main_dna.$id,
	    "message": deprecation_notice,
	});
	log.normal("Deprecated DNA (metadata): %s -> %s", String(dna.$addr), json.debug(dna) );

	expect( dna.$header		).to.not.deep.equal( second_header_hash );

	let dna_info			= await alice_client( zome, "get_dna", {
	    "addr": main_dna.$id,
	});
	log.info("DNA post deprecation: %s", json.debug(dna_info) );
	expect( dna_info.deprecation.message	).to.equal( deprecation_notice );
	expect( dna_info.$header		).to.not.deep.equal( second_header_hash );

	let dnas			= await alice_client( zome, "get_my_dnas", null);
	expect( dnas			).to.have.length( 0 );
    }

    {
	let dnas			= await alice_client( zome, "get_my_deprecated_dnas", null);
	log.info("My deprecated DNAs: %s", json.debug(dnas) );

	log.normal("Deprecated DNA list (%s):", dnas.length,  );
	dnas.forEach( v => {
	    log.normal("  - Dna { name: %s, published_at: %s }", v.name, v.published_at );
	});

	expect( dnas			).to.have.length( 1 );
    }

    {
	let failed			= false;
	try {
	    await alice_client( zome, "get_dna", {
		"addr": dna_version_hash,
	    });
	} catch (err) {
	    console.error("Controlled failure:", err.toJSON() );

	    expect( err.kind		).to.equal( "UtilsError" );
	    expect( err.name		).to.equal( "EntryNotFoundError" );
	    expect( err.message		).to.have.string( "Entry not found for address: " );

	    failed			= true;
	}

	expect( failed			).to.be.true;
    }
});

orchestrator.run();
