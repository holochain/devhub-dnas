import { Logger }			from '@whi/weblogger';
const log				= new Logger("test-dna_hub-basic", process.env.LOG_LEVEL );

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
}					from '@holochain/zome-hub-zomelets';
import {
    DnaHubCSRZomelet,
}					from '@holochain/dna-hub-zomelets';
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
const DNA_PATH				= path.join( __dirname, "../../dnas/dna_hub.dna" );
const ZOMEHUB_DNA_PATH			= path.join( __dirname, "../../dnas/zome_hub.dna" );
const APP_PORT				= 23_567;

const DNA_NAME				= "dna_hub";
const ZOMEHUB_DNA_NAME			= "zome_hub";

const zome_hub_spec			= new CellZomelets({
    "zome_hub_csr": ZomeHubCSRZomelet,
    "mere_memory_api": MereMemoryZomelet,
});

const dna_hub_spec			= new CellZomelets({
    "dna_hub_csr": DnaHubCSRZomelet,
});


describe("DnaHub", function () {
    const holochain			= new Holochain({
	"timeout": 60_000,
	"default_stdout_loggers": process.env.LOG_LEVEL === "trace",
    });

    before(async function () {
	this.timeout( 60_000 );

	const actors			= await holochain.backdrop({
	    "test": {
		[DNA_NAME]:		DNA_PATH,
		[ZOMEHUB_DNA_NAME]:	ZOMEHUB_DNA_PATH,
	    },
	}, {
	    "app_port": APP_PORT,
	});

	const cell			= actors.alice.test.cells[ DNA_NAME ];
	await holochain.admin.grantUnrestrictedCapability(
	    "testing", cell.agent, cell.dna, "*"
	);
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
    let zome_hub;
    let zome_hub_csr;
    let dna_hub;
    let dna_hub_csr;
    let dna1_addr, dna_entry;

    before(async function () {
	client				= new AppInterfaceClient( APP_PORT, {
	    "logging": process.env.LOG_LEVEL || "normal",
	});
	app_client			= await client.app( "test-alice" );

	app_client.setCellZomelets( DNA_NAME,		dna_hub_spec );
	app_client.setCellZomelets( ZOMEHUB_DNA_NAME,	zome_hub_spec );

	zome_hub			= app_client.cells.zome_hub.zomes;
	zome_hub_csr			= zome_hub.zome_hub_csr.functions;

	dna_hub				= app_client.cells.dna_hub.zomes;
	dna_hub_csr			= dna_hub.dna_hub_csr.functions;
    });

    it("should call whoami", async function () {
	this.timeout( 30_000 );

	{
	    const agent_info		= await zome_hub_csr.whoami();
	    log.trace("ZomeHub: %s", json.debug(agent_info) );
	}
	{
	    const agent_info		= await dna_hub_csr.whoami();
	    log.trace("DnaHub: %s", json.debug(agent_info) );
	}
    });

    it("should create DNA entry", async function () {
	this.timeout( 30_000 );

	const dna_bytes			= await fs.readFile( ZOMEHUB_DNA_PATH );

	dna1_addr			= await dna_hub_csr.save_dna( dna_bytes );

	expect( dna1_addr		).to.be.a("ActionHash");
    });

    it("should get DNA entry", async function () {
	dna_entry			= await dna_hub_csr.get_dna_entry( dna1_addr );
	log.trace("%s", json.debug(dna_entry) );

	// expect( dna_entry		).to.have.any.keys( "mere_memory_addr" );
    });

    after(async function () {
	await client.close();
    });
}
