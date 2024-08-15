import { Logger }			from '@whi/weblogger';
const log				= new Logger("test-real-app", process.env.LOG_LEVEL );

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
    AppHubCell,
    DnaHubCell,
    ZomeHubCell,
}					from '@holochain/apphub-zomelets';
import {
    AppInterfaceClient,
}					from '@spartan-hc/app-interface-client';

import {
    expect_reject,
    linearSuite,
    sha256,
}					from '../utils.js';


const __dirname				= path.dirname( new URL(import.meta.url).pathname );
const APPHUB_DNA_PATH			= path.join( __dirname, "../../dnas/apphub.dna" );
const DNAHUB_DNA_PATH			= path.join( __dirname, "../../dnas/dnahub.dna" );
const ZOMEHUB_DNA_PATH			= path.join( __dirname, "../../dnas/zomehub.dna" );
const DEVHUB_APP_PATH			= path.join( __dirname, "../../happ/devhub.happ" );
const APP_PORT				= 23_567;

const APPHUB_DNA_NAME			= "apphub";
const DNAHUB_DNA_NAME			= "dnahub";
const ZOMEHUB_DNA_NAME			= "zomehub";

let app_port;
let installations;


describe("AppHub - Real", function () {
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
		    [APPHUB_DNA_NAME]:	APPHUB_DNA_PATH,
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
    let apphub;
    let apphub_csr;
    let app1;

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
	    apphub
	}				= app_client.createInterface({
	    [ZOMEHUB_DNA_NAME]:		ZomeHubCell,
	    [DNAHUB_DNA_NAME]:		DnaHubCell,
	    [APPHUB_DNA_NAME]:		AppHubCell,
	}));

	zomehub_csr			= zomehub.zomes.zomehub_csr.functions;
	dnahub_csr			= dnahub.zomes.dnahub_csr.functions;
	apphub_csr			= apphub.zomes.apphub_csr.functions;

	await zomehub_csr.whoami();
	await dnahub_csr.whoami();
	await apphub_csr.whoami();
    });

    let src_bundle;

    it("should create App entry", async function () {
	this.timeout( 30_000 );

	const app_bytes			= await fs.readFile( DEVHUB_APP_PATH );

	app1				= await apphub_csr.save_app( app_bytes );

	expect( app1.$addr		).to.be.a("EntryHash");

	// Setup source bundle for comparison later
	{
	    src_bundle			= new Bundle( app_bytes, "happ" );

	    src_bundle.dnas().forEach( (dna_bundle, i) => {
		const role_manifest		= src_bundle.manifest.roles[i];
		const rpath			= role_manifest.dna.bundled;

		// Replace DNA bytes with deterministic bytes
		src_bundle.resources[ rpath ]	= dna_bundle.toBytes({ sortKeys: true });
	    });
	}
    });

    it("should get App entry", async function () {
	const app			= await apphub_csr.get_app_entry( app1.$addr );

	log.normal("%s", json.debug(app) );
    });

    it("should get hApp bundle", async function () {
	this.timeout( 60_000 );

	const bundle_bytes		= await apphub_csr.get_happ_bundle( app1.$addr );
	const bundle			= new Bundle( bundle_bytes, "happ" );

	bundle.dnas().forEach( (dna_bundle, i) => {
	    // Add integrity's 'dependencies' field back in for comparing against source bundle;
	    // which has the field because `hc` bundler adds it.
	    for ( let zome_manifest of dna_bundle.manifest.integrity.zomes ) {
		zome_manifest.dependencies	= null;
	    }

	    const role_manifest		= bundle.manifest.roles[i];
	    const rpath			= role_manifest.dna.bundled;

	    bundle.resources[ rpath ]	= dna_bundle.toBytes({ sortKeys: true });
	});

	const src_manifest		= src_bundle.manifest.toJSON();
	const src_msgpack_hash		= sha256( src_bundle.toEncoded({ sortKeys: true }) );

	const new_manifest		= bundle.manifest.toJSON();
	const new_msgpack_hash		= sha256( bundle.toEncoded({ sortKeys: true }) );

	expect( src_manifest		).to.deep.equal( new_manifest );
	expect( src_msgpack_hash	).to.equal( new_msgpack_hash );
    });

    after(async function () {
	await client.close();
    });
}
