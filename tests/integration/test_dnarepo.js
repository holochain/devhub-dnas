const path				= require('path');
const log				= require('@whi/stdlog')(path.basename( __filename ), {
    level: process.env.LOG_LEVEL || 'fatal',
});


const fs				= require('fs');
const expect				= require('chai').expect;
const Identicon				= require('identicon.js');
const { HoloHash }			= require('@whi/holo-hash');
const { Holochain }			= require('@whi/holochain-backdrop');
const json				= require('@whi/json');
const why				= require('why-is-node-running');

// setTimeout(() => {
//     console.log( why() );
// }, 6000 );

const { backdrop }			= require('./setup.js');

const delay				= (n) => new Promise(f => setTimeout(f, n));
const DNAREPO_PATH			= path.join( __dirname, "../../bundled/dnarepo/dnarepo.dna" );
const zome				= "storage";

let clients;

function basic_tests () {
    it("should get whoami info", async function () {
	let whoami			= await clients.alice.dnarepo.call("storage", "whoami");

	log.normal("Alice whoami: %s", whoami );
    });

    it("should manage developer profile", async function () {
	this.timeout( 10_000 );

	const alice			= clients.alice.dnarepo;
	const bobby			= clients.bobby.dnarepo;
	const carol			= clients.carol.dnarepo;

	let profile_hash;
	let profile_input		= {
	    "name": "Zed Shaw",
	    "email": "zed.shaw@example.com",
	    "avatar_image": Buffer.from( (new Identicon( Buffer.from( clients.alice ).toString("hex"), 10)).toString(), "base64"),
	};

	{
	    let profile_info		= await alice.call( zome, "create_profile", profile_input );
	    log.normal("Set Developer profile: %s -> %s", String(profile_info.$addr), profile_info.name );

	    expect( profile_info.name	).to.equal( profile_input.name );

	    profile_hash		= profile_info.$id;
	}

	{
	    let a_profile		= await alice.call( zome, "get_profile", {} );
	    log.normal("Alice profile: %s", a_profile.name );

	    let failed			= false;
	    try {
		let b_profile		= await bobby.call( zome, "get_profile", {} );
		log.normal("Bobby profile: %s", b_profile.name );
	    } catch (err) {
		failed			= true;

		expect( String(err)	).to.have.string("CustomError: Agent Profile has not been created yet");
	    }
	    expect( failed		).to.be.true;
	}

	{
	    let header_hash		= await alice.call( zome, "follow_developer", {
		"agent": clients.bobby,
	    });
	    log.normal("Following link hash: %s", String(new HoloHash(header_hash)) );

	    await alice.call( zome, "follow_developer", {
		"agent": clients.carol,
	    });

	    let following		= await alice.call( zome, "get_following", null );
	    log.normal("Following %s developers", following.length );

	    expect( following		).to.have.length( 2 );

	    let delete_hash		= await alice.call( zome, "unfollow_developer", {
		"agent": clients.carol,
	    });
	    log.normal("Unfollowing link hash: %s", String(new HoloHash(delete_hash)) );

	    await delay(200);

	    let updated_following	= await alice.call( zome, "get_following", null );
	    log.normal("Following %s developers", following.length );

	    expect( updated_following	).to.have.length( 1 );
	}

	{
	    let profile_update_input	= {
		"email": "zed.shaw@example.com",
		"website": "zedshaw.example.com",
	    };
	    let profile_info		= await alice.call( zome, "update_profile", {
		"addr": profile_hash,
		"properties": profile_update_input,
	    });
	    log.normal("Updated Developer profile: %s -> %s", String(profile_info.$addr), profile_info.name );

	    expect( profile_info.name	).to.equal( profile_input.name );
	    expect( profile_info.email	).to.equal( profile_update_input.email );
	}
    });


    it("should CRUD Dna, DnaVersion, and DnaChunk", async function () {
	this.timeout( 10_000 );

	const alice			= clients.alice.dnarepo;
	const bobby			= clients.bobby.dnarepo;
	const carol			= clients.carol.dnarepo;

	let dna_input			= {
	    "name": "Game Turns",
	    "description": "A tool for turn-based games to track the order of player actions",
	};

	let new_entry			= await alice.call( zome, "create_dna", dna_input );
	let main_dna			= new_entry;
	log.normal("New DNA (metadata): %s -> %s", String(main_dna.$id), new_entry.name );

	let first_header_hash;
	{
	    // Check the created entry
	    let dna_info		= await alice.call( zome, "get_dna", {
		"id": main_dna.$id,
	    });
	    log.info("DNA: %s", dna_info.name );

	    expect( dna_info.name		).to.equal( dna_input.name );
	    expect( dna_info.description	).to.equal( dna_input.description );

	    first_header_hash		= dna_info.$header;
	}


	const dna_bytes			= fs.readFileSync( path.resolve(__dirname, "../test.dna") );
	log.debug("DNA file bytes (%s): typeof %s", dna_bytes.length, typeof dna_bytes );

	let chunk_size			= (2**20 /*1 megabyte*/) * 2;
	let dna_version_hash;
	{
	    let chunk_hashes		= [];
	    let chunk_count		= Math.ceil( dna_bytes.length / chunk_size );
	    for (let i=0; i < chunk_count; i++) {
		let chunk		= await alice.call( zome, "create_dna_chunk", {
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

	    let version			= await alice.call( zome, "create_dna_version", {
		"for_dna": main_dna.$id,
		"version": 1,
		"file_size": dna_bytes.length,
		"chunk_addresses": chunk_hashes,
	    });
	    log.normal("New DNA version: %s -> %s", String(version.$address), version.version );
	    dna_version_hash		= version.$address;
	}

	const bigdna_bytes		= Buffer.concat( Array(5).fill(dna_bytes) );
	log.debug("Big DNA file bytes (%s): typeof %s", bigdna_bytes.length, typeof bigdna_bytes );

	{
	    let chunk_hashes		= [];
	    let chunk_count		= Math.ceil( bigdna_bytes.length / chunk_size );
	    for (let i=0; i < chunk_count; i++) {
		let chunk		= await alice.call( zome, "create_dna_chunk", {
		    "sequence": {
			"position": i+1,
			"length": chunk_count,
		    },
		    "bytes": bigdna_bytes.slice( i*chunk_size, (i+1)*chunk_size ),
		});
		log.info("Chunk %s/%s hash: %s", i+1, chunk_count, String(chunk.$address) );

		chunk_hashes.push( chunk.$address );
	    }
	    log.debug("Final chunks:", json.debug(chunk_hashes) );

	    let version			= await alice.call( zome, "create_dna_version", {
		"for_dna": main_dna.$id,
		"version": 2,
		"file_size": dna_bytes.length,
		"chunk_addresses": chunk_hashes,
	    });
	    log.normal("New DNA version: %s -> %s", String(version.$address), version.version );
	}

	{
	    let dna_versions		= await alice.call( zome, "get_dna_versions", {
		"for_dna": main_dna.$id,
	    });
	    log.info("DNA Versions: %s", dna_versions.version );

	    log.normal("Version list (%s):", dna_versions.length,  );
	    dna_versions.forEach( v => {
		log.normal("  - DnaVersion { version: %s, file_size: %s, published_at: %s }", v.version, v.file_size, v.published_at );
	    });

	    expect( dna_versions	).to.have.length( 2 );
	}

	{
	    let dnas			= await alice.call( zome, "get_my_dnas", null);
	    log.info("My DNAs: %s", dnas.length );

	    log.normal("DNA list (%s):", dnas.length,  );
	    dnas.forEach( v => {
		log.normal("  - Dna { name: %s, published_at: %s }", v.name, v.published_at );
	    });

	    expect( dnas		).to.have.length( 1 );

	    let b_dnas			= await alice.call( zome, "get_dnas", {
		"agent": clients.bobby,
	    });
	    log.normal("Bobby DNAs: %s", b_dnas.length );
	    expect( b_dnas		).to.have.length( 0 );
	}

	let second_header_hash;
	{
	    // Update DNA
	    const dna_name		= "Game Turns (new)";
	    let dna			= await alice.call( zome, "update_dna", {
		"id": main_dna.$id,
		"addr": main_dna.$addr,
		"properties": {
		    "name": dna_name,
		}
	    });
	    expect( dna.$header		).to.not.deep.equal( first_header_hash );
	    log.normal("Updated DNA (metadata): %s -> %s", String(dna.$addr), dna.name );

	    let dna_info		= await alice.call( zome, "get_dna", {
		"id": main_dna.$id,
	    });
	    log.info("DNA post update: %s", dna_info.name );

	    expect( dna_info.name	).to.equal( dna_name );
	    expect( dna_info.$header	).to.not.deep.equal( first_header_hash );

	    second_header_hash		= dna.$header;
	}

	{
	    // Update DNA Version
	    const properties		= {
		"changelog": "# Changelog\nFeatures\n...",
		"contributors": [
		    [ "kevin@open-games.example", null ],
		    [ "stuart@open-games.example", clients.bobby ],
		    [ "bob@open-games.example", clients.carol ],
		],
	    };
	    let dna_version		= await alice.call( zome, "update_dna_version", {
		"addr": dna_version_hash,
		"properties": properties,
	    });
	    log.normal("Updated DNA Version (metadata): %s -> %s", String(dna_version.$address), dna_version.version );

	    let dna_version_info	= await alice.call( zome, "get_dna_version", {
		"id": dna_version_hash,
	    });
	    log.info("DNA Version post update: %s", dna_version_info.version );
	    expect( dna_version_info.changelog		).to.equal( properties.changelog );
	    expect( dna_version_info.contributors	).to.have.length( 3 );
	}

	{
	    let pack			= await alice.call( zome, "get_dna_package", {
		"id": dna_version_hash,
	    });
	    log.info("DNA Package bytes: %s", pack.bytes.length );

	    expect( pack.bytes.length	).to.equal( dna_bytes.length );
	}

	{
	    // Unpublish DNA Version
	    let deleted_dna_version_hash	= await alice.call( zome, "delete_dna_version", {
		"id": dna_version_hash,
	    });
	    log.normal("Deleted DNA Version hash: %s", String(new HoloHash(deleted_dna_version_hash)) );

	    let dna_versions		= await alice.call( zome, "get_dna_versions", {
		"for_dna": main_dna.$id,
	    });
	    expect( dna_versions	).to.have.length( 1 );

	    let failed			= false;
	    try {
		await alice.call( zome, "get_dna_version", {
		    "id": dna_version_hash,
		});
	    } catch (err) {
		expect( err.kind	).to.equal( "UserError" );
		expect( err.name	).to.equal( "EntryNotFoundError" );
		expect( err.message	).to.have.string( "Entry not found for address: " );

		failed			= true;
	    }

	    expect( failed		).to.be.true;
	}

	{
	    // Deprecate DNA
	    let deprecation_notice	= "No longer maintained";
	    let dna			= await alice.call( zome, "deprecate_dna", {
		"addr": main_dna.$addr,
		"message": deprecation_notice,
	    });
	    log.normal("Deprecated DNA (metadata): %s -> %s", String(dna.$addr), dna.name );

	    expect( dna.$header		).to.not.deep.equal( second_header_hash );

	    let dna_info		= await alice.call( zome, "get_dna", {
		"id": main_dna.$id,
	    });
	    log.info("DNA post deprecation: %s", dna_info.name );
	    expect( dna_info.deprecation.message	).to.equal( deprecation_notice );
	    expect( dna_info.$header			).to.not.deep.equal( second_header_hash );

	    let dnas			= await alice.call( zome, "get_my_dnas", null);
	    expect( dnas		).to.have.length( 0 );
	}
    });
}

function errors_tests () {
}

describe("DNArepon", () => {

    const holochain			= new Holochain();

    before(async function () {
	this.timeout( 5_000 );

	clients				= await backdrop( holochain, {
	    "dnarepo": DNAREPO_PATH,
	}, [
	    "alice",
	    "bobby",
	    "carol",
	]);
    });

    describe("Basic", basic_tests.bind( this, holochain ) );
    describe("Errors", errors_tests.bind( this, holochain ) );

    after(async () => {
	await holochain.stop();
	await holochain.destroy();
    });

});
