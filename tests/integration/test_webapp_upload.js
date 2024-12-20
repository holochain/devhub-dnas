import { Logger }			from '@whi/weblogger';
const log				= new Logger("test-webapp-upload", process.env.LOG_LEVEL );

import why				from 'why-is-node-running';

import * as fs				from 'node:fs/promises';
import path				from 'path';
import crypto				from 'crypto';

import { expect }			from 'chai';
import { faker }			from '@faker-js/faker';

import json				from '@whi/json';
import {
    HoloHash,
    DnaHash, AgentPubKey,
    ActionHash, EntryHash,
}					from '@spartan-hc/holo-hash';
import { Holochain }			from '@spartan-hc/holochain-backdrop';

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
const TEST_WEBAPP_PATH			= path.join( __dirname, "../test.webhapp" );
const TEST_UI_PATH			= path.join( __dirname, "../test.zip" );
const APP_PORT				= 23_567;

const APPHUB_DNA_NAME			= "apphub";
const DNAHUB_DNA_NAME			= "dnahub";
const ZOMEHUB_DNA_NAME			= "zomehub";

let app_port;
let installations;


describe("AppHub: WebApp", function () {
    const holochain			= new Holochain({
	"timeout": 120_000,
	"default_stdout_loggers": log.level_rank > 3,
    });

    before(async function () {
	this.timeout( 120_000 );

	installations			= await holochain.install([
	    "alice",
	], [
	    {
		"app_name": "test",
		"bundle": {
		    [APPHUB_DNA_NAME]:	DNA_PATH,
		    [DNAHUB_DNA_NAME]:	DNAHUB_DNA_PATH,
		    [ZOMEHUB_DNA_NAME]:	ZOMEHUB_DNA_PATH,
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


function basic_tests () {
    let client;
    let app_client;
    let zomehub;
    let zomehub_csr;
    let dnahub;
    let dnahub_csr;
    let apphub;
    let apphub_csr;
    let webapp1;

    before(async function () {
	this.timeout( 30_000 );

	client				= new AppInterfaceClient( app_port, {
	    "logging": process.env.LOG_LEVEL || "normal",
	});

	const app_token			= installations.alice.test.auth.token;
	app_client			= await client.app( app_token );

	({
	    zomehub,
	    dnahub,
	    apphub
	}				= app_client.createInterface({
	    [ZOMEHUB_DNA_NAME]:		ZomeHubCell,
	    [DNAHUB_DNA_NAME]:		DnaHubCell,
	    [APPHUB_DNA_NAME]:		AppHubCell,
	}));

	zomehub_csr			= zomehub.zomes.zomehub_csr.functions;
	dnahub_csr			= dnahub.zomes.dnahub_csr.functions;
	apphub_csr			= apphub.zomes.apphub_csr.functions;

	await zomehub_csr.whoami();
	await dnahub_csr.whoami();
	await apphub_csr.whoami();
    });

    it("should create WebApp entry", async function () {
	this.timeout( 30_000 );

	const webapp_bytes		= await fs.readFile( TEST_WEBAPP_PATH );

	webapp1				= await apphub_csr.save_webapp( webapp_bytes );
	// apphub.zomes.apphub_csr.prevCall().printTree();

	expect( webapp1.$addr		).to.be.a("EntryHash");
    });

    it("should get WebApp entry", async function () {
	const webapp			= await apphub_csr.get_webapp_entry( webapp1.$addr );

	log.normal("WebApp entry: %s", json.debug(webapp) );
    });

    after(async function () {
	await client.close();
    });
}
