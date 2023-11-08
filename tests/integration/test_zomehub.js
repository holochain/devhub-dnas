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
    ZomeHubCell,
    WASM_TYPES,
}					from '@holochain/zomehub-zomelets';
import {
    AppInterfaceClient,
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


describe("ZomeHub", function () {
    const holochain			= new Holochain({
	"timeout": 60_000,
	"default_stdout_loggers": log.level_rank > 3,
    });

    before(async function () {
	this.timeout( 60_000 );

	await holochain.backdrop({
	    "test": {
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

const wasm1_bytes			= crypto.randomBytes( 1_000 );


function basic_tests () {
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
	    [ZOMEHUB_DNA_NAME]:	ZomeHubCell,
	}));

	zomehub_csr			= zomehub.zomes.zomehub_csr.functions;

	await zomehub_csr.whoami();
    });

    it("should save integrity wasm", async function () {
	wasm1				= await zomehub_csr.save_integrity( wasm1_bytes );

	expect( wasm1			).to.be.a("Wasm");

	wasm1_addr			= wasm1.$addr;
    });

    it("should get wasm entry", async function () {
	const wasm			= await zomehub_csr.get_wasm_entry( wasm1_addr );
	log.trace("%s", json.debug(wasm) );

	expect( wasm			).to.have.any.keys( "mere_memory_addr" );
    });

    it("should get all wasm entries for agent", async function () {
	const wasm_entries		= await zomehub_csr.get_wasm_entries_for_agent();
	log.trace("%s", json.debug(wasm_entries) );

	expect( wasm_entries		).to.have.length( 1 );
    });

    it("should upload the same wasm", async function () {
	const wasm			= await zomehub_csr.save_integrity( wasm1_bytes );

	expect( wasm.$addr		).to.deep.equal( wasm1_addr );
    });

    it("should delete wasm", async function () {
	const result			= await zomehub_csr.delete_wasm( wasm1.$id );

	expect( result			).to.be.a("ActionHash");
    });

    linearSuite("Errors", function () {

	it("should fail to create wasm entry because of wrong file size", async function () {
	    await expect_reject(async () => {
		await zomehub_csr.create_wasm_entry({
		    "wasm_type": WASM_TYPES.INTEGRITY,
		    "mere_memory_addr": wasm1.mere_memory_addr,
		    "file_size": 0,
		});
	    }, "file size does not match memory address" );
	});

	it("should fail to update wasm entry");

	it("should fail to delete wasm entry because author", async function () {
	    let wasm			= await zomehub_csr.save_integrity( wasm1_bytes );

	    const bobby_client		= await client.app( "test-bobby" );
	    const bobby_zomehub_csr	= bobby_client
		  .createCellInterface( ZOMEHUB_DNA_NAME, ZomeHubCell )
		  .zomes.zomehub_csr.functions;

	    await expect_reject(async () => {
		await bobby_zomehub_csr.delete_wasm( wasm.$id );
	    }, "Not authorized to delete entry created by author" );
	});

    });

    after(async function () {
	await client.close();
    });
}
