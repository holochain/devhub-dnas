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
const TEST_WEBAPP_PATH			= path.join( __dirname, "../test.webhapp" );
const TEST_UI_PATH			= path.join( __dirname, "../test.zip" );
const APP_PORT				= 23_567;

const APPHUB_DNA_NAME			= "apphub";
const DNAHUB_DNA_NAME			= "dnahub";
const ZOMEHUB_DNA_NAME			= "zomehub";



describe("AppHub: WebApp", function () {
    const holochain			= new Holochain({
	"timeout": 60_000,
	"default_stdout_loggers": log.level_rank > 3,
    });

    before(async function () {
	this.timeout( 60_000 );

	const actors			= await holochain.backdrop({
	    "test": {
		[APPHUB_DNA_NAME]:	DNA_PATH,
		[DNAHUB_DNA_NAME]:	DNAHUB_DNA_PATH,
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


function basic_tests () {
    let client;
    let app_client;
    let zomehub;
    let zomehub_csr;
    let dnahub;
    let dnahub_csr;
    let apphub;
    let apphub_csr;
    let webapp1_addr;
    let pack1;
    let pack1_v1;

    before(async function () {
	this.timeout( 30_000 );

	client				= new AppInterfaceClient( APP_PORT, {
	    "logging": process.env.LOG_LEVEL || "normal",
	});
	app_client			= await client.app( "test-alice" );

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

	webapp1_addr			= await apphub_csr.save_webapp( webapp_bytes );
	// apphub.zomes.apphub_csr.prevCall().printTree();

	expect( webapp1_addr		).to.be.a("EntryHash");
    });

    it("should get App entry", async function () {
	const webapp_entry		= await apphub_csr.get_webapp_entry( webapp1_addr );

	log.normal("WebApp entry: %s", json.debug(webapp_entry) );
    });

    it("should create WebApp Package entry", async function () {
	pack1				= await apphub_csr.create_webapp_package_entry({
	    "title": faker.commerce.productName(),
	    "subtitle": faker.lorem.sentence(),
	    "description": faker.lorem.paragraphs( 2 ),
	    "icon": crypto.randomBytes( 1_000 ),
	    "source_code_url": faker.internet.url(),
	});

	log.normal("Create WebApp package result:", pack1 );
	log.normal("Create WebApp package JSON: %s", json.debug(pack1) );

	expect( pack1			).to.be.a("WebAppPackage");
    });

    it("should create WebApp Package Version entry", async function () {
	pack1_v1			= await apphub_csr.create_webapp_package_version({
	    "version": "0.1.0",
	    "for_package": pack1.$id,
	    "webapp": webapp1_addr,
	    "source_code_url": faker.internet.url(),
	});

	log.normal("Create WebApp package version result:", pack1_v1 );
	log.normal("Create WebApp package version JSON: %s", json.debug(pack1_v1) );

	expect( pack1_v1		).to.be.a("WebAppPackageVersion");
    });

    it("should get Version's WebApp Package", async function () {
	const result			= await pack1_v1.getWebAppPackage();

	expect( result			).to.deep.equal( pack1 );
    });

    it("should get WebApp Package versions (sorted with semver)", async function () {
	async function create_version ( vtag ) {
	    return await apphub_csr.create_webapp_package_version({
		"version": vtag,
		"for_package": pack1.$id,
		"webapp": webapp1_addr,
		"source_code_url": faker.internet.url(),
	    });
	}
	await create_version("0.1.0");
	await create_version("0.1.0-beta-rc.0");
	await create_version("0.1.0-beta-rc.1");
	await create_version("0.1.0-beta-rc.2");
	await create_version("0.1.0-beta-rc.11");

	const versions			= await pack1.versions();

	log.normal("WebApp package versions JSON: %s", json.debug(versions) );
	expect( versions[0]		).to.be.a("WebAppPackageVersion");
	expect( versions[0].version	).to.equal("0.1.0");
	expect( versions[1].version	).to.equal("0.1.0-beta-rc.11");
	expect( versions[2].version	).to.equal("0.1.0-beta-rc.2");
	expect( versions[3].version	).to.equal("0.1.0-beta-rc.1");
	expect( versions[4].version	).to.equal("0.1.0-beta-rc.0");
    });

    it("should get all WebApp Packages", async function () {
	const result			= await apphub_csr.get_all_webapp_package_entries();

	expect( result			).to.have.length( 1 );
    });

    after(async function () {
	await client.close();
    });
}
