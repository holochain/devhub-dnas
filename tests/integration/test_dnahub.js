import { Logger }			from '@whi/weblogger';
const log				= new Logger("test-dnahub-basic", process.env.LOG_LEVEL );

import why				from 'why-is-node-running';

import * as fs				from 'node:fs/promises';
import path				from 'path';
import crypto				from 'crypto';

import { expect }			from 'chai';

import json				from '@whi/json';
import {
    Bundle,
}					from '@spartan-hc/bundles';
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
    dnaConfig,
    sha256,
    delay,
}					from '../utils.js';


const __dirname				= path.dirname( new URL(import.meta.url).pathname );
const DNAHUB_DNA_PATH			= path.join( __dirname, "../../dnas/dnahub.dna" );
const ZOMEHUB_DNA_PATH			= path.join( __dirname, "../../dnas/zomehub.dna" );

let app_port;
let installations;


describe("DnaHub", function () {
    const holochain			= new Holochain({
	"timeout": 120_000,
	"default_stdout_loggers": log.level_rank > 3,
    });

    before(async function () {
	this.timeout( 120_000 );

	installations			= await holochain.install([
	    "alice",
	    "bobby",
	], [
	    {
		"app_name": "test",
		"bundle": {
		    "dnahub":	DNAHUB_DNA_PATH,
		    "zomehub":	ZOMEHUB_DNA_PATH,
		},
	    },
	]);

	app_port			= await holochain.ensureAppPort();
    });

    linearSuite("Basic", basic_tests );

    after(async () => {
	await holochain.destroy();
    });
});


const TEST_DNA_CONFIG			= dnaConfig();


function basic_tests () {
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
	    "zomehub":		ZomeHubCell,
	    "dnahub":		DnaHubCell,
	}));

	zomehub_csr			= zomehub.zomes.zomehub_csr.functions;
	dnahub_csr			= dnahub.zomes.dnahub_csr.functions;

	await zomehub_csr.whoami();
	await dnahub_csr.whoami();
    });

    it("should upload DNA bundle", async function () {
	const bundle			= Bundle.createDna( TEST_DNA_CONFIG );
	const bundle_bytes		= bundle.toBytes();

	dna1				= await dnahub_csr.save_dna( bundle_bytes );

	expect( dna1			).to.be.a("Dna");
    });

    it("should get DNA entry", async function () {
	const dna			= await dnahub_csr.get_dna_entry( dna1.$addr );

	log.normal("DNA entry: %s", json.debug(dna) );
    });

    it("should get some zome (with bytes)", async function () {
	const zome			= await dnahub_csr.get_integrity_zome({
	    "dna_entry": dna1.$addr,
	    "name": "fake-zome-1",
	});

	log.normal("ZOME [fake-zome-1]: %s", json.debug(zome) );
    });

    it("should get DNA bundle", async function () {
	const bundle_bytes		= await dnahub_csr.get_dna_bundle( dna1.$addr );
	const bundle			= new Bundle( bundle_bytes, "dna" );

	log.normal("DNA bundle: %s", json.debug(bundle) );
    });

    it("should get DNA asset", async function () {
	const dna_asset			= await dnahub_csr.get_dna_asset( dna1.$addr );

	log.normal("DNA asset: %s", json.debug(dna_asset) );

	const bundle1			= Bundle.createDna( TEST_DNA_CONFIG );
	const bundle1_bytes		= bundle1.toBytes();
	const bundle2			= await dnahub_csr.bundle_from_dna_asset( dna_asset );
	const bundle2_bytes		= bundle2.toBytes();

	log.normal("Bundle original: %s", json.debug(bundle1) );
	log.normal("Bundle packaged: %s", json.debug(bundle2) );

	log.normal("Bundle original: %s", json.debug(bundle1_bytes) );
	log.normal("Bundle packaged: %s", json.debug(bundle2_bytes) );

	log.normal(
	    "Bundle hashes: %s === %s",
	    sha256(bundle1_bytes),
	    sha256(bundle2_bytes),
	);
    });

    it("should upload the same DNA bundle", async function () {
	const bundle			= Bundle.createDna( TEST_DNA_CONFIG );
	const bundle_bytes		= bundle.toBytes();

	const dna			= await dnahub_csr.save_dna( bundle_bytes );

	expect( dna.$addr		).to.deep.equal( dna1.$addr );
    });

    linearSuite("Errors", function () {

	it("should fail to create DNA entry because of wrong invalid DNA token", async function () {
	    await expect_reject(async () => {
		const entry		= await dnahub_csr.get_dna_entry( dna1.$addr );

		entry.dna_token.integrity_hash = crypto.randomBytes( 32 );

		await dnahub_csr.create_dna_entry( entry );
	    }, "Invalid DNA Token" );
	});

	it("should fail to create DNA entry because of wrong invalid integrities token", async function () {
	    await expect_reject(async () => {
		const entry		= await dnahub_csr.get_dna_entry( dna1.$addr );

		entry.integrities_token[0][1] = crypto.randomBytes( 32 );

		await dnahub_csr.create_dna_entry( entry );
	    }, "Invalid Integrities Token" );
	});

	it("should fail to create DNA entry because of wrong invalid coordinators token", async function () {
	    await expect_reject(async () => {
		const entry		= await dnahub_csr.get_dna_entry( dna1.$addr );

		entry.coordinators_token[0][1] = crypto.randomBytes( 32 );

		await dnahub_csr.create_dna_entry( entry );
	    }, "Invalid Coordinators Token" );
	});

	it("should fail to update DNA entry");

	it("should fail to delete DNA entry because author", async function () {
	    const bundle		= Bundle.createDna( TEST_DNA_CONFIG );
	    const dna_bytes		= bundle.toBytes();

	    let dna			= await dnahub_csr.save_dna( dna_bytes );

	    const app_token		= installations.bobby.test.auth.token;
	    const bobby_client		= await client.app( app_token );
	    const bobby_dnahub_csr	= bobby_client
		  .createCellInterface( "dnahub", DnaHubCell )
		  .zomes.dnahub_csr.functions;

            await delay();

	    await expect_reject(async () => {
		await bobby_dnahub_csr.delete_dna( dna.$id );
	    }, "Not authorized to delete entry created by author" );
	});

    });

    after(async function () {
	await client.close();
    });
}
