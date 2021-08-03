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
const storage				= "dna_library";
const mm_zome				= "mere_memory";

let clients;
let zome_version_1;
let zome_version_2;
let dna_version_hash;

function basic_tests () {
    const zome_bytes			= fs.readFileSync( path.resolve(__dirname, "../../zomes/target/wasm32-unknown-unknown/release/mere_memory.wasm") );
    const bigzome_bytes			= Buffer.concat( Array(5).fill(zome_bytes) );

    it("should get whoami info", async function () {
	let whoami			= await clients.alice.dnarepo.call( storage, "whoami" );

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
	    let profile_info		= await alice.call( storage, "create_profile", profile_input );
	    log.normal("Set Developer profile: %s -> %s", String(profile_info.$addr), profile_info.name );

	    expect( profile_info.name	).to.equal( profile_input.name );

	    profile_hash		= profile_info.$id;
	}

	{
	    let a_profile		= await alice.call( storage, "get_profile", {} );
	    log.normal("Alice profile: %s", a_profile.name );
	}

	{
	    let header_hash		= await alice.call( storage, "follow_developer", {
		"agent": clients.bobby,
	    });
	    log.normal("Following link hash: %s", String(new HoloHash(header_hash)) );

	    await alice.call( storage, "follow_developer", {
		"agent": clients.carol,
	    });

	    let following		= await alice.call( storage, "get_following", null );
	    log.normal("Following %s developers", following.length );

	    expect( following		).to.have.length( 2 );

	    let delete_hash		= await alice.call( storage, "unfollow_developer", {
		"agent": clients.carol,
	    });
	    log.normal("Unfollowing link hash: %s", String(new HoloHash(delete_hash)) );

	    await delay(200);

	    let updated_following	= await alice.call( storage, "get_following", null );
	    log.normal("Following %s developers", following.length );

	    expect( updated_following	).to.have.length( 1 );
	}

	{
	    let profile_update_input	= {
		"email": "zed.shaw@example.com",
		"website": "zedshaw.example.com",
	    };
	    let profile_info		= await alice.call( storage, "update_profile", {
		"addr": profile_hash,
		"properties": profile_update_input,
	    });
	    log.normal("Updated Developer profile: %s -> %s", String(profile_info.$addr), profile_info.name );

	    expect( profile_info.name	).to.equal( profile_input.name );
	    expect( profile_info.email	).to.equal( profile_update_input.email );
	}
    });

    it("should CRUD Zome and ZomeVersion", async function () {
	this.timeout( 30_000 );

	const alice			= clients.alice.dnarepo;
	const bobby			= clients.bobby.dnarepo;
	const carol			= clients.carol.dnarepo;

	let zome_input			= {
	    "name": "File Storage",
	    "description": "A generic API for fs-like data management",
	};

	let new_entry			= await alice.call( storage, "create_zome", zome_input );
	let main_zome			= new_entry;
	log.normal("New ZOME (metadata): %s -> %s", String(main_zome.$id), new_entry.name );

	let first_header_hash;
	{
	    // Check the created entry
	    let zome_info		= await alice.call( storage, "get_zome", {
		"id": main_zome.$id,
	    });
	    log.info("ZOME: %s", zome_info.name );

	    expect( zome_info.name		).to.equal( zome_input.name );
	    expect( zome_info.description	).to.equal( zome_input.description );

	    first_header_hash		= zome_info.$header;
	}

	{
	    log.debug("ZOME file bytes (%s): typeof %s", zome_bytes.length, typeof zome_bytes );
	    let version			= await alice.call( storage, "create_zome_version", {
		"for_zome": main_zome.$id,
		"version": 1,
		"zome_bytes": zome_bytes,
	    });
	    log.normal("New ZOME version: %s -> %s", String(version.$address), version.version );

	    zome_version_1		= version;
	}

	{
	    log.debug("Big ZOME file bytes (%s): typeof %s", bigzome_bytes.length, typeof bigzome_bytes );
	    let version			= await alice.call( storage, "create_zome_version", {
		"for_zome": main_zome.$id,
		"version": 2,
		"zome_bytes": bigzome_bytes,
	    });
	    log.normal("New ZOME version: %s -> %s", String(version.$address), version.version );

	    zome_version_2		= version;
	}

	{
	    let zome_versions		= await alice.call( storage, "get_zome_versions", {
		"for_zome": main_zome.$id,
	    });
	    log.info("ZOME Versions: %s", zome_versions.version );

	    log.normal("Version list (%s):", zome_versions.length,  );
	    zome_versions.forEach( v => {
		log.normal("  - ZomeVersion { version: %s, published_at: %s }", v.version, v.published_at );
	    });

	    expect( zome_versions	).to.have.length( 2 );
	}

	{
	    let zomes			= await alice.call( storage, "get_my_zomes", null);
	    log.info("My ZOMEs: %s", zomes.length );

	    log.normal("ZOME list (%s):", zomes.length,  );
	    zomes.forEach( v => {
		log.normal("  - Zome { name: %s, published_at: %s }", v.name, v.published_at );
	    });

	    expect( zomes		).to.have.length( 1 );

	    let b_zomes			= await alice.call( storage, "get_zomes", {
		"agent": clients.bobby,
	    });
	    log.normal("Bobby ZOMEs: %s", b_zomes.length );
	    expect( b_zomes		).to.have.length( 0 );
	}

	let second_header_hash;
	{
	    // Update ZOME
	    const zome_name		= "Game Turns (new)";
	    let zome			= await alice.call( storage, "update_zome", {
		"id": main_zome.$id,
		"addr": main_zome.$addr,
		"properties": {
		    "name": zome_name,
		}
	    });
	    expect( zome.$header		).to.not.deep.equal( first_header_hash );
	    log.normal("Updated ZOME (metadata): %s -> %s", String(zome.$addr), zome.name );

	    let zome_info		= await alice.call( storage, "get_zome", {
		"id": main_zome.$id,
	    });
	    log.info("ZOME post update: %s", zome_info.name );

	    expect( zome_info.name	).to.equal( zome_name );
	    expect( zome_info.$header	).to.not.deep.equal( first_header_hash );

	    second_header_hash		= zome.$header;
	}

	{
	    // Update ZOME Version
	    const properties		= {
		"changelog": "# Changelog\nFeatures\n...",
	    };
	    let zome_version		= await alice.call( storage, "update_zome_version", {
		"addr": zome_version_1.$id,
		"properties": properties,
	    });
	    log.normal("Updated ZOME Version (metadata): %s -> %s", String(zome_version.$address), zome_version.version );

	    let zome_version_info	= await alice.call( storage, "get_zome_version", {
		"id": zome_version_1.$id,
	    });
	    log.info("ZOME Version post update: %s", zome_version_info.version );
	    expect( zome_version_info.changelog		).to.equal( properties.changelog );
	}

	{
	    // Unpublish ZOME Version
	    let deleted_zome_version_hash	= await alice.call( storage, "delete_zome_version", {
		"id": zome_version_1.$id,
	    });
	    log.normal("Deleted ZOME Version hash: %s", String(new HoloHash(deleted_zome_version_hash)) );

	    let zome_versions		= await alice.call( storage, "get_zome_versions", {
		"for_zome": main_zome.$id,
	    });
	    expect( zome_versions	).to.have.length( 1 );
	}

	{
	    // Deprecate ZOME
	    let deprecation_notice	= "No longer maintained";
	    let zome			= await alice.call( storage, "deprecate_zome", {
		"addr": main_zome.$addr,
		"message": deprecation_notice,
	    });
	    log.normal("Deprecated ZOME (metadata): %s -> %s", String(zome.$addr), zome.name );

	    expect( zome.$header		).to.not.deep.equal( second_header_hash );

	    let zome_info		= await alice.call( storage, "get_zome", {
		"id": main_zome.$id,
	    });
	    log.info("ZOME post deprecation: %s", zome_info.name );
	    expect( zome_info.deprecation.message	).to.equal( deprecation_notice );
	    expect( zome_info.$header			).to.not.deep.equal( second_header_hash );

	    let zomes			= await alice.call( storage, "get_my_zomes", null);
	    expect( zomes		).to.have.length( 0 );
	}
    });

    it("should CRUD Dna and DnaVersion", async function () {
	this.timeout( 30_000 );

	const alice			= clients.alice.dnarepo;
	const bobby			= clients.bobby.dnarepo;
	const carol			= clients.carol.dnarepo;

	let dna_input			= {
	    "name": "Game Turns",
	    "description": "A tool for turn-based games to track the order of player actions",
	};

	let new_entry			= await alice.call( storage, "create_dna", dna_input );
	let main_dna			= new_entry;
	log.normal("New DNA (metadata): %s -> %s", String(main_dna.$id), new_entry.name );

	let first_header_hash;
	{
	    // Check the created entry
	    let dna_info		= await alice.call( storage, "get_dna", {
		"id": main_dna.$id,
	    });
	    log.info("DNA: %s", dna_info.name );

	    expect( dna_info.name		).to.equal( dna_input.name );
	    expect( dna_info.description	).to.equal( dna_input.description );

	    first_header_hash		= dna_info.$header;
	}

	{
	    let version			= await alice.call( storage, "create_dna_version", {
		"for_dna": main_dna.$id,
		"version": 1,
		"zomes": [{
		    "name": "mere_memory",
		    "zome": zome_version_1.for_zome.$id,
		    "version": zome_version_1.$id,
		    "resource": zome_version_1.mere_memory_addr,
		}],
	    });
	    log.normal("New DNA version: %s -> %s", String(version.$address), version.version );

	    dna_version_hash		= version.$id;
	}

	{
	    let version			= await alice.call( storage, "create_dna_version", {
		"for_dna": main_dna.$id,
		"version": 2,
		"zomes": [{
		    "name": "mere_memory",
		    "zome": zome_version_2.for_zome.$id,
		    "version": zome_version_2.$id,
		    "resource": zome_version_2.mere_memory_addr,
		}],
	    });
	    log.normal("New DNA version: %s -> %s", String(version.$address), version.version );
	}

	{
	    let dna_versions		= await alice.call( storage, "get_dna_versions", {
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
	    let dnas			= await alice.call( storage, "get_my_dnas", null);
	    log.info("My DNAs: %s", dnas.length );

	    log.normal("DNA list (%s):", dnas.length,  );
	    dnas.forEach( v => {
		log.normal("  - Dna { name: %s, published_at: %s }", v.name, v.published_at );
	    });

	    expect( dnas		).to.have.length( 1 );

	    let b_dnas			= await alice.call( storage, "get_dnas", {
		"agent": clients.bobby,
	    });
	    log.normal("Bobby DNAs: %s", b_dnas.length );
	    expect( b_dnas		).to.have.length( 0 );
	}

	let second_header_hash;
	{
	    // Update DNA
	    const dna_name		= "Game Turns (new)";
	    let dna			= await alice.call( storage, "update_dna", {
		"id": main_dna.$id,
		"addr": main_dna.$addr,
		"properties": {
		    "name": dna_name,
		}
	    });
	    expect( dna.$header		).to.not.deep.equal( first_header_hash );
	    log.normal("Updated DNA (metadata): %s -> %s", String(dna.$addr), dna.name );

	    let dna_info		= await alice.call( storage, "get_dna", {
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
	    let dna_version		= await alice.call( storage, "update_dna_version", {
		"addr": dna_version_hash,
		"properties": properties,
	    });
	    log.normal("Updated DNA Version (metadata): %s -> %s", String(dna_version.$address), dna_version.version );

	    let dna_version_info	= await alice.call( storage, "get_dna_version", {
		"id": dna_version_hash,
	    });
	    log.info("DNA Version post update: %s", dna_version_info.version );
	    expect( dna_version_info.changelog		).to.equal( properties.changelog );
	    expect( dna_version_info.contributors	).to.have.length( 3 );
	}

	{
	    let pack			= await alice.call( storage, "get_dna_package", {
		"id": dna_version_hash,
	    });
	    log.info("DNA Package bytes: %s", pack.bytes.length );

	    expect( pack.bytes.constructor.name		).to.equal("Uint8Array");
	}

	{
	    // Unpublish DNA Version
	    let deleted_dna_version_hash	= await alice.call( storage, "delete_dna_version", {
		"id": dna_version_hash,
	    });
	    log.normal("Deleted DNA Version hash: %s", String(new HoloHash(deleted_dna_version_hash)) );

	    let dna_versions		= await alice.call( storage, "get_dna_versions", {
		"for_dna": main_dna.$id,
	    });
	    expect( dna_versions	).to.have.length( 1 );
	}

	{
	    // Deprecate DNA
	    let deprecation_notice	= "No longer maintained";
	    let dna			= await alice.call( storage, "deprecate_dna", {
		"addr": main_dna.$addr,
		"message": deprecation_notice,
	    });
	    log.normal("Deprecated DNA (metadata): %s -> %s", String(dna.$addr), dna.name );

	    expect( dna.$header		).to.not.deep.equal( second_header_hash );

	    let dna_info		= await alice.call( storage, "get_dna", {
		"id": main_dna.$id,
	    });
	    log.info("DNA post deprecation: %s", dna_info.name );
	    expect( dna_info.deprecation.message	).to.equal( deprecation_notice );
	    expect( dna_info.$header			).to.not.deep.equal( second_header_hash );

	    let dnas			= await alice.call( storage, "get_my_dnas", null);
	    expect( dnas		).to.have.length( 0 );
	}
    });
}

function errors_tests () {
    it("should fail to get profile because it is not made yet", async function () {
	const bobby			= clients.bobby.dnarepo;

	{
	    let failed			= false;
	    try {
		let b_profile		= await bobby.call( storage, "get_profile", {} );
		log.normal("Bobby profile: %s", b_profile.name );
	    } catch (err) {
		failed			= true;

		expect( String(err)	).to.have.string("CustomError: Agent Profile has not been created yet");
	    }
	    expect( failed		).to.be.true;
	}
    });

    it("should fail to get deleted DNA version", async function () {
	const alice			= clients.alice.dnarepo;

	{
	    let failed			= false;
	    try {
		await alice.call( storage, "get_dna_version", {
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
    });

    it("should fail to create ZOME version because missing ZOME package info", async function () {
	const alice			= clients.alice.dnarepo;

	{
	    let failed			= false;
	    try {
		await alice.call( storage, "create_zome_version", {
		    "for_zome": new HoloHash("uhCEkvriXQtLwCt8urCSqAxS6MYUGPEVbb3h0CH0aVj4QVba1fEzj"),
		    "version": 1,
		    "file_size": 0,
		});
	    } catch (err) {
		expect( err.kind	).to.equal( "UserError" );
		expect( err.name	).to.equal( "CustomError" );
		expect( err.message	).to.have.string( "must supply an address or bytes" );

		failed			= true;
	    }

	    expect( failed		).to.be.true;
	}
    });
}

describe("DNArepo", () => {

    const holochain			= new Holochain();

    before(async function () {
	this.timeout( 30_000 );

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
