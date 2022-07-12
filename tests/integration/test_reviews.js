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

const { backdrop }			= require('./setup.js');

const delay				= (n) => new Promise(f => setTimeout(f, n));
const DNAREPO_PATH			= path.join( __dirname, "../../bundled/dnarepo.dna" );

let clients;
let zome_1;
let zome_version_1;
let dna_version_1;
let expected_average;
let expected_median;
const review_count			= 20;
let review_1;
let review_summary_1;

function basic_tests () {
    it("should create reviews", async function () {
	this.timeout( 30_000 );

	let review_input			= {
	    "subject_ids": [
		[ zome_1.$id,		zome_1.$header ],
		[ zome_version_1.$id,	zome_version_1.$header ],
	    ],
	    "ratings": {
		"accuracy": 3,
	    },
	    "message": "This code is not good",
	    "related_entries": {},
	};

	let review				= review_1 = await clients.alice.call( "dnarepo", "reviews", "create_review", review_input );
	log.normal("New Review: %s -> %s", String(review.$id), review.ratings.accuracy );

	let zome_version			= zome_version_1 = await clients.alice.call( "dnarepo", "dna_library", "create_zome_version_review_summary", {
	    "subject_header": zome_version_1.$header,
	    "addr": zome_version_1.$addr,
	});
	let review_summary			= review_summary_1 = await clients.alice.call( "dnarepo", "reviews", "get_review_summary", {
	    "id": zome_version.review_summary,
	});
	console.log( json.debug(review_summary) );

	{
	    // Check the created entry
	    let review_info			= await clients.alice.call( "dnarepo", "reviews", "get_review", {
		"id": review.$id,
	    });
	    log.info("Review: %s", review_info.ratings.accuracy );

	    expect( review_info.ratings.accuracy	).to.equal( review_input.ratings.accuracy );
	    expect( review_info.message			).to.equal( review_input.message );
	}

	{
	    let reviews				= await clients.alice.call( "dnarepo", "reviews", "get_my_reviews", null);
	    log.info("My Reviews: %s", reviews.length );

	    log.normal("Review list (%s):", reviews.length,  );
	    reviews.forEach( review => {
		log.normal("  - Review { rating: %s, published_at: %s }", review.ratings.accuracy, review.published_at );
	    });

	    expect( reviews			).to.have.length( 1 );
	}

	for (let i=0; i < review_count-1; i++ ) {
	    await clients.alice.call( "dnarepo", "reviews", "create_review", {
		"subject_ids": [
		    [ zome_1.$id,		zome_1.$header ],
		    [ zome_version_1.$id,	zome_version_1.$header ],
		],
		"ratings": {
		    "accuracy":		faker.datatype.number(10),
		    "efficiency":	faker.datatype.number(4),
		},
		"message": faker.lorem.sentence(),
	    });
	}

	{
	    let reviews				= await clients.alice.call( "dnarepo", "reviews", "get_reviews_for_subject", {
		"id": zome_version_1.$id,
	    });
	    log.info("%s Reviews for subject: %s", reviews.length, String(zome_version_1.$id) );
	}
    });

    it("should create review summary report before review update", async function () {
	let review_summary			= review_summary_1 = await clients.alice.call( "dnarepo", "reviews", "update_review_summary", {
	    "id": review_summary_1.$id,
	});
	console.log( json.debug(review_summary) );

	expect( review_summary.factored_action_count	).to.equal( review_count );
    });

    it("should update review", async function () {
	{
	    // Update Review
	    const accuracy_review_rating	= 8;
	    const efficiency_review_rating	= 9;
	    const review			= review_1 = await clients.alice.call( "dnarepo", "reviews", "update_review", {
		"addr": review_1.$addr,
		"properties": {
		    "ratings": {
			"accuracy":	accuracy_review_rating,
			"efficiency":	efficiency_review_rating,
		    },
		}
	    });
	    log.normal("Updated Review: %s -> %s", String(review.$addr), review.ratings.accuracy );

	    let review_info			= await clients.alice.call( "dnarepo", "reviews", "get_review", {
		"id": review.$id,
	    });

	    expect( review_info.ratings.accuracy	).to.equal( accuracy_review_rating );
	    expect( review_info.ratings.efficiency	).to.equal( efficiency_review_rating );
	}
    });

    it("should create review summary report after review update", async function () {
	let review_summary			= review_summary_1 = await clients.alice.call( "dnarepo", "reviews", "update_review_summary", {
	    "id": review_summary_1.$id,
	});
	console.log( json.debug(review_summary) );

	expect( review_summary.factored_action_count	).to.equal( review_count + 1 );
    });

    it("should delete review", async function () {
	{
	    await clients.alice.call( "dnarepo", "reviews", "delete_review", {
		"addr": review_1.$addr,
	    });
	}

	{
	    let reviews				= await clients.alice.call( "dnarepo", "reviews", "get_reviews_for_subject", {
		"id": zome_version_1.$id,
	    });
	    log.info("My Reviews: %s", reviews.length );

	    expect( reviews			).to.have.length( 19 );
	}
    });

    it("should create review summary report after review delete", async function () {
	let review_summary			= review_summary_1 = await clients.alice.call( "dnarepo", "reviews", "update_review_summary", {
	    "id": review_summary_1.$id,
	});
	console.log( json.debug(review_summary) );

	let deleted_reviews_list		= Object.keys( review_summary.deleted_reviews );

	expect( review_summary.factored_action_count	).to.equal( review_count - 1 + 3 );
	expect( deleted_reviews_list			).to.have.length( 1 );
	expect( review_summary.$id			).to.not.deep.equal( zome_version_1.review_summary );
    });

    it("should get review summaries", async function () {
	let review_summaries			= await clients.alice.call( "dnarepo", "reviews", "get_review_summaries_for_subject", {
	    "id": zome_version_1.$id,
	});

	expect( review_summaries		).to.have.length( 1 );
    });
}

function errors_tests () {
    it("should fail to...", async function () {
	this.skip();
    });
}

describe("Reviews", () => {

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
	    "version": "v0.1.0",
	    "ordering": 1,
	    "zome_bytes": [],
	    "hdk_version": "v0.0.136",
	});

	let dna_input			= {
	    "name": "game_turns",
	    "description": "A tool for turn-based games to track the order of player actions",
	};

	let dna				= await clients.alice.call( "dnarepo", "dna_library", "create_dna", dna_input );
	let dna_version			= dna_version_1 = await clients.alice.call( "dnarepo", "dna_library", "create_dna_version", {
	    "for_dna": dna.$id,
	    "version": "v0.1.0",
	    "ordering": 1,
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
