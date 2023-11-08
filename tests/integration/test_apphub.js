import { Logger }			from '@whi/weblogger';
const log				= new Logger("test-dnahub-basic", process.env.LOG_LEVEL );

import why				from 'why-is-node-running';

import path				from 'path';
import crypto				from 'crypto';

import { expect }			from 'chai';
import { faker }			from '@faker-js/faker';

import json				from '@whi/json';
import {
    Bundle,
}					from '@spartan-hc/bundles';
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
    dnaConfig,
    happConfig,
    webhappConfig,
}					from '../utils.js';
import apps_suite			from './apphub/apps_suite.js';
import webapps_suite			from './apphub/webapps_suite.js';
import webapp_packages_suite		from './apphub/webapp_packages_suite.js';
import webapp_package_versions_suite	from './apphub/webapp_package_versions_suite.js';


const __dirname				= path.dirname( new URL(import.meta.url).pathname );
const APPHUB_DNA_PATH			= path.join( __dirname, "../../dnas/apphub.dna" );
const DNAHUB_DNA_PATH			= path.join( __dirname, "../../dnas/dnahub.dna" );
const ZOMEHUB_DNA_PATH			= path.join( __dirname, "../../dnas/zomehub.dna" );
const APP_PORT				= 23_567;



describe("AppHub", function () {
    const holochain			= new Holochain({
	"timeout": 60_000,
	"default_stdout_loggers": log.level_rank > 3,
    });

    before(async function () {
	this.timeout( 60_000 );

	await holochain.backdrop({
	    "test": {
		"apphub":	APPHUB_DNA_PATH,
		"dnahub":	DNAHUB_DNA_PATH,
		"zomehub":	ZOMEHUB_DNA_PATH,
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


const TEST_DNA_CONFIG			= dnaConfig();
const TEST_HAPP_CONFIG			= happConfig([{
    "name": "fake-role-1",
    "dna": {
	"bytes": Bundle.createDna( TEST_DNA_CONFIG ).toBytes(),
    },
}]);
const TEST_WEBHAPP_CONFIG		= webhappConfig({
    "bytes": Bundle.createHapp( TEST_HAPP_CONFIG ).toBytes(),
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
    let app1;
    let webapp1;

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
	    "zomehub":		ZomeHubCell,
	    "dnahub":		DnaHubCell,
	    "apphub":		AppHubCell,
	}));

	zomehub_csr			= zomehub.zomes.zomehub_csr.functions;
	dnahub_csr			= dnahub.zomes.dnahub_csr.functions;
	apphub_csr			= apphub.zomes.apphub_csr.functions;

	await zomehub_csr.whoami();
	await dnahub_csr.whoami();
	await apphub_csr.whoami();
    });

    it("should upload App bundle", async function () {
	const bundle			= Bundle.createHapp( TEST_HAPP_CONFIG );
	const bundle_bytes		= bundle.toBytes();

	app1				= await apphub_csr.save_app( bundle_bytes );

	expect( app1.$addr		).to.be.a("EntryHash");
    });

    it("should get App entry", async function () {
	const app			= await apphub_csr.get_app_entry( app1.$addr );

	log.normal("%s", json.debug(app) );
    });

    it("should upload the same App bundle", async function () {
	const bundle			= Bundle.createHapp( TEST_HAPP_CONFIG );
	const bundle_bytes		= bundle.toBytes();

	const app			= await apphub_csr.save_app( bundle_bytes );

	expect( app.$addr		).to.deep.equal( app1.$addr );
    });

    it("should upload WebApp bundle", async function () {
	const bundle			= Bundle.createWebhapp( TEST_WEBHAPP_CONFIG );
	const bundle_bytes		= bundle.toBytes();

	webapp1				= await apphub_csr.save_webapp( bundle_bytes );

	expect( webapp1.$addr		).to.be.a("EntryHash");
    });

    it("should get WebApp entry", async function () {
	const webapp			= await apphub_csr.get_webapp_entry( webapp1.$addr );

	log.normal("%s", json.debug(webapp) );
    });

    it("should upload the same WebApp bundle", async function () {
	const bundle			= Bundle.createWebhapp( TEST_WEBHAPP_CONFIG );
	const bundle_bytes		= bundle.toBytes();

	const webapp			= await apphub_csr.save_webapp( bundle_bytes );

	expect( webapp.$addr		).to.deep.equal( webapp1.$addr );
    });

    function common_args_plus( args ) {
	return Object.assign({
	    client,
	    app_client,
	    zomehub,
	    dnahub,
	    apphub,
	    zomehub_csr,
	    dnahub_csr,
	    apphub_csr,
	}, args );
    }

    linearSuite("Apps", apps_suite, () => common_args_plus() );
    linearSuite("WebApps", webapps_suite, () => common_args_plus() );

    linearSuite("WebApp Packages", webapp_packages_suite, () => common_args_plus({
	"webapp1_addr": webapp1.$addr,
    }));

    linearSuite("WebApp Package Versions", webapp_package_versions_suite, () => common_args_plus({
	"webapp1_addr": webapp1.$addr,
    }));

    after(async function () {
	await client.close();
    });
}
