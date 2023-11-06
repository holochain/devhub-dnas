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
import HolochainBackdrop		from '@spartan-hc/holochain-backdrop';
const { Holochain }			= HolochainBackdrop;

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
}					from '../utils.js';


const __dirname				= path.dirname( new URL(import.meta.url).pathname );
const DNAHUB_DNA_PATH			= path.join( __dirname, "../../dnas/dnahub.dna" );
const ZOMEHUB_DNA_PATH			= path.join( __dirname, "../../dnas/zomehub.dna" );
const APP_PORT				= 23_567;

const DNAHUB_DNA_NAME			= "dnahub";
const ZOMEHUB_DNA_NAME			= "zomehub";


describe("DnaHub", function () {
    const holochain			= new Holochain({
	"timeout": 60_000,
	"default_stdout_loggers": log.level_rank > 3,
    });

    before(async function () {
	this.timeout( 60_000 );

	const actors			= await holochain.backdrop({
	    "test": {
		[DNAHUB_DNA_NAME]:	DNAHUB_DNA_PATH,
		[ZOMEHUB_DNA_NAME]:	ZOMEHUB_DNA_PATH,
	    },
	}, {
	    "app_port": APP_PORT,
	    "actors": [ "alice", "bobby" ],
	});
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
    let dna1_addr, dna1;

    before(async function () {
	this.timeout( 30_000 );

	client				= new AppInterfaceClient( APP_PORT, {
	    "logging": process.env.LOG_LEVEL || "normal",
	});
	app_client			= await client.app( "test-alice" );

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

    it("should upload DNA bundle", async function () {
	const bundle			= Bundle.createDna( TEST_DNA_CONFIG );
	const bundle_bytes		= bundle.toBytes();

	dna1				= await dnahub_csr.save_dna( bundle_bytes );

	expect( dna1			).to.be.a("Dna");

	dna1_addr			= dna1.$addr;
    });

    it("should get DNA entry", async function () {
	const dna			= await dnahub_csr.get_dna_entry( dna1_addr );

	log.normal("%s", json.debug(dna) );
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
		const entry		= await dnahub_csr.get_dna_entry( dna1_addr );

		entry.dna_token.integrity_hash = crypto.randomBytes( 32 );

		await dnahub_csr.create_dna_entry( entry );
	    }, "Invalid DNA Token" );
	});

	it("should fail to create DNA entry because of wrong invalid integrities token", async function () {
	    await expect_reject(async () => {
		const entry		= await dnahub_csr.get_dna_entry( dna1_addr );

		entry.integrities_token[0][1] = crypto.randomBytes( 32 );

		await dnahub_csr.create_dna_entry( entry );
	    }, "Invalid Integrities Token" );
	});

	it("should fail to create DNA entry because of wrong invalid coordinators token", async function () {
	    await expect_reject(async () => {
		const entry		= await dnahub_csr.get_dna_entry( dna1_addr );

		entry.coordinators_token[0][1] = crypto.randomBytes( 32 );

		await dnahub_csr.create_dna_entry( entry );
	    }, "Invalid Coordinators Token" );
	});

	it("should fail to update DNA entry");

	it("should fail to delete DNA entry because author", async function () {
	    const bundle		= Bundle.createDna( TEST_DNA_CONFIG );
	    const dna_bytes		= bundle.toBytes();

	    let dna			= await dnahub_csr.save_dna( dna_bytes );

	    const bobby_client		= await client.app( "test-bobby" );
	    const bobby_dnahub_csr	= bobby_client
		  .createCellInterface( DNAHUB_DNA_NAME, DnaHubCell )
		  .zomes.dnahub_csr.functions;

	    await expect_reject(async () => {
		await bobby_dnahub_csr.delete_dna( dna.$id );
	    }, "Not authorized to delete entry created by author" );
	});

    });

    after(async function () {
	await client.close();
    });
}
