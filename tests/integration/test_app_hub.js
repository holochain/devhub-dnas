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
    AppHubCell,
    DnaHubCell,
    ZomeHubCell,
}					from '@holochain/app-hub-zomelets';
import {
    AppInterfaceClient,
}					from '@spartan-hc/app-interface-client';

import {
    expect_reject,
    linearSuite,
}					from '../utils.js';


const __dirname				= path.dirname( new URL(import.meta.url).pathname );
const DNA_PATH				= path.join( __dirname, "../../dnas/app_hub.dna" );
const DNAHUB_DNA_PATH			= path.join( __dirname, "../../dnas/dna_hub.dna" );
const ZOMEHUB_DNA_PATH			= path.join( __dirname, "../../dnas/zome_hub.dna" );
const DEVHUB_APP_PATH			= path.join( __dirname, "../../happ/devhub.happ" );
const APP_PORT				= 23_567;

const DNA_NAME				= "app_hub";
const DNAHUB_DNA_NAME			= "dna_hub";
const ZOMEHUB_DNA_NAME			= "zome_hub";



describe("AppHub", function () {
    const holochain			= new Holochain({
	"timeout": 60_000,
	"default_stdout_loggers": process.env.LOG_LEVEL === "trace",
    });

    before(async function () {
	this.timeout( 60_000 );

	const actors			= await holochain.backdrop({
	    "test": {
		[DNA_NAME]:		DNA_PATH,
		[DNAHUB_DNA_NAME]:	DNAHUB_DNA_PATH,
		[ZOMEHUB_DNA_NAME]:	ZOMEHUB_DNA_PATH,
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
    let zome_hub;
    let zome_hub_csr;
    let dna_hub;
    let dna_hub_csr;
    let app_hub;
    let app_hub_csr;
    let app1_addr, app_entry;

    before(async function () {
	client				= new AppInterfaceClient( APP_PORT, {
	    "logging": process.env.LOG_LEVEL || "normal",
	});
	app_client			= await client.app( "test-alice" );

	app_client.setCellZomelets( DNA_NAME,		AppHubCell );
	app_client.setCellZomelets( DNAHUB_DNA_NAME,	DnaHubCell );
	app_client.setCellZomelets( ZOMEHUB_DNA_NAME,	ZomeHubCell );

	zome_hub			= app_client.cells.zome_hub.zomes;
	zome_hub_csr			= zome_hub.zome_hub_csr.functions;

	dna_hub				= app_client.cells.dna_hub.zomes;
	dna_hub_csr			= dna_hub.dna_hub_csr.functions;

	app_hub				= app_client.cells.app_hub.zomes;
	app_hub_csr			= app_hub.app_hub_csr.functions;
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
	{
	    const agent_info		= await app_hub_csr.whoami();
	    log.trace("AppHub: %s", json.debug(agent_info) );
	}
    });

    it("should create App entry", async function () {
	this.timeout( 30_000 );

	const app_bytes			= await fs.readFile( DEVHUB_APP_PATH );

	app1_addr			= await app_hub_csr.save_app( app_bytes );

	expect( app1_addr		).to.be.a("ActionHash");
    });

    it("should get App entry", async function () {
	app_entry			= await app_hub_csr.get_app_entry( app1_addr );
	log.trace("%s", json.debug(app_entry) );
    });

    after(async function () {
	await client.close();
    });
}
