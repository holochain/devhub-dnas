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
    let dna1_addr;

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

	dna1_addr			= await dnahub_csr.save_dna( bundle_bytes );

	expect( dna1_addr		).to.be.a("EntryHash");
    });

    it("should get DNA entry", async function () {
	const dna1			= await dnahub_csr.get_dna_entry( dna1_addr );

	log.normal("%s", json.debug(dna1) );
    });

    it("should upload the same DNA bundle", async function () {
	const bundle			= Bundle.createDna( TEST_DNA_CONFIG );
	const bundle_bytes		= bundle.toBytes();

	const addr			= await dnahub_csr.save_dna( bundle_bytes );

	expect( addr			).to.deep.equal( dna1_addr );
    });

    after(async function () {
	await client.close();
    });
}
