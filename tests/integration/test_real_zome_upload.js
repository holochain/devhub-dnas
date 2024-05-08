import { Logger }			from '@whi/weblogger';
const log				= new Logger("test-real-zome", process.env.LOG_LEVEL );

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
import { Holochain }			from '@spartan-hc/holochain-backdrop';

import {
    ZomeHubCSRZomelet,
    MereMemoryZomelet,
}					from '@holochain/zomehub-zomelets';
import {
    AppInterfaceClient,
    CellZomelets,
    Zomelet,
    Transformer,
}					from '@spartan-hc/app-interface-client';

import {
    expect_reject,
    linearSuite,
}					from '../utils.js';


const __dirname				= path.dirname( new URL(import.meta.url).pathname );
const ZOMEHUB_DNA_PATH			= path.join( __dirname, "../../dnas/zomehub.dna" );

const ZOMEHUB_DNA_NAME			= "zomehub";
const MAIN_ZOME				= "zomehub_csr";
const MERE_ZOME				= "mere_memory_api";

const zomehub_spec			= new CellZomelets({
    [MAIN_ZOME]: ZomeHubCSRZomelet,
    [MERE_ZOME]: MereMemoryZomelet,
});

let agents				= {};
let app_port;
let installations;


describe("ZomeHub - Real", function () {
    const holochain			= new Holochain({
	"timeout": 60_000,
	"default_stdout_loggers": log.level_rank > 3,
    });

    before(async function () {
	this.timeout( 60_000 );

	installations			= await holochain.install([
	    "alice",
	], [
	    {
		"app_name": "test",
		"bundle": {
		    [ZOMEHUB_DNA_NAME]: ZOMEHUB_DNA_PATH,
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
    let zome1_addr, zome1;

    before(async function () {
	this.timeout( 30_000 );

	client				= new AppInterfaceClient( app_port, {
	    "logging": process.env.LOG_LEVEL || "normal",
	});

	const app_token			= installations.alice.test.auth.token;
	app_client			= await client.app( app_token );

	({
	    zomehub,
	}				= app_client.createInterface({
	    [ZOMEHUB_DNA_NAME]:	zomehub_spec
	}));

	zomehub_csr			= zomehub.zomes.zomehub_csr.functions;

	await zomehub_csr.whoami();
    });

    it("should create zome entry", async function () {
	this.timeout( 10_000 );

	const ZOME_PATH			= path.join( __dirname, "../../zomes/zomehub.wasm" );
	const zome_bytes		= await fs.readFile( ZOME_PATH );

	zome1				= await zomehub_csr.save_integrity( zome_bytes );
	zome1_addr			= zome1.$addr;

	expect( zome1_addr		).to.be.a("EntryHash");
    });

    it("should get zome entry", async function () {
	const zome			= await zomehub_csr.get_zome_entry( zome1_addr );
	log.trace("%s", json.debug(zome) );

	expect( zome			).to.have.any.keys( "mere_memory_addr" );
    });

    after(async function () {
	await client.close();
    });
}
