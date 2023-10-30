import { Logger }			from '@whi/weblogger';
const log				= new Logger("test-zomehub-basic", process.env.LOG_LEVEL );

import why				from 'why-is-node-running';

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

describe("ZomeHub", function () {
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

    linearSuite("Basic", basic_tests );

    after(async () => {
	await holochain.destroy();
    });
});

const wasm1_bytes			= crypto.randomBytes( 1_000 );


function basic_tests () {
    let client;
    let app_client;
    let zomehub;
    let zomehub_csr;
    let wasm1_addr, wasm_entry;

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

	wasm1_addr			= await zomehub_csr.save_integrity( wasm1_bytes );

	expect( wasm1_addr		).to.be.a("EntryHash");
    });

    it("should get wasm entry", async function () {
	wasm_entry			= await zomehub_csr.get_wasm_entry( wasm1_addr );
	log.trace("%s", json.debug(wasm_entry) );

	expect( wasm_entry		).to.have.any.keys( "mere_memory_addr" );
    });

    it("should get all wasm entries for agent", async function () {
	const wasm_entries		= await zomehub_csr.get_wasm_entries_for_agent();
	log.trace("%s", json.debug(wasm_entries) );

	expect( wasm_entries		).to.have.length( 1 );
    });

    it("should upload the same wasm", async function () {
	const addr			= await zomehub_csr.save_integrity( wasm1_bytes );

	expect( addr			).to.deep.equal( wasm1_addr );
    });

    after(async function () {
	await client.close();
    });
}
