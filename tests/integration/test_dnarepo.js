const path				= require('path');
const log				= require('@whi/stdlog')(path.basename( __filename ), {
    level: process.env.LOG_LEVEL || 'fatal',
});


const fs				= require('fs');
const crypto				= require('crypto');
const expect				= require('chai').expect;
const Identicon				= require('identicon.js');
const msgpack				= require('@msgpack/msgpack');
const { EntryHash,
	HoloHash }			= require('@whi/holo-hash');
const { Holochain,
	HolochainClientLib }		= require('@whi/holochain-backdrop');
const json				= require('@whi/json');
const why				= require('why-is-node-running');
const { ConductorError,
	EntryNotFoundError,
	DeserializationError,
	CustomError,
	...hc_client }			= HolochainClientLib;

const { expect_reject }			= require('./utils.js');
const { backdrop }			= require('./setup.js');

const delay				= (n) => new Promise(f => setTimeout(f, n));
const DNAREPO_PATH			= path.join( __dirname, "../../bundled/dnarepo.dna" );
const HDK_VERSION			= "v0.0.146";

let clients;
let zome_1;
let zome_version_1_core;
let zome_version_1;
let zome_version_2;
let dna_1;
let dna_addr;
let dna_version_hash, dna_version_action;

function basic_tests () {
    const zome_bytes			= fs.readFileSync( path.resolve(__dirname, "../../zomes/mere_memory.wasm") );
    const bigzome_bytes			= Buffer.concat( Array(3).fill(zome_bytes) );

    it("should manage developer profile", async function () {
	this.timeout( 10_000 );

	const alice			= clients.alice;
	const bobby			= clients.bobby;

	let profile_action;
	let profile_input		= {
	    "name": "Zed Shaw",
	    "email": "zed.shaw@example.com",
	    "avatar_image": Buffer.from( (new Identicon( Buffer.from( alice.cellAgent() ).toString("hex"), 10)).toString(), "base64"),
	};

	{
	    let profile_info		= await alice.call( "dnarepo", "dna_library", "create_profile", profile_input );
	    log.normal("Set Developer profile: %s -> %s", String(profile_info.$addr), profile_info.name );

	    expect( profile_info.name	).to.equal( profile_input.name );

	    profile_action		= profile_info.$action;
	}

	{
	    let a_profile		= await alice.call( "dnarepo", "dna_library", "get_profile", {} );
	    log.normal("Alice profile: %s", a_profile.name );
	}

	{
	    let action_hash		= await alice.call( "dnarepo", "dna_library", "follow_developer", {
		"agent": clients.bobby.cellAgent(),
	    });
	    log.normal("Following link hash: %s", String(new HoloHash(action_hash)) );

	    await alice.call( "dnarepo", "dna_library", "follow_developer", {
		"agent": clients.carol.cellAgent(),
	    });

	    let following		= await alice.call( "dnarepo", "dna_library", "get_following", null );
	    log.normal("Following %s developers", following.length );

	    expect( following		).to.have.length( 2 );

	    let delete_hash		= await alice.call( "dnarepo", "dna_library", "unfollow_developer", {
		"agent": clients.carol.cellAgent(),
	    });
	    log.normal("Unfollowing link hash: %s", String(new HoloHash(delete_hash)) );

	    await delay(200);

	    let updated_following	= await alice.call( "dnarepo", "dna_library", "get_following", null );
	    log.normal("Following %s developers", following.length );

	    expect( updated_following	).to.have.length( 1 );
	}

	{
	    let profile_update_input	= {
		"email": "zed.shaw@example.com",
		"website": "zedshaw.example.com",
	    };
	    let profile_info		= await alice.call( "dnarepo", "dna_library", "update_profile", {
		"addr": profile_action,
		"properties": profile_update_input,
	    });
	    log.normal("Updated Developer profile: %s -> %s", String(profile_info.$addr), profile_info.name );

	    expect( profile_info.name	).to.equal( profile_input.name );
	    expect( profile_info.email	).to.equal( profile_update_input.email );
	}
    });

    it("should CRUD Zome and ZomeVersion", async function () {
	this.timeout( 60_000 );

	const alice			= clients.alice;
	const bobby			= clients.bobby;

	let zome_input			= {
	    "name": "file_storage",
	    "display_name": "File Storage",
	    "description": "Extra API for file_storage_core",
	    "tags": [ "Storage", "General-use" ],
	};

	let zome			= zome_1 = await alice.call( "dnarepo", "dna_library", "create_zome", zome_input );;
	log.normal("New ZOME (metadata): %s -> %s", String(zome.$id), zome.name );

	let first_action_hash;
	{
	    // Check the created entry
	    let zome_info		= await alice.call( "dnarepo", "dna_library", "get_zome", {
		"id": zome.$id,
	    });
	    log.info("ZOME: %s", zome_info.name );

	    expect( zome_info.name		).to.equal( zome_input.name );
	    expect( zome_info.description	).to.equal( zome_input.description );

	    first_action_hash		= zome_info.$action;
	}

	{
	    let zomes			= await alice.call( "dnarepo", "dna_library", "get_zomes_by_filter", {
		"filter": "name",
		"keyword": zome_input.name.toLowerCase(),
	    });
	    log.normal("Zomes by name: %s", zomes.length );

	    expect( zomes		).to.have.length( 1 );
	}

	{
	    let zomes			= await alice.call( "dnarepo", "dna_library", "get_zomes_by_tags", [ "Storage" ] );
	    log.normal("Zomes by title: %s", zomes.length );

	    expect( zomes		).to.have.length( 1 );
	}
	{
	    let zomes			= await alice.call( "dnarepo", "dna_library", "get_zomes_by_tags", [ "storage", "general-use" ] );
	    log.normal("Zomes by title: %s", zomes.length );

	    expect( zomes		).to.have.length( 1 );
	}
	{
	    let zomes			= await alice.call( "dnarepo", "dna_library", "get_zomes_by_tags", [ "storage", "non-existent" ] );
	    log.normal("Zomes by title: %s", zomes.length );

	    expect( zomes		).to.have.length( 0 );
	}

	{
	    log.debug("ZOME file bytes (%s): typeof %s", zome_bytes.length, typeof zome_bytes );
	    let version			= await alice.call( "dnarepo", "dna_library", "create_zome_version", {
		"for_zome": zome.$id,
		"version": "v0.1.0",
		"ordering": 1,
		"zome_bytes": zome_bytes,
		"hdk_version": HDK_VERSION,
	    });
	    log.normal("New ZOME version: %s -> %s", String(version.$address), version.version );

	    zome_version_1		= version;
	}

	{
	    let zome_core		= zome_1 = await alice.call( "dnarepo", "dna_library", "create_zome", {
		"name": "file_storage_core",
		"zome_type": 0,
		"display_name": "File Storage - Integrity",
		"description": "A generic API for fs-like data management",
	    });;
	    zome_version_1_core			= await alice.call( "dnarepo", "dna_library", "create_zome_version", {
		"for_zome": zome_core.$id,
		"version": "v0.1.0",
		"ordering": 1,
		"zome_bytes": zome_bytes.slice(5_000),
		"hdk_version": HDK_VERSION,
	    });
	}

	{
	    log.debug("Big ZOME file bytes (%s): typeof %s", bigzome_bytes.length, typeof bigzome_bytes );
	    let version			= await alice.call( "dnarepo", "dna_library", "create_zome_version", {
		"for_zome": zome.$id,
		"version": "v0.2.0",
		"ordering": 2,
		"zome_bytes": bigzome_bytes,
		"hdk_version": HDK_VERSION,
	    });
	    log.normal("New ZOME version: %s -> %s", String(version.$address), version.version );

	    zome_version_2		= version;
	}

	{
	    // Make another zome version for list tests
	    await alice.call( "dnarepo", "dna_library", "create_zome_version", {
		"for_zome": zome.$id,
		"version": "v0.3.0",
		"ordering": 3,
		"zome_bytes": [ 1, 2, 3 ],
		"hdk_version": HDK_VERSION,
	    });
	}

	{
	    let zome_versions		= await alice.call( "dnarepo", "dna_library", "get_zome_versions", {
		"for_zome": zome.$id,
	    });
	    log.info("ZOME Versions: %s", zome_versions.version );

	    log.normal("Version list (%s):", zome_versions.length,  );
	    zome_versions.forEach( v => {
		log.normal("  - ZomeVersion { version: %s, published_at: %s }", v.version, v.published_at );
	    });

	    expect( zome_versions	).to.have.length( 3 );
	}

	{
	    let zomes			= await alice.call( "dnarepo", "dna_library", "get_zome_versions_by_filter", {
		"filter": "uniqueness_hash",
		"keyword": zome_version_1.mere_memory_hash,
	    });
	    log.normal("Zomes by name: %s", zomes.length );

	    expect( zomes		).to.have.length( 1 );
	}

	{
	    let zomes			= await alice.call( "dnarepo", "dna_library", "get_my_zomes", null);
	    log.info("My ZOMEs: %s", zomes.length );

	    log.normal("ZOME list (%s):", zomes.length,  );
	    zomes.forEach( v => {
		log.normal("  - Zome { name: %s, published_at: %s }", v.name, v.published_at );
	    });

	    expect( zomes		).to.have.length( 2 );

	    let b_zomes			= await alice.call( "dnarepo", "dna_library", "get_zomes", {
		"agent": clients.bobby.cellAgent(),
	    });
	    log.normal("Bobby ZOMEs: %s", b_zomes.length );
	    expect( b_zomes		).to.have.length( 0 );
	}

	let second_action_hash;
	{
	    // Update ZOME
	    const zome_name		= "whi_game_turns";
	    const tags			= [ "Storage", "Tool" ];
	    zome			= await alice.call( "dnarepo", "dna_library", "update_zome", {
		"addr": zome.$action,
		"properties": {
		    "name": zome_name,
		    tags,
		}
	    });
	    expect( zome.$action		).to.not.deep.equal( first_action_hash );
	    log.normal("Updated ZOME (metadata): %s -> %s", String(zome.$addr), zome.name );

	    let zome_info		= await alice.call( "dnarepo", "dna_library", "get_zome", {
		"id": zome.$id,
	    });
	    log.info("ZOME post update: %s", zome_info.name );

	    expect( zome_info.name	).to.equal( zome_name );
	    expect( zome_info.$action	).to.not.deep.equal( first_action_hash );

	    second_action_hash		= zome.$action;
	}

	{
	    let zomes			= await alice.call( "dnarepo", "dna_library", "get_zomes_by_tags", [ "general-use" ] );
	    log.normal("Zomes by title: %s", zomes.length );

	    expect( zomes		).to.have.length( 0 );
	}
	{
	    let zomes			= await alice.call( "dnarepo", "dna_library", "get_zomes_by_tags", [ "storage", "tool" ] );
	    log.normal("Zomes by title: %s", zomes.length );

	    expect( zomes		).to.have.length( 1 );
	}

	{
	    let zomes			= await alice.call( "dnarepo", "dna_library", "get_zomes_by_filter", {
		"filter": "name",
		"keyword": zome_input.name.toLowerCase(),
	    });
	    log.normal("Zomes by name: %s", zomes.length );

	    expect( zomes		).to.have.length( 0 );
	}

	{
	    let zomes			= await alice.call( "dnarepo", "dna_library", "get_zomes_by_filter", {
		"filter": "name",
		"keyword": zome.name.toLowerCase(),
	    });
	    log.normal("Zomes by name: %s", zomes.length );

	    expect( zomes		).to.have.length( 1 );
	}

	{
	    // Update ZOME Version
	    const properties		= {
		"changelog": "# Changelog\nFeatures\n...",
	    };
	    let zome_version		= await alice.call( "dnarepo", "dna_library", "update_zome_version", {
		"addr": zome_version_1.$action,
		"properties": properties,
	    });
	    log.normal("Updated ZOME Version (metadata): %s -> %s", String(zome_version.$address), zome_version.version );

	    let zome_version_info	= await alice.call( "dnarepo", "dna_library", "get_zome_version", {
		"id": zome_version_1.$id,
	    });
	    log.info("ZOME Version post update: %s", zome_version_info.version );
	    expect( zome_version_info.changelog		).to.equal( properties.changelog );
	}

	{
	    // Unpublish ZOME Version
	    let deleted_zome_version_hash	= await alice.call( "dnarepo", "dna_library", "delete_zome_version", {
		"id": zome_version_1.$id,
	    });
	    log.normal("Deleted ZOME Version hash: %s", String(new HoloHash(deleted_zome_version_hash)) );

	    let zome_versions		= await alice.call( "dnarepo", "dna_library", "get_zome_versions", {
		"for_zome": zome.$id,
	    });
	    expect( zome_versions	).to.have.length( 2 );
	}

	{
	    let zomes			= await clients.alice.call( "dnarepo", "dna_library", "get_all_zomes");
	    log.normal("Zomes by hash: %s", zomes.length );

	    expect( zomes			).to.have.length( 2 );
	}

	{
	    let zomes			= await alice.call( "dnarepo", "dna_library", "get_zome_versions_by_filter", {
		"filter": "uniqueness_hash",
		"keyword": zome_version_1.mere_memory_hash,
	    });
	    log.normal("Zomes by name: %s", zomes.length );

	    expect( zomes		).to.have.length( 0 );
	}

	{
	    // Deprecate ZOME
	    let deprecation_notice	= "No longer maintained";
	    zome			= zome_1 = await alice.call( "dnarepo", "dna_library", "deprecate_zome", {
		"addr": zome.$action,
		"message": deprecation_notice,
	    });
	    log.normal("Deprecated ZOME (metadata): %s -> %s", String(zome.$addr), zome.name );

	    expect( zome.$action		).to.not.deep.equal( second_action_hash );

	    let zome_info		= await alice.call( "dnarepo", "dna_library", "get_zome", {
		"id": zome.$id,
	    });
	    log.info("ZOME post deprecation: %s", zome_info.name );
	    expect( zome_info.deprecation.message	).to.equal( deprecation_notice );
	    expect( zome_info.$action			).to.not.deep.equal( second_action_hash );

	    let zomes			= await alice.call( "dnarepo", "dna_library", "get_my_zomes", null);
	    expect( zomes		).to.have.length( 1 );
	}

	{
	    let zomes			= await alice.call( "dnarepo", "dna_library", "get_zomes_by_filter", {
		"filter": "name",
		"keyword": zome.name.toLowerCase(),
	    });
	    log.normal("Zomes by name: %s", zomes.length );

	    expect( zomes		).to.have.length( 0 );
	}
    });

    it("should CRUD Dna and DnaVersion", async function () {
	this.timeout( 60_000 );

	const alice			= clients.alice;
	const bobby			= clients.bobby;

	let dna_input			= {
	    "name": "game_turns",
	    "display_name": "Game Turns",
	    "description": "A tool for turn-based games to track the order of player actions",
	    "tags": [ "Games", "Turn-based" ],
	    "metadata": {
		"color": "blue",
	    },
	};

	let dna				= dna_1 = await alice.call( "dnarepo", "dna_library", "create_dna", dna_input );
	dna_addr			= dna.$action;
	log.normal("New DNA (metadata): %s -> %s", String(dna.$id), dna.name );

	let first_action_hash;
	{
	    // Check the created entry
	    let dna_info		= await alice.call( "dnarepo", "dna_library", "get_dna", {
		"id": dna.$id,
	    });
	    log.info("DNA: %s", dna_info.name );

	    expect( dna_info.name		).to.equal( dna_input.name );
	    expect( dna_info.description	).to.equal( dna_input.description );
	    expect( dna_info.metadata.color	).to.equal( "blue" );

	    first_action_hash		= dna_info.$action;
	}

	{
	    let dnas			= await alice.call( "dnarepo", "dna_library", "get_dnas_by_tags", [ "Games" ] );
	    log.normal("DNAs by title: %s", dnas.length );

	    expect( dnas		).to.have.length( 1 );
	}
	{
	    let dnas			= await alice.call( "dnarepo", "dna_library", "get_dnas_by_tags", [ "games", "turn-based" ] );
	    log.normal("DNAs by title: %s", dnas.length );

	    expect( dnas		).to.have.length( 1 );
	}
	{
	    let dnas			= await alice.call( "dnarepo", "dna_library", "get_dnas_by_tags", [ "Games", "Action" ] );
	    log.normal("DNAs by title: %s", dnas.length );

	    expect( dnas		).to.have.length( 0 );
	}

	{
	    let version			= await alice.call( "dnarepo", "dna_library", "create_dna_version", {
		"for_dna": dna.$id,
		"version": "v0.1.0",
		"ordering": 1,
		"hdk_version": HDK_VERSION,
		"integrity_zomes": [{
		    "name": "mere_memory",
		    "zome": new EntryHash( zome_version_1_core.for_zome ),
		    "version": zome_version_1_core.$id,
		    "resource": new EntryHash( zome_version_1_core.mere_memory_addr ),
		    "resource_hash": zome_version_1_core.mere_memory_hash,
		}],
		"zomes": [{
		    "name": "mere_memory_api",
		    "zome": new EntryHash( zome_version_1.for_zome ),
		    "version": zome_version_1.$id,
		    "resource": new EntryHash( zome_version_1.mere_memory_addr ),
		    "resource_hash": zome_version_1.mere_memory_hash,
		    "dependencies": [
			"mere_memory_core",
		    ],
		}],
		"origin_time": "2022-02-11T23:05:19.470323Z",
	    });
	    log.normal("New DNA version: %s -> %s", String(version.$address), version.version );

	    dna_version_hash		= version.$id;
	    dna_version_action		= version.$action;
	}

	{
	    let wasm_hash_bytes		= Buffer.from( zome_version_1_core.mere_memory_hash, "hex" );
	    let hash			= crypto.createHash("sha256");
	    hash.update( wasm_hash_bytes );
	    let keyword			= hash.digest("hex");

	    let versions		= await alice.call( "dnarepo", "dna_library", "get_dna_versions_by_filter", {
		"filter": "uniqueness_hash",
		"keyword": keyword,
	    });
	    log.normal("DNA versions by hash: %s", versions.length );

	    expect( versions		).to.have.length( 1 );
	}

	{
	    let version			= await alice.call( "dnarepo", "dna_library", "create_dna_version", {
		"for_dna": dna.$id,
		"version": "v0.2.0",
		"ordering": 2,
		"hdk_version": HDK_VERSION,
		"integrity_zomes": [{
		    "name": "mere_memory_core",
		    "zome": new EntryHash( zome_version_1_core.for_zome ),
		    "version": zome_version_1_core.$id,
		    "resource": new EntryHash( zome_version_1_core.mere_memory_addr ),
		    "resource_hash": zome_version_1_core.mere_memory_hash,
		}],
		"zomes": [],
		"origin_time": "2022-02-11T23:05:19.470323Z",
	    });
	    log.normal("New DNA version: %s -> %s", String(version.$address), version.version );
	}

	{
	    let wasm_hash_bytes		= Buffer.from( zome_version_2.mere_memory_hash, "hex" );
	    let hash			= crypto.createHash("sha256");
	    hash.update( wasm_hash_bytes );
	    let keyword			= hash.digest("hex");

	    let versions		= await alice.call( "dnarepo", "dna_library", "get_dna_versions_by_filter", {
		"filter": "uniqueness_hash",
		"keyword": keyword,
	    });
	    log.normal("DNA versions by hash: %s", versions.length );

	    expect( versions		).to.have.length( 0 );
	}

	{
	    let dna_versions		= await alice.call( "dnarepo", "dna_library", "get_dna_versions", {
		"for_dna": dna.$id,
	    });
	    log.info("DNA Versions: %s", dna_versions.version );

	    log.normal("Version list (%s):", dna_versions.length,  );
	    dna_versions.forEach( v => {
		log.normal("  - DnaVersion { version: %s, file_size: %s, published_at: %s }", v.version, v.file_size, v.published_at );
	    });

	    expect( dna_versions	).to.have.length( 2 );
	}

	{
	    let dnas			= await alice.call( "dnarepo", "dna_library", "get_my_dnas", null);
	    log.info("My DNAs: %s", dnas.length );

	    log.normal("DNA list (%s):", dnas.length,  );
	    dnas.forEach( v => {
		log.normal("  - Dna { name: %s, published_at: %s }", v.name, v.published_at );
	    });

	    expect( dnas		).to.have.length( 1 );

	    let b_dnas			= await alice.call( "dnarepo", "dna_library", "get_dnas", {
		"agent": clients.bobby.cellAgent(),
	    });
	    log.normal("Bobby DNAs: %s", b_dnas.length );
	    expect( b_dnas		).to.have.length( 0 );
	}

	{
	    let dnas			= await alice.call( "dnarepo", "dna_library", "get_dnas_by_filter", {
		"filter": "name",
		"keyword": dna_input.name.toLowerCase(),
	    });
	    log.normal("DNAs by name: %s", dnas.length );

	    expect( dnas		).to.have.length( 1 );
	}

	let second_action_hash;
	{
	    // Update DNA
	    const dna_name		= "game_turns_new";
	    const tags			= [ "Games", "Turns" ];
	    dna				= await alice.call( "dnarepo", "dna_library", "update_dna", {
		"addr": dna.$action,
		"properties": {
		    "name": dna_name,
		    tags,
		}
	    });
	    expect( dna.$action		).to.not.deep.equal( first_action_hash );
	    log.normal("Updated DNA (metadata): %s -> %s", String(dna.$addr), dna.name );

	    let dna_info		= await alice.call( "dnarepo", "dna_library", "get_dna", {
		"id": dna.$id,
	    });
	    log.info("DNA post update: %s", dna_info.name );

	    expect( dna_info.name	).to.equal( dna_name );
	    expect( dna_info.$action	).to.not.deep.equal( first_action_hash );

	    second_action_hash		= dna.$action;
	}

	{
	    let dnas			= await alice.call( "dnarepo", "dna_library", "get_dnas_by_tags", [ "turn-based" ] );
	    log.normal("DNAs by title: %s", dnas.length );

	    expect( dnas		).to.have.length( 0 );
	}
	{
	    let dnas			= await alice.call( "dnarepo", "dna_library", "get_dnas_by_tags", [ "games", "turns" ] );
	    log.normal("DNAs by title: %s", dnas.length );

	    expect( dnas		).to.have.length( 1 );
	}

	{
	    let dnas			= await alice.call( "dnarepo", "dna_library", "get_dnas_by_filter", {
		"filter": "name",
		"keyword": dna_input.name.toLowerCase(),
	    });
	    log.normal("DNAs by name: %s", dnas.length );

	    expect( dnas		).to.have.length( 0 );
	}

	{
	    let dnas			= await alice.call( "dnarepo", "dna_library", "get_dnas_by_filter", {
		"filter": "name",
		"keyword": dna.name.toLowerCase(),
	    });
	    log.normal("DNAs by name: %s", dnas.length );

	    expect( dnas		).to.have.length( 1 );
	}

	{
	    // Update DNA Version
	    const properties		= {
		"changelog": "# Changelog\nFeatures\n...",
	    };
	    let dna_version		= await alice.call( "dnarepo", "dna_library", "update_dna_version", {
		"addr": dna_version_action,
		"properties": properties,
	    });
	    log.normal("Updated DNA Version (metadata): %s -> %s", String(dna_version.$address), dna_version.version );

	    let dna_version_info	= await alice.call( "dnarepo", "dna_library", "get_dna_version", {
		"id": dna_version_hash,
	    });
	    log.info("DNA Version post update: %s", dna_version_info.version );
	    expect( dna_version_info.changelog		).to.equal( properties.changelog );
	}

	{
	    let pack			= await alice.call( "dnarepo", "dna_library", "get_dna_package", {
		"id": dna_version_hash,
	    });
	    log.info("DNA Package bytes: %s", pack.bytes.length );

	    expect( pack.bytes.constructor.name		).to.equal("Array");
	}

	{
	    // Unpublish DNA Version
	    let deleted_dna_version_hash	= await alice.call( "dnarepo", "dna_library", "delete_dna_version", {
		"id": dna_version_hash,
	    });
	    log.normal("Deleted DNA Version hash: %s", String(new HoloHash(deleted_dna_version_hash)) );

	    let dna_versions		= await alice.call( "dnarepo", "dna_library", "get_dna_versions", {
		"for_dna": dna.$id,
	    });
	    expect( dna_versions	).to.have.length( 1 );
	}

	{
	    let dnas			= await clients.alice.call( "dnarepo", "dna_library", "get_all_dnas");
	    log.normal("DNAs by hash: %s", dnas.length );

	    expect( dnas			).to.have.length( 1 );
	}

	{
	    let wasm_hash_bytes		= Buffer.from( zome_version_1.mere_memory_hash, "hex" );
	    let hash			= crypto.createHash("sha256");
	    hash.update( wasm_hash_bytes );

	    let versions		= await alice.call( "dnarepo", "dna_library", "get_dna_versions_by_filter", {
		"filter": "uniqueness_hash",
		"keyword": hash.digest("hex"),
	    });
	    log.normal("DNA versions by hash: %s", versions.length );

	    expect( versions		).to.have.length( 0 );
	}

	{
	    // Deprecate DNA
	    let deprecation_notice	= "No longer maintained";
	    dna				= await alice.call( "dnarepo", "dna_library", "deprecate_dna", {
		"addr": dna.$action,
		"message": deprecation_notice,
	    });
	    log.normal("Deprecated DNA (metadata): %s -> %s", String(dna.$addr), dna.name );

	    expect( dna.$action		).to.not.deep.equal( second_action_hash );

	    let dna_info		= await alice.call( "dnarepo", "dna_library", "get_dna", {
		"id": dna.$id,
	    });
	    log.info("DNA post deprecation: %s", dna_info.name );
	    expect( dna_info.deprecation.message	).to.equal( deprecation_notice );
	    expect( dna_info.$action			).to.not.deep.equal( second_action_hash );

	    let dnas			= await alice.call( "dnarepo", "dna_library", "get_my_dnas", null);
	    expect( dnas		).to.have.length( 0 );
	}

	{
	    let dnas			= await alice.call( "dnarepo", "dna_library", "get_dnas_by_filter", {
		"filter": "name",
		"keyword": dna_input.name.toLowerCase(),
	    });
	    log.normal("DNAs by name: %s", dnas.length );

	    expect( dnas		).to.have.length( 0 );
	}

	{
	    let dnas			= await alice.call( "dnarepo", "dna_library", "get_dnas_by_tags", [ "games", "turns" ] );
	    log.normal("DNAs by title: %s", dnas.length );

	    expect( dnas		).to.have.length( 0 );
	}
    });

    it("should make multiple asynchronous calls to get_zomes_by_filter", async function () {
	await Promise.all( [1,2].map( async () => {
	    let zomes		= await clients.alice.call( "dnarepo", "dna_library", "get_zomes_by_filter", {
		"filter": "name",
		"keyword": crypto.randomBytes( 10 ).toString("hex"),
	    });
	    log.normal("Zomes by name: %s", zomes.length );

	    expect( zomes		).to.have.length( 0 );
	}) );
    });

    it("should make multiple asynchronous calls to get_zome_versions_by_filter", async function () {
	await Promise.all( [1,2].map( async () => {
	    let versions		= await clients.alice.call( "dnarepo", "dna_library", "get_zome_versions_by_filter", {
		"filter": "uniqueness_hash",
		"keyword": crypto.randomBytes( 10 ).toString("hex"),
	    });
	    log.normal("Versions by name: %s", versions.length );

	    expect( versions		).to.have.length( 0 );
	}) );
    });

    it("should make multiple asynchronous calls to get_dnas_by_filter", async function () {
	await Promise.all( [1,2].map( async () => {
	    let dnas			= await clients.alice.call( "dnarepo", "dna_library", "get_dnas_by_filter", {
		"filter": "name",
		"keyword": crypto.randomBytes( 10 ).toString("hex"),
	    });
	    log.normal("DNAs by name: %s", dnas.length );

	    expect( dnas		).to.have.length( 0 );
	}) );
    });

    it("should make multiple asynchronous calls to get_dna_versions_by_filter", async function () {
	await Promise.all( [1,2].map( async () => {
	    let versions		= await clients.alice.call( "dnarepo", "dna_library", "get_dna_versions_by_filter", {
		"filter": "uniqueness_hash",
		"keyword": crypto.randomBytes( 10 ).toString("hex"),
	    });
	    log.normal("DNA versions by hash: %s", versions.length );

	    expect( versions		).to.have.length( 0 );
	}) );
    });

    it("should get all DNAs", async function () {
	let dnas			= await clients.alice.call( "dnarepo", "dna_library", "get_all_dnas");
	log.normal("DNAs by hash: %s", dnas.length );

	expect( dnas			).to.have.length( 0 );
    });

    it("should get all Zomes", async function () {
	let zomes			= await clients.alice.call( "dnarepo", "dna_library", "get_all_zomes");
	log.normal("Zomes by hash: %s", zomes.length );

	expect( zomes			).to.have.length( 1 );
    });

    let hdk_version;
    it("should get HDK version list", async function () {
	let hdkvs			= await clients.alice.call( "dnarepo", "dna_library", "get_hdk_versions");
	log.normal("HDK versions: %s", hdkvs.length );

	expect( hdkvs			).to.have.length( 1 );
	expect( hdkvs[0]		).to.equal( HDK_VERSION );

	hdk_version			= hdkvs[0];
    });

    it("should get Zome Versions by HDK version", async function () {
	if ( hdk_version === undefined )
	    this.skip();
	let zomes			= await clients.alice.call( "dnarepo", "dna_library", "get_zome_versions_by_hdk_version", hdk_version );
	log.normal("Zomes by hash: %s", zomes.length );

	expect( zomes			).to.have.length( 3 );
    });

    it("should get Zome by HDK version", async function () {
	if ( hdk_version === undefined )
	    this.skip();
	let zomes			= await clients.alice.call( "dnarepo", "dna_library", "get_zomes_with_an_hdk_version", hdk_version );
	log.normal("Zomes by hash: %s", zomes.length );

	expect( zomes			).to.have.length( 2 );
    });

    it("should get Dna by HDK version", async function () {
	if ( hdk_version === undefined )
	    this.skip();
	let dnas			= await clients.alice.call( "dnarepo", "dna_library", "get_dnas_with_an_hdk_version", hdk_version );
	log.normal("Dnas by hash: %s", dnas.length );

	expect( dnas			).to.have.length( 1 );
    });
}

