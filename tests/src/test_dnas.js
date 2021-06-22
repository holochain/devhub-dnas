const path				= require('path');
const log				= require('@whi/stdlog')(path.basename( __filename ), {
    level: process.env.LOG_LEVEL || 'silly',
});


const fs				= require('fs');
const expect				= require('chai').expect;
const { Orchestrator,
	Config }			= require('@holochain/tryorama');
const Identicon				= require('identicon.js');
const { HoloHash }			= require('@whi/holo-hash');
const Essence				= require('@whi/essence');
const json				= require('@whi/json');

const { b64 }				= require('./utils.js');

const dna				= path.join(__dirname, "../../bundled/dnas/dnas.dna");
const storage_zome			= "storage";
const devhub_bundle			= [ dna ];

const conductorConfig			= Config.gen();

const agent_0_happs			= [ devhub_bundle ];
const agent_1_happs			= [ devhub_bundle ];
const agent_2_happs			= [ devhub_bundle ];

const delay				= ms => new Promise(f => setTimeout(f,ms));


const orchestrator			= new Orchestrator({
    "mode": {
	executor: { tape: require('tape') },
	spawning: 'local',
    },
});


function define_hidden_prop ( obj, key, value ) {
    Object.defineProperty( obj, key, {
	"value": value,
	"writable": false,
	"enumerable": false,
	"configurable": false,
    });
}

function Entity ( data ) {
    let content				= data.content;

    define_hidden_prop( content, "$id",		data.id );
    define_hidden_prop( content, "$address",	data.address );
    define_hidden_prop( content, "$addr",	data.address );

    return content;
}

function Collection ( data ) {
    let entities			= data.items.map(item => Entity( item ) );

    define_hidden_prop( data, "$base", data.base );

    return entities;
}

function Result ( msg, strict = false ) {
    let pack;
    try {
	pack				= Essence.parse( msg );
    } catch (err) {
	// If essence fails to parse and stict mode is on, throw.  Otherwise, assume the msg is the
	// payload content.
	if ( strict === true )
	    throw err;

	return msg;
    }

    let payload				= pack.value();
    let composition			= pack.metadata('composition');

    log.debug("Parsed msg value (composition: %s): %s", composition, typeof payload );
    if ( composition === "single" )
	return Entity( payload );
    else if ( composition === "collection" )
	return Collection( payload );
    else
	throw new Error(`Unknown composition: ${composition}`);
}

async function callZome ( client, fn_name, args ) {
    let response			= await client.call(storage_zome, fn_name, args );
    log.silly("Call Zome FULL Response: %s", json.debug(response) );

    return Result( response );
}


const dna_input				= {
    "name": "Game Turns",
    "description": "A tool for turn-based games to track the order of player actions",
};

