import { Logger }			from '@whi/weblogger';
const log				= new Logger("test-real-dna", process.env.LOG_LEVEL );

// import why				from 'why-is-node-running';

import * as fs				from 'node:fs/promises';
import path				from 'path';
import crypto				from 'crypto';

import { expect }			from 'chai';

import json				from '@whi/json';
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
}					from '../utils.js';


const __dirname				= path.dirname( new URL(import.meta.url).pathname );
const DNAHUB_DNA_PATH			= path.join( __dirname, "../../dnas/dnahub.dna" );
const ZOMEHUB_DNA_PATH			= path.join( __dirname, "../../dnas/zomehub.dna" );
const APP_PORT				= 23_567;

const DNAHUB_DNA_NAME			= "dnahub";
const ZOMEHUB_DNA_NAME			= "zomehub";


describe("DnaHub - Real", function () {
    const holochain			= new Holochain({
	"timeout": 60_000,
	"default_stdout_loggers": process.env.LOG_LEVEL === "trace",
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
    let dna1_addr, dna_entry;

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

    it("should create DNA entry", async function () {
	this.timeout( 30_000 );

	const dna_bytes			= await fs.readFile( ZOMEHUB_DNA_PATH );

	dna1_addr			= await dnahub_csr.save_dna( dna_bytes );

	expect( dna1_addr		).to.be.a("EntryHash");
    });

    it("should get DNA entry", async function () {
	dna_entry			= await dnahub_csr.get_dna_entry( dna1_addr );
	log.trace("%s", json.debug(dna_entry) );

	// expect( dna_entry		).to.have.any.keys( "mere_memory_addr" );
    });

    after(async function () {
	await client.close();
    });
}