function errors_tests () {
    it("should fail to update another Agent's zome", async function () {
	if ( zome_1 === undefined )
	    this.skip();

	await expect_reject( async () => {
	    await clients.bobby.call( "dnarepo", "dna_library", "update_zome", {
		"addr": zome_1.$action,
		"properties": {
		    "name": "bla bla bla",
		}
	    });
	}, ConductorError, "InvalidCommit error: Previous entry author does not match Action author" );
    });

    it("should fail to update deprecated zome", async function () {
	if ( zome_1 === undefined || !zome_1.deprecation )
	    this.skip();

	await expect_reject( async () => {
	    await clients.alice.call( "dnarepo", "dna_library", "update_zome", {
		"addr": zome_1.$action,
		"properties": {
		    "name": "bla bla bla",
		}
	    });
	}, ConductorError, "InvalidCommit error: Cannot update deprecated Zome" );
    });

    it("should fail to update another Agent's zome version", async function () {
	if ( zome_version_2 === undefined )
	    this.skip();

	await expect_reject( async () => {
	    await clients.bobby.call( "dnarepo", "dna_library", "update_zome_version", {
		"addr": zome_version_2.$action,
		"properties": {
		    "changelog": "",
		}
	    });
	}, ConductorError, "InvalidCommit error: ZomeEntry author does not match Action author" );
    });

    it("should fail to delete another Agent's zome version", async function () {
	if ( zome_version_2 === undefined )
	    this.skip();

	await expect_reject( async () => {
	    await clients.bobby.call( "dnarepo", "dna_library", "delete_zome_version", {
		"id": zome_version_2.$id,
	    });
	}, ConductorError, "InvalidCommit error: Delete author does not match Create author" );
    });

    it("should fail to get profile because it is not made yet", async function () {
	if ( zome_version_2 === undefined )
	    this.skip();

	await expect_reject( async () => {
	    await clients.bobby.call( "dnarepo", "dna_library", "get_profile", {
		"id": zome_version_2.$id,
	    });
	}, Error, "Agent Profile has not been created yet" );
    });

    it("should fail to get deleted DNA version", async function () {
	if ( dna_version_hash === undefined )
	    this.skip();

	await expect_reject( async () => {
	    await clients.alice.call( "dnarepo", "dna_library", "get_dna_version", {
		"id": dna_version_hash,
	    });
	}, EntryNotFoundError, "Record not found for Entry address" );
    });

    it("should fail to create ZOME version because missing ZOME package info", async function () {
	await expect_reject( async () => {
	    await clients.alice.call( "dnarepo", "dna_library", "create_zome_version", {
		"for_zome": new HoloHash("uhCEkvriXQtLwCt8urCSqAxS6MYUGPEVbb3h0CH0aVj4QVba1fEzj"),
		"version": "v0.1.0",
		"ordering": 1,
		"file_size": 0,
		"hdk_version": HDK_VERSION,
	    });
	}, Error, "must supply an address or bytes" );
    });

    it("should fail to update DNA because the address is a different entry type", async function () {
	if ( dna_addr === undefined )
	    this.skip();

	await expect_reject( async () => {
	    await clients.alice.call( "dnarepo", "dna_library", "update_dna_version", {
		"addr": dna_addr,
		"properties": {
		    "name": "Bla bla",
		}
	    });
	}, Error, `Failed to deserialize to entry type 'DnaVersion'` );
    });

    it("should fail to create DNA version with empty Zomes", async function () {
	if ( dna_1 === undefined )
	    this.skip();

	await expect_reject( async () => {
	    await clients.alice.call( "dnarepo", "dna_library", "create_dna_version", {
		"for_dna": dna_1.$id,
		"version": "v0.1.0",
		"ordering": 1,
		"hdk_version": HDK_VERSION,
		"integrity_zomes": [],
		"zomes": [],
		"origin_time": "2022-02-11T23:05:19.470323Z",
	    });
	}, Error, "Must have at least 1 integrity zome" );
    });
}

