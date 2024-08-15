import { Logger }			from '@whi/weblogger';
const log				= new Logger("apps-suite", process.env.LOG_LEVEL );

import crypto				from 'crypto';
import { expect }			from 'chai';
import { faker }			from '@faker-js/faker';

import json				from '@whi/json';
import {
    Bundle,
}					from '@spartan-hc/bundles';
import {
    AppHubCell,
}					from '@holochain/apphub-zomelets';

import {
    expect_reject,
    linearSuite,
    dnaConfig,
    happConfig,
    delay,
}					from '../../utils.js';


const TEST_DNA_CONFIG			= dnaConfig();
const TEST_HAPP_CONFIG			= happConfig([{
    "name": "fake-role-1",
    "dna": {
	"bytes": Bundle.createDna( TEST_DNA_CONFIG ).toBytes(),
    },
}]);


export default function ( args_fn ) {
    let installations;
    let client;
    let app_client;
    let zomehub;
    let zomehub_csr;
    let dnahub;
    let dnahub_csr;
    let apphub;
    let apphub_csr;
    let app1;

    before(async function () {
	({
	    installations,
	    client,
	    app_client,
	    zomehub,
	    dnahub,
	    apphub,
	    zomehub_csr,
	    dnahub_csr,
	    apphub_csr,
	}				= args_fn());
    });

    it("should create App", async function () {
	const bundle			= Bundle.createHapp( TEST_HAPP_CONFIG );
	const bundle_bytes		= bundle.toBytes();

	app1				= await apphub_csr.save_app( bundle_bytes );

	expect( app1.$addr		).to.be.a("EntryHash");
    });

    it("should create modified App"); // make sure integrity hash is different from app1

    it("should get my app entries", async function () {
	const apps			= await apphub_csr.get_app_entries_for_agent();

	log.normal("My App entries: %s", json.debug(apps) );

	expect( apps			).to.have.length( 1 );
    });

    linearSuite("Errors", function () {

	it("should fail to create App entry because of wrong invalid App token", async function () {
	    await expect_reject(async () => {
		const entry		= await apphub_csr.get_app_entry( app1.$addr );

		entry.app_token.integrity_hash = crypto.randomBytes( 32 );

		await apphub_csr.create_app_entry( entry );
	    }, "Invalid App Token" );

	    // Invalid roles token hash
	    await expect_reject(async () => {
		const entry		= await apphub_csr.get_app_entry( app1.$addr );

		entry.app_token.roles_token_hash = crypto.randomBytes( 32 );

		await apphub_csr.create_app_entry( entry );
	    }, "Invalid App Token" );

	    // Invalid roles token
	    await expect_reject(async () => {
		const entry		= await apphub_csr.get_app_entry( app1.$addr );

		entry.app_token.roles_token[0][0] = "invalid_role_name";

		await apphub_csr.create_app_entry( entry );
	    }, "Missing RoleToken for role" );
	});

	it("should fail to update App entry");

	it("should fail to delete App entry because author", async function () {
            this.timeout( 10_000 );

	    const bundle		= Bundle.createHapp( TEST_HAPP_CONFIG );
	    const app_bytes		= bundle.toBytes();

	    let app			= await apphub_csr.save_app( app_bytes );

	    const app_token		= installations.bobby.test.auth.token;
	    const bobby_client		= await client.app( app_token );
	    const bobby_apphub_csr	= bobby_client
		  .createCellInterface( "apphub", AppHubCell )
		  .zomes.apphub_csr.functions;

            await delay();

	    await expect_reject(async () => {
		await bobby_apphub_csr.delete_app( app.$id );
	    }, "Not authorized to delete entry created by author" );
	});

    });

}
