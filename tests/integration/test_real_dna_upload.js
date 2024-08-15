import { Logger }			from '@whi/weblogger';
const log				= new Logger("test-real-dna", process.env.LOG_LEVEL );

// import why				from 'why-is-node-running';

import * as fs				from 'node:fs/promises';
import path				from 'path';
import crypto				from 'crypto';

import { expect }			from 'chai';

import json				from '@whi/json';
import { Bundle }			from '@spartan-hc/bundles';
import {
    HoloHash,
    DnaHash, AgentPubKey,
    ActionHash, EntryHash,
}					from '@spartan-hc/holo-hash';
import { Holochain }			from '@spartan-hc/holochain-backdrop';

import {
    DnaHubCell,
    ZomeHubCell,
}					from '@holochain/dnahub-zomelets';
import {
    AppInterfaceClient,
}					from '@spartan-hc/app-interface-client';

import {
    expect_reject,
    linearSuite,
    sha256,
}					from '../utils.js';


const __dirname				= path.dirname( new URL(import.meta.url).pathname );
const DNAHUB_DNA_PATH			= path.join( __dirname, "../../dnas/dnahub.dna" );
const ZOMEHUB_DNA_PATH			= path.join( __dirname, "../../dnas/zomehub.dna" );

const DNAHUB_DNA_NAME			= "dnahub";
const ZOMEHUB_DNA_NAME			= "zomehub";

let app_port;
let installations;


describe("DnaHub - Real", function () {
    const holochain			= new Holochain({
	"timeout": 120_000,
	"default_stdout_loggers": log.level_rank > 3,
    });

    before(async function () {
	this.timeout( 120_000 );

	installations			= await holochain.install([
	    "alice",
	], [
	    {
		"app_name": "test",
		"bundle": {
		    [DNAHUB_DNA_NAME]:	DNAHUB_DNA_PATH,
		    [ZOMEHUB_DNA_NAME]:	ZOMEHUB_DNA_PATH,
		},
	    },
	]);

	app_port			= await holochain.ensureAppPort();
    });

    linearSuite("Upload", real_tests );

    after(async () => {
	await holochain.destroy();
    });
});


function real_tests () {
    let client;
    let app_client;
    let zomehub;
    let zomehub_csr;
    let dnahub;
    let dnahub_csr;
    let dna1;

    before(async function () {
	this.timeout( 30_000 );

	client				= new AppInterfaceClient( app_port, {
	    "logging": process.env.LOG_LEVEL || "normal",
	});

	const app_token			= installations.alice.test.auth.token;
	app_client			= await client.app( app_token );

	({
	    zomehub,
	    dnahub,
	}				= app_client.createInterface({
	    [ZOMEHUB_DNA_NAME]:		ZomeHubCell,
	    [DNAHUB_DNA_NAME]:		DnaHubCell,
	}));

	zomehub_csr			= zomehub.zomes.zomehub_csr.functions;
	dnahub_csr			= dnahub.zomes.dnahub_csr.functions;

	await zomehub_csr.whoami();
	await dnahub_csr.whoami();
    });

    it("should create DNA entry", async function () {
	this.timeout( 30_000 );

	const dna_bytes			= await fs.readFile( ZOMEHUB_DNA_PATH );

	src_bundle			= new Bundle( dna_bytes, "dna" );
	dna1				= await dnahub_csr.save_dna( dna_bytes );

	expect( dna1.$addr		).to.be.a("EntryHash");
    });

    let src_bundle;

    it("should get DNA entry", async function () {
	const dna			= await dnahub_csr.get_dna_entry( dna1.$addr );
	log.normal("%s", json.debug(dna) );

	expect( dna			).to.have.any.keys(
	    "manifest",
	    "dna_token",
	    "integrities_token",
	    "coordinators_token",
	);
    });

    it("should get DNA bundle", async function () {
	this.timeout( 30_000 );

	const bundle_bytes		= await dnahub_csr.get_dna_bundle( dna1.$addr );
	const bundle			= new Bundle( bundle_bytes, "dna" );

	const src_msgpack_hash		= sha256( bundle.toEncoded({ sortKeys: true }) );
	const src_manifest		= src_bundle.manifest.source;
	const src_resources		= src_bundle.resources;
	const new_manifest		= bundle.manifest.toJSON();

	for ( let zome_manifest of new_manifest.integrity.zomes ) {
	    zome_manifest.dependencies	= null;
	}

	expect( src_manifest		).to.deep.equal( new_manifest );
	expect( src_resources		).to.deep.equal( bundle.resources );
	expect( src_msgpack_hash	).to.deep.equal( sha256( bundle.msgpack_source ) );
    });

    after(async function () {
	await client.close();
    });
}
