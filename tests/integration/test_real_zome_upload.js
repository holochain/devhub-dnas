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
import HolochainBackdrop		from '@spartan-hc/holochain-backdrop';
const { Holochain }			= HolochainBackdrop;

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
const APP_PORT				= 23_567;

const ZOMEHUB_DNA_NAME			= "zomehub";
const MAIN_ZOME				= "zomehub_csr";
const MERE_ZOME				= "mere_memory_api";

const zomehub_spec			= new CellZomelets({
    [MAIN_ZOME]: ZomeHubCSRZomelet,
    [MERE_ZOME]: MereMemoryZomelet,
});

let agents				= {};

describe("ZomeHub - Real", function () {
    const holochain			= new Holochain({
	"timeout": 60_000,
	"default_stdout_loggers": log.level_rank > 3,
    });

    before(async function () {
	this.timeout( 60_000 );

	const actors			= await holochain.backdrop({
	    "test": {
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
    let wasm1_addr, wasm1;

    before(async function () {
	this.timeout( 30_000 );

	client				= new AppInterfaceClient( APP_PORT, {
	    "logging": process.env.LOG_LEVEL || "normal",
	});
	app_client			= await client.app( "test-alice" );

	({
	    zomehub,
	}				= app_client.createInterface({
	    [ZOMEHUB_DNA_NAME]:	zomehub_spec
	}));

	zomehub_csr			= zomehub.zomes.zomehub_csr.functions;

	await zomehub_csr.whoami();
    });

    it("should create wasm entry", async function () {
	this.timeout( 10_000 );

	const WASM_PATH			= path.join( __dirname, "../../zomes/zomehub.wasm" );
	const wasm_bytes		= await fs.readFile( WASM_PATH );

	wasm1				= await zomehub_csr.save_integrity( wasm_bytes );
	wasm1_addr			= wasm1.$addr;

	expect( wasm1_addr		).to.be.a("EntryHash");
    });

    it("should get wasm entry", async function () {
	const wasm			= await zomehub_csr.get_wasm_entry( wasm1_addr );
	log.trace("%s", json.debug(wasm) );

	expect( wasm			).to.have.any.keys( "mere_memory_addr" );
    });

    after(async function () {
	await client.close();
    });
}
