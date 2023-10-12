import { Logger }			from '@whi/weblogger';
const log				= new Logger("test-real-app", process.env.LOG_LEVEL );

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
    AppHubCell,
    DnaHubCell,
    ZomeHubCell,
}					from '@holochain/apphub-zomelets';
import {
    AppInterfaceClient,
}					from '@spartan-hc/app-interface-client';

import {
    expect_reject,
    linearSuite,
}					from '../utils.js';


const __dirname				= path.dirname( new URL(import.meta.url).pathname );
const DNA_PATH				= path.join( __dirname, "../../dnas/apphub.dna" );
const DNAHUB_DNA_PATH			= path.join( __dirname, "../../dnas/dnahub.dna" );
const ZOMEHUB_DNA_PATH			= path.join( __dirname, "../../dnas/zomehub.dna" );
const DEVHUB_APP_PATH			= path.join( __dirname, "../../happ/devhub.happ" );
const APP_PORT				= 23_567;

const DNA_NAME				= "apphub";
const DNAHUB_DNA_NAME			= "dnahub";
const ZOMEHUB_DNA_NAME			= "zomehub";



describe("AppHub - Real", function () {
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

    linearSuite("Upload", real_tests );

    after(async () => {
	await holochain.destroy();
    });
});


const k					= obj => Object.keys( obj );


function real_tests () {
    let client;
    let app_client;
    let zomehub;
    let zomehub_csr;
    let dnahub;
    let dnahub_csr;
    let apphub;
    let apphub_csr;
    let app1_addr, app_entry;

    before(async function () {
	this.timeout( 30_000 );

	client				= new AppInterfaceClient( APP_PORT, {
	    "logging": process.env.LOG_LEVEL || "normal",
	});
	app_client			= await client.app( "test-alice" );

	app_client.setCellZomelets( DNA_NAME,		AppHubCell );
	app_client.setCellZomelets( DNAHUB_DNA_NAME,	DnaHubCell );
	app_client.setCellZomelets( ZOMEHUB_DNA_NAME,	ZomeHubCell );

	zomehub				= app_client.cells.zomehub.zomes;
	zomehub_csr			= zomehub.zomehub_csr.functions;

	dnahub				= app_client.cells.dnahub.zomes;
	dnahub_csr			= dnahub.dnahub_csr.functions;

	apphub				= app_client.cells.apphub.zomes;
	apphub_csr			= apphub.apphub_csr.functions;

	await zomehub_csr.whoami();
	await dnahub_csr.whoami();
	await apphub_csr.whoami();
    });

    it("should create App entry", async function () {
	this.timeout( 30_000 );

	const app_bytes			= await fs.readFile( DEVHUB_APP_PATH );

	app1_addr			= await apphub_csr.save_app( app_bytes );

	expect( app1_addr		).to.be.a("ActionHash");
    });

    it("should get App entry", async function () {
	app_entry			= await apphub_csr.get_app_entry( app1_addr );
	log.trace("%s", json.debug(app_entry) );
    });

    after(async function () {
	await client.close();
    });
}