describe("DNArepo", () => {

    const holochain			= new Holochain({
	"default_stdout_loggers": process.env.LOG_LEVEL === "silly",
    });

    before(async function () {
	this.timeout( 60_000 );

	clients				= await backdrop( holochain, {
	    "dnarepo": DNAREPO_PATH,
	}, [
	    "alice",
	    "bobby",
	    "carol",
	]);

	// Must call whoami on each cell to ensure that init has finished.
	{
	    let whoami			= await clients.alice.call( "dnarepo", "dna_library", "whoami", null, 30_000 );
	    log.normal("Alice whoami: %s", String(new HoloHash( whoami.agent_initial_pubkey )) );
	}
	{
	    let whoami			= await clients.bobby.call( "dnarepo", "dna_library", "whoami", null, 30_000 );
	    log.normal("Bobby whoami: %s", String(new HoloHash( whoami.agent_initial_pubkey )) );
	}
	{
	    let whoami			= await clients.carol.call( "dnarepo", "dna_library", "whoami", null, 30_000 );
	    log.normal("Carol whoami: %s", String(new HoloHash( whoami.agent_initial_pubkey )) );
	}
    });

    describe("Basic", basic_tests.bind( this, holochain ) );
    describe("Errors", errors_tests.bind( this, holochain ) );

    after(async () => {
	await holochain.destroy();
    });

});
