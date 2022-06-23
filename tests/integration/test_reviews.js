const path				= require('path');
const log				= require('@whi/stdlog')(path.basename( __filename ), {
    level: process.env.LOG_LEVEL || 'fatal',
});


const fs				= require('fs');
const crypto				= require('crypto');
const expect				= require('chai').expect;
const { faker }				= require('@faker-js/faker');
const msgpack				= require('@msgpack/msgpack');
const { EntryHash,
	HoloHash }			= require('@whi/holo-hash');
const { Holochain }			= require('@whi/holochain-backdrop');
const json				= require('@whi/json');
const why				= require('why-is-node-running');

// setTimeout(() => {
//     console.log( why() );
// }, 6000 );

const { backdrop }			= require('./setup.js');

const delay				= (n) => new Promise(f => setTimeout(f, n));
const DNAREPO_PATH			= path.join( __dirname, "../../bundled/dnarepo.dna" );

let clients;
let zome_1;
let zome_version_1;
let dna_1;
let dna_version_1;
let expected_average;
let expected_median;

function basic_tests () {
    it("should CRUD Reviews", async function () {
	this.timeout( 30_000 );

	let review_input			= {
	    "subject_id": zome_version_1.$id,
	    "subject_addr": zome_version_1.$addr,
	    "rating": 5,
	    "message": "This code is not good",
	};

	let review				= review_1 = await clients.alice.call( "dnarepo", "reviews", "create_review", review_input );
	log.normal("New Review: %s -> %s", String(review.$id), review.rating );

	{
	    // Check the created entry
	    let review_info			= await clients.alice.call( "dnarepo", "reviews", "get_review", {
		"id": review.$id,
	    });
	    log.info("Review: %s", review_info.rating );

	    expect( review_info.rating		).to.equal( review_input.rating );
	    expect( review_info.message		).to.equal( review_input.message );
	}

	{
	    let reviews				= await clients.alice.call( "dnarepo", "reviews", "get_my_reviews", null);
	    log.info("My Reviews: %s", reviews.length );

	    log.normal("Review list (%s):", reviews.length,  );
	    reviews.forEach( review => {
		log.normal("  - Review { rating: %s, published_at: %s }", review.rating, review.published_at );
	    });

	    expect( reviews			).to.have.length( 1 );
	}

	{
	    // Update Review
	    const review_rating			= 6;
	    review				= await clients.alice.call( "dnarepo", "reviews", "update_review", {
		"addr": review.$addr,
		"properties": {
		    "rating": review_rating,
		}
	    });
	    log.normal("Updated Review: %s -> %s", String(review.$addr), review.rating );

	    let review_info			= await clients.alice.call( "dnarepo", "reviews", "get_review", {
		"id": review.$id,
	    });
	    log.info("Review post update: %s", review_info.rating );

	    expect( review_info.rating		).to.equal( review_rating );
	}

	const review_count			= 20;
	for (let i=0; i < review_count-1; i++ ) {
	    await clients.alice.call( "dnarepo", "reviews", "create_review", {
		"subject_id": zome_version_1.$id,
		"subject_addr": zome_version_1.$addr,
		"rating": faker.datatype.number(10),
		"message": faker.lorem.sentence(),
	    });
	}

	{
	    let reviews				= await clients.alice.call( "dnarepo", "reviews", "get_reviews_for_subject", {
		"id": zome_version_1.$id,
	    });
	    log.info("%s Reviews for subject: %s", reviews.length, String(zome_version_1.$id) );

	    let rating_sum = 0;
	    reviews.forEach( review => {
		rating_sum		       += review.rating;
		log.normal("  - Review { rating: %s, published_at: %s }", review.rating, review.published_at );
	    });

	    let all_ratings			= [].map.call( reviews, r => r.rating );
	    all_ratings.sort( (a,b) => {
		return a === b ? 0 : (a > b ? 1 : -1 );
	    });
	    console.log("Median index (%s):", Math.floor( (all_ratings.length - 1) / 2 ), all_ratings );

	    expect( reviews			).to.have.length( review_count );

	    expected_average			= rating_sum / reviews.length;
	    expected_median			= all_ratings[ Math.floor( (all_ratings.length - 1) / 2 ) ];
	}
    });

    it("should create review summary report", async function () {
	let review_summary			= await clients.alice.call( "dnarepo", "reviews", "create_summary_for_subject", {
	    "subject_id": zome_version_1.$id,
	    "subject_addr": zome_version_1.$addr,
	});
	// console.log( json.debug( review_summary ) );

	expect( review_summary.average		).to.be.closeTo( expected_average, 0.0001 );
	expect( review_summary.median		).to.equal( expected_median );
    });
}

function errors_tests () {
    it("should fail to update DNA because the address is a different entry type", async function () {
	this.skip();

	let failed			= false;
	try {
	    let dna			= await clients.alice.call( "dnarepo", "dna_library", "update_dna_version", {
		"addr": dna_addr,
		"properties": {
		    "name": "Bla bla",
		}
	    });
	} catch (err) {
	    expect( err.kind		).to.equal( "UtilsError" );
	    expect( err.name		).to.equal( "DeserializationError" );
	    expect( err.message		).to.have.string( 'App("dna_version")' );

	    failed			= true;
	}

	expect( failed			).to.be.true;
    });
}

describe("DNArepo", () => {

    const holochain			= new Holochain({
	"default_stdout_loggers": process.env.LOG_LEVEL === "silly",
    });

    before(async function () {
	this.timeout( 30_000 );

	clients				= await backdrop( holochain, {
	    "dnarepo": DNAREPO_PATH,
	}, [
	    "alice",
	]);

	// Must call whoami on each cell to ensure that init has finished.
	{
	    let whoami			= await clients.alice.call( "dnarepo", "dna_library", "whoami" );
	    log.normal("Alice whoami: %s", String(new HoloHash( whoami.agent_initial_pubkey )) );
	}

	let zome_input			= {
	    "name": "file_storage",
	    "description": "A generic API for fs-like data management",
	};

	let zome			= zome_1 = await clients.alice.call( "dnarepo", "dna_library", "create_zome", zome_input );;
	let zome_version		= zome_version_1 = await clients.alice.call( "dnarepo", "dna_library", "create_zome_version", {
	    "for_zome": zome.$id,
	    "version": 1,
	    "zome_bytes": [],
	    "hdk_version": "v0.0.136",
	});

	let dna_input			= {
	    "name": "game_turns",
	    "description": "A tool for turn-based games to track the order of player actions",
	};

	let dna				= dna_1 = await clients.alice.call( "dnarepo", "dna_library", "create_dna", dna_input );
	let dna_version			= dna_version_1 = await clients.alice.call( "dnarepo", "dna_library", "create_dna_version", {
	    "for_dna": dna.$id,
	    "version": 1,
	    "hdk_version": "v0.0.120",
	    "zomes": [{
		"name":			"file_storage",
		"zome":			new EntryHash( zome_version_1.for_zome ),
		"version":		zome_version_1.$id,
		"resource":		new EntryHash( zome_version_1.mere_memory_addr ),
		"resource_hash":	zome_version_1.mere_memory_hash,
	    }],
	});
    });

    describe("Basic", basic_tests.bind( this, holochain ) );
    describe("Errors", errors_tests.bind( this, holochain ) );

    after(async () => {
	await holochain.destroy();
    });

});
