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
    dnaConfig,
    happConfig,
    webhappConfig,
    sha256,
}					from '../utils.js';
import apps_suite			from './apphub/apps_suite.js';
import webapps_suite			from './apphub/webapps_suite.js';
import webapp_packages_suite		from './apphub/webapp_packages_suite.js';
import webapp_package_versions_suite	from './apphub/webapp_package_versions_suite.js';


const __dirname				= path.dirname( new URL(import.meta.url).pathname );
const APPHUB_DNA_PATH			= path.join( __dirname, "../../dnas/apphub.dna" );
const DNAHUB_DNA_PATH			= path.join( __dirname, "../../dnas/dnahub.dna" );
const ZOMEHUB_DNA_PATH			= path.join( __dirname, "../../dnas/zomehub.dna" );

let app_port;


describe("AppHub", function () {
    const holochain			= new Holochain({
	"timeout": 60_000,
	"default_stdout_loggers": log.level_rank > 3,
    });

    before(async function () {
	this.timeout( 120_000 );

	await holochain.install([
	    "alice",
	    "bobby",
	], [
	    {
		"app_name": "test",
		"bundle": {
		    "apphub":	APPHUB_DNA_PATH,
		    "dnahub":	DNAHUB_DNA_PATH,
		    "zomehub":	ZOMEHUB_DNA_PATH,
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

	client				= new AppInterfaceClient( app_port, {
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

    it("should get some DNA", async function () {
	const dna_entry			= await apphub_csr.get_app_dna_entry({
	    "app_entry": app1.$addr,
	    "name": "fake-role-1",
	});

	log.normal("%s", json.debug(dna_entry) );
    });

    it("should get hApp bundle", async function () {
	const bundle_bytes		= await apphub_csr.get_happ_bundle( app1.$addr );
	const bundle			= new Bundle( bundle_bytes, "happ" );

	log.normal("hApp bundle: %s", json.debug(bundle) );
    });

    it("should get hApp package", async function () {
	const app_package		= await apphub_csr.get_app_package( app1.$addr );

	log.normal("App package: %s", json.debug(app_package) );

	const manifest			= app_package.app_entry.manifest;

	for ( let role_manifest of manifest.roles ) {
	    delete role_manifest.dna.dna_hrl;

	    const dna_package		= app_package.dna_packages[ role_manifest.name ];
	    const dna_manifest		= dna_package.dna_entry.manifest;

	    for ( let zome_manifest of dna_manifest.integrity.zomes ) {
		delete zome_manifest.wasm_hrl;
		delete zome_manifest.dependencies;

		const compressed_bytes	= new Uint8Array(
		    dna_package.wasm_packages[ zome_manifest.name ]
		);

		zome_manifest.bytes		= await zomehub.zomes.mere_memory_api.functions.gzip_uncompress(
		    compressed_bytes
		);
	    }

	    for ( let zome_manifest of dna_manifest.coordinator.zomes ) {
		delete zome_manifest.wasm_hrl;

		const compressed_bytes	= new Uint8Array(
		    dna_package.wasm_packages[ zome_manifest.name ]
		);

		zome_manifest.bytes		= await zomehub.zomes.mere_memory_api.functions.gzip_uncompress(
		    compressed_bytes
		);
	    }

	    const dna_bundle		= Bundle.createDna( dna_manifest );
	    const dna_bundle_bytes	= dna_bundle.toBytes();

	    role_manifest.dna.bytes	= dna_bundle_bytes;
	}

	const bundle1			= Bundle.createHapp( TEST_HAPP_CONFIG );
	const bundle1_bytes		= bundle1.toBytes();
	const bundle2			= Bundle.createHapp( manifest );
	const bundle2_bytes		= bundle2.toBytes();

	log.normal("Bundle original: %s", json.debug(bundle1) );
	log.normal("Bundle packaged: %s", json.debug(bundle2) );

	log.normal("Bundle original: %s", json.debug(bundle1_bytes) );
	log.normal("Bundle packaged: %s", json.debug(bundle2_bytes) );

	log.normal(
	    "Bundle hashes: %s === %s",
	    sha256(bundle1_bytes),
	    sha256(bundle2_bytes),
	);
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

    it("should get Webhapp bundle", async function () {
	const bundle_bytes		= await apphub_csr.get_webhapp_bundle( webapp1.$addr );
	const bundle			= new Bundle( bundle_bytes, "webhapp" );

	log.normal("Webhapp bundle: %s", json.debug(bundle) );
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
