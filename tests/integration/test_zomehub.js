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
import { Holochain }			from '@spartan-hc/holochain-backdrop';

import {
    ZomeHubCell,
    ZOME_TYPES,
}					from '@holochain/zomehub-zomelets';
import {
    AppInterfaceClient,
}					from '@spartan-hc/app-interface-client';

import {
    expect_reject,
    linearSuite,
}					from '../utils.js';
import zome_packages_suite		from './zomehub/zome_packages_suite.js';


const __dirname				= path.dirname( new URL(import.meta.url).pathname );
const ZOMEHUB_DNA_PATH			= path.join( __dirname, "../../dnas/zomehub.dna" );

const ZOMEHUB_DNA_NAME			= "zomehub";
const MAIN_ZOME				= "zomehub_csr";
const MERE_ZOME				= "mere_memory_api";

let app_port;
let installations;


describe("ZomeHub", function () {
    const holochain			= new Holochain({
	"timeout": 60_000,
	"default_stdout_loggers": log.level_rank > 3,
    });

    before(async function () {
	this.timeout( 60_000 );

	installations			= await holochain.install([
	    "alice",
	    "bobby",
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

    linearSuite("Basic", basic_tests );

    after(async () => {
	await holochain.destroy();
    });
});

const zome1_bytes			= crypto.randomBytes( 1_000 );


function basic_tests () {
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
	    [ZOMEHUB_DNA_NAME]:	ZomeHubCell,
	}));

	zomehub_csr			= zomehub.zomes.zomehub_csr.functions;

	await zomehub_csr.whoami();
    });

    it("should save integrity zome", async function () {
	zome1				= await zomehub_csr.save_integrity( zome1_bytes );

	expect( zome1			).to.be.a("Zome");

	zome1_addr			= zome1.$addr;
    });

    it("should get zome entry", async function () {
	const zome			= await zomehub_csr.get_zome_entry( zome1_addr );
	log.trace("%s", json.debug(zome) );

	expect( zome			).to.have.any.keys( "mere_memory_addr" );
    });

    it("should get zome asset", async function () {
	const zome_asset		= await zomehub_csr.get_zome_asset( zome1_addr );
	log.normal("%s", json.debug(zome_asset) );

	expect( zome_asset		).to.have.any.keys( "bytes" );
    });

    it("should get all zome entries for agent", async function () {
	const zome_entries		= await zomehub_csr.get_zome_entries_for_agent();
	log.trace("%s", json.debug(zome_entries) );

	expect( zome_entries		).to.have.length( 1 );
    });

    it("should upload the same zome", async function () {
	const zome			= await zomehub_csr.save_integrity( zome1_bytes );

	expect( zome.$addr		).to.deep.equal( zome1_addr );
    });

    it("should delete zome", async function () {
	const result			= await zomehub_csr.delete_zome( zome1.$id );

	expect( result			).to.be.a("ActionHash");
    });

    linearSuite("Errors", function () {

	it("should fail to create zome entry because of wrong file size", async function () {
	    await expect_reject(async () => {
		await zomehub_csr.create_zome_entry({
		    "zome_type": ZOME_TYPES.INTEGRITY,
		    "mere_memory_addr": zome1.mere_memory_addr,
		    "file_size": 0,
		    "hash": "a1a142877da15f0a46bfd9ec9450954dd8363846a8397413e342c91aef842b32",
		});
	    }, "file size does not match memory address" );
	});

	it("should fail to update zome entry");

	it("should fail to delete zome entry because author", async function () {
	    let zome			= await zomehub_csr.save_integrity( zome1_bytes );

	    const app_token		= installations.bobby.test.auth.token;
	    const bobby_client		= await client.app( app_token );
	    const bobby_zomehub_csr	= bobby_client
		  .createCellInterface( ZOMEHUB_DNA_NAME, ZomeHubCell )
		  .zomes.zomehub_csr.functions;

	    await expect_reject(async () => {
		await bobby_zomehub_csr.delete_zome( zome.$id );
	    }, "Not authorized to delete entry created by author" );
	});

    });

    function common_args_plus( args ) {
	return Object.assign({
	    installations,
	    client,
	    app_client,
	    zomehub,
	    zomehub_csr,
	    zome1_addr,
	    zome1,
	}, args );
    }

    linearSuite("Zome Packages", zome_packages_suite, () => common_args_plus() );

    after(async function () {
	await client.close();
    });
}
