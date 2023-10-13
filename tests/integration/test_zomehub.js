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
const DNA_PATH				= path.join( __dirname, "../../dnas/zomehub.dna" );
const APP_PORT				= 23_567;

const DNA_NAME				= "zomehub";
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
	"default_stdout_loggers": process.env.LOG_LEVEL === "trace",
    });

    before(async function () {
	this.timeout( 60_000 );

	const actors			= await holochain.backdrop({
	    "test": {
		[DNA_NAME]:	DNA_PATH,
	    },
	}, {
	    "app_port": APP_PORT,
	});
    });

    linearSuite( "Basic", basic_tests );

    after(async () => {
	await holochain.destroy();
    });
});


const k					= obj => Object.keys( obj );


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

	app_client.setCellZomelets( DNA_NAME, zomehub_spec );
	zomehub				= app_client.cells.zomehub.zomes;
	zomehub_csr			= zomehub.zomehub_csr.functions;

	await zomehub_csr.whoami();
    });

    it("should create wasm entry", async function () {
	this.timeout( 10_000 );

	const wasm_bytes		= crypto.randomBytes( 1_000 );

	wasm1_addr			= await zomehub_csr.save_wasm( wasm_bytes );

	expect( wasm1_addr		).to.be.a("ActionHash");
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

    after(async function () {
	await client.close();
    });
}
