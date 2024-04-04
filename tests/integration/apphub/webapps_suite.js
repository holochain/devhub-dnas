import { Logger }			from '@whi/weblogger';
const log				= new Logger("webapps-suite", process.env.LOG_LEVEL );

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
    webhappConfig,
}					from '../../utils.js';


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


export default function ( args_fn ) {
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
	({
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
	const bundle			= Bundle.createWebhapp( TEST_WEBHAPP_CONFIG );
	const bundle_bytes		= bundle.toBytes();

	webapp1				= await apphub_csr.save_webapp( bundle_bytes );

	expect( webapp1.$addr		).to.be.a("EntryHash");
    });

    it("should get my webapp entries", async function () {
	const webapps			= await apphub_csr.get_webapp_entries_for_agent();

	log.normal("My App entries: %s", json.debug(webapps) );

	expect( webapps			).to.have.length( 1 );
    });

    linearSuite("Errors", function () {

	it("should fail to create WebApp entry because of wrong invalid WebApp token", async function () {
	    await expect_reject(async () => {
		const entry		= await apphub_csr.get_webapp_entry( webapp1.$addr );

		entry.webapp_token.app_token.integrity_hash = crypto.randomBytes( 32 );

		await apphub_csr.create_webapp_entry( entry );
	    }, "Invalid WebApp Token" );
	});

	it("should fail to update WebApp entry");

	it("should fail to delete WebApp entry because author", async function () {
	    const bundle		= Bundle.createWebhapp( TEST_WEBHAPP_CONFIG );
	    const webapp_bytes		= bundle.toBytes();

	    let webapp			= await apphub_csr.save_webapp( webapp_bytes );

	    const bobby_client		= await client.app( "test-bobby" );
	    const bobby_apphub_csr	= bobby_client
		  .createCellInterface( "apphub", AppHubCell )
		  .zomes.apphub_csr.functions;

	    await expect_reject(async () => {
		await bobby_apphub_csr.delete_webapp( webapp.$id );
	    }, "Not authorized to delete entry created by author" );
	});

    });
}