orchestrator.registerScenario('Check uniqueness', async (scenario, _) => {
    const [a_and_b_conductor]		= await scenario.players([ conductorConfig ]);
    const [
	[ alice_devhub_happ ],
	[ bobby_devhub_happ ],
	[ carol_devhub_happ ],
    ]					= await a_and_b_conductor.installAgentsHapps([
	agent_0_happs,
	agent_1_happs,
	agent_2_happs,
    ]);

    const alice_devhub			= alice_devhub_happ.cells[0];
    const bobby_devhub			= bobby_devhub_happ.cells[0];
    const carol_devhub			= carol_devhub_happ.cells[0];


    let a_agent_info			= await alice_devhub.call(storage_zome, "whoami", null);
    log.info("Agent info 'alice': %s", json.debug(a_agent_info) );
    log.info("Agent ID 'alice': %s", a_agent_info.agent_initial_pubkey.toString("base64") );
    let b_agent_info			= await bobby_devhub.call(storage_zome, "whoami", null);
    let c_agent_info			= await carol_devhub.call(storage_zome, "whoami", null);

    let profile_hash;
    let profile_input			= {
	"name": "Zed Shaw",
	"email": "zed.shaw@example.com",
	"avatar_image": b64( (new Identicon(a_agent_info.agent_initial_pubkey.toString("hex"), 10)).toString() ),
    };

    {
	let [hash, profile_info]	= await alice_devhub.call(storage_zome, "create_profile", profile_input );
	log.normal("Set Developer profile: %s -> %s", b64(hash), json.debug(profile_info) );

	expect( profile_info.name	).to.equal( profile_input.name );

	profile_hash			= hash;
    }

    {
	let a_profile			= await alice_devhub.call(storage_zome, "get_profile", {} );
	log.normal("Alice profile: %s", json.debug(a_profile) );

	let failed			= false;
	try {
	    let b_profile		= await bobby_devhub.call(storage_zome, "get_profile", {} );
	    log.normal("Bobby profile: %s", json.debug(b_profile) );
	} catch (err) {
	    failed			= true;

	    expect( err.data.data	).to.have.string("has not been created yet");
	}
	expect( failed			).to.be.true;
    }

    {
	let header_hash			= await alice_devhub.call(storage_zome, "follow_developer", {
	    "agent": b_agent_info.agent_initial_pubkey,
	});
	log.normal("Following link hash: %s", b64(header_hash) );

	await alice_devhub.call(storage_zome, "follow_developer", {
	    "agent": c_agent_info.agent_initial_pubkey,
	});

	let following			= await alice_devhub.call(storage_zome, "get_following", null );
	log.normal("Following developers: %s", json.debug(following) );

	expect( following		).to.have.length( 2 );

	let delete_hash			= await alice_devhub.call(storage_zome, "unfollow_developer", {
	    "agent": c_agent_info.agent_initial_pubkey,
	});
	log.normal("Unfollowing link hash: %s", b64(delete_hash) );

	let updated_following		= await alice_devhub.call(storage_zome, "get_following", null );
	log.normal("Following developers: %s", json.debug(following) );

	expect( updated_following	).to.have.length( 1 );
    }

    {
	let profile_update_input	= {
	    "email": "zed.shaw@example.com",
	    "website": "zedshaw.example.com",
	};
	let [hash, profile_info]	= await alice_devhub.call(storage_zome, "update_profile", {
	    "addr": profile_hash,
	    "properties": profile_update_input,
	});
	log.normal("Updated Developer profile: %s -> %s", b64(hash), json.debug(profile_info) );

	expect( profile_info.name	).to.equal( profile_input.name );
	expect( profile_info.email	).to.equal( profile_update_input.email );
    }

    let [dna_hash, new_entry]		= await alice_devhub.call(storage_zome, "create_dna", dna_input );
    log.normal("New DNA (metadata): %s -> %s", b64(dna_hash), json.debug(new_entry) );

    {
	// Check the created entry
	let dna_info			= await callZome( alice_devhub, "get_dna", {
	    "addr": dna_hash,
	});
	console.log("Result: %s", json.debug(dna_info) );

	expect( dna_info.name		).to.equal( dna_input.name );
	expect( dna_info.description	).to.equal( dna_input.description );
    }


    const dna_bytes			= fs.readFileSync( path.resolve(__dirname, "../tiny.dna") );
    log.debug("DNA file bytes (%s): typeof %s", dna_bytes.length, typeof dna_bytes );

    let chunk_size			= (2**20 /*1 megabyte*/) * 2;
    let dna_version_hash;
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
	    log.info("Chunk %s/%s hash: %s", i+1, chunk_count, json.debug(chunk_hash) );

	    chunk_hashes.push( chunk_hash );
	}
	console.log("Final chunks:", chunk_hashes );

	let [version_hash, version]	= await alice_devhub.call(storage_zome, "create_dna_version", {
	    "for_dna": dna_hash,
	    "version": 1,
	    "file_size": dna_bytes.length,
	    "chunk_addresses": chunk_hashes,
	});
	log.normal("New DNA version: %s -> %s", b64(version_hash), json.debug(version) );
	dna_version_hash		= version_hash;
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
	    log.info("Chunk %s/%s hash: %s", i+1, chunk_count, json.debug(chunk_hash) );

	    chunk_hashes.push( chunk_hash );
	}
	console.log("Final chunks:", chunk_hashes );

	let [version_hash, version]	= await alice_devhub.call(storage_zome, "create_dna_version", {
	    "for_dna": dna_hash,
	    "version": 2,
	    "file_size": dna_bytes.length,
	    "chunk_addresses": chunk_hashes,
	});
	log.normal("New DNA version: %s -> %s", b64(version_hash), json.debug(version) );
    }

    {
	let dna_versions		= await alice_devhub.call(storage_zome, "get_dna_versions", {
	    "for_dna": dna_hash,
	});
	console.log( json.debug(dna_versions) );

	log.normal("Version list (%s): %s", dna_versions.length, json.debug(dna_versions.map(([_,v]) => {
	    return `DnaVersion { version: ${v.version}, file_size: ${v.file_size}, published_at: ${v.published_at} }`;
	})) );

	expect( dna_versions		).to.have.length( 2 );
    }

    {
	let dnas			= await callZome( alice_devhub, "get_my_dnas", null);
	console.log( json.debug(dnas) );

	log.normal("DNA list (%s): %s", dnas.length, json.debug(dnas.map( dna => {
	    return `Dna { name: ${dna.name}, published_at: ${dna.published_at} }`;
	})) );

	expect( dnas			).to.have.length( 1 );

	let b_dnas			= await callZome( alice_devhub, "get_dnas", {
	    "agent": b_agent_info.agent_initial_pubkey,
	});
	log.normal("Bobby DNAs: %s", json.debug(b_dnas) );
	expect( b_dnas			).to.have.length( 0 );
    }

    {
	// Update DNA
	const dna_name			= "Game Turns (new)";
	let [updated_dna_hash, dna]	= await alice_devhub.call(storage_zome, "update_dna", {
	    "addr": dna_hash,
	    "properties": {
		"name": dna_name,
	    }
	});
	log.normal("Updated DNA (metadata): %s -> %s", b64(updated_dna_hash), json.debug(dna) );

	let dna_info			= await callZome( alice_devhub, "get_dna", {
	    "addr": dna_hash,
	});
	console.log("Result: %s", json.debug(dna_info) );

	expect( dna_info.name		).to.equal( dna_name );
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
	let [updated_dna_version_hash, dna_version]	= await alice_devhub.call(storage_zome, "update_dna_version", {
	    "addr": dna_version_hash,
	    "properties": properties,
	});
	log.normal("Updated DNA Version (metadata): %s -> %s", b64(updated_dna_version_hash), json.debug(dna_version) );

	let dna_version_info		= await alice_devhub.call(storage_zome, "get_dna_version", {
	    "addr": dna_version_hash,
	});
	console.log( dna_version_info );
	expect( dna_version_info.changelog	).to.equal( properties.changelog );
	expect( dna_version_info.contributors	).to.have.length( 3 );
    }

    {
	// Unpublish DNA Version
	let deleted_dna_version_hash	= await alice_devhub.call(storage_zome, "delete_dna_version", {
	    "addr": dna_version_hash,
	});
	log.normal("Deleted DNA Version hash: %s", b64(deleted_dna_version_hash) );

	let dna_versions		= await alice_devhub.call(storage_zome, "get_dna_versions", {
	    "for_dna": dna_hash,
	});
	expect( dna_versions		).to.have.length( 1 );
    }

    {
	// Deprecate DNA
	let deprecation_notice		= "No longer maintained";
	let [deprecated_dna_hash, dna]	= await alice_devhub.call(storage_zome, "deprecate_dna", {
	    "addr": dna_hash,
	    "message": deprecation_notice,
	});
	log.normal("Deprecated DNA (metadata): %s -> %s", b64(deprecated_dna_hash), json.debug(dna) );

	let dna_info			= await callZome( alice_devhub, "get_dna", {
	    "addr": dna_hash,
	});
	console.log("Result: %s", json.debug(dna_info) );
	expect( dna_info.deprecation.message	).to.equal( deprecation_notice );

	let dnas			= await callZome( alice_devhub, "get_my_dnas", null);
	expect( dnas			).to.have.length( 0 );
    }

    {
	let dnas			= await callZome( alice_devhub, "get_my_deprecated_dnas", null);
	console.log( json.debug(dnas) );

	log.normal("Deprecated DNA list (%s): %s", dnas.length, json.debug(dnas.map( dna => {
	    return `Dna { name: ${dna.name}, published_at: ${dna.published_at} }`;
	})) );

	expect( dnas			).to.have.length( 1 );
    }
});

orchestrator.run();
