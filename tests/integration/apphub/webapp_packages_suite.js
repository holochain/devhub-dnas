import { Logger }			from '@whi/weblogger';
const log				= new Logger("webapp-packages-suite", process.env.LOG_LEVEL );

import crypto				from 'crypto';
import { expect }			from 'chai';
import { faker }			from '@faker-js/faker';

import json				from '@whi/json';
import {
    AppHubCell,
}					from '@holochain/apphub-zomelets';

import {
    expect_reject,
    linearSuite,
}					from '../../utils.js';


export default function ( args_fn ) {
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
    let bobby_client, bobby_apphub_csr;

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
	    webapp1_addr,
	}				= args_fn());

	bobby_client			= await client.app( "test-bobby" );
	bobby_apphub_csr		= bobby_client
	      .createCellInterface( "apphub", AppHubCell )
	      .zomes.apphub_csr.functions;
    });

    it("should create WebApp Package entry", async function () {
	pack1				= await apphub_csr.create_webapp_package({
	    "title": faker.commerce.productName(),
	    "subtitle": faker.lorem.sentence(),
	    "description": faker.lorem.paragraphs( 2 ),
	    "icon": crypto.randomBytes( 1_000 ),
	    "source_code_uri": faker.internet.url(),
	});

	log.normal("Create WebApp package: %s", json.debug(pack1) );

	expect( pack1			).to.be.a("WebAppPackage");
    });

    it("should get all WebApp Packages", async function () {
	const result			= await apphub_csr.get_all_webapp_packages();

	expect( result			).to.have.length( 1 );
    });

    it("should update WebApp Package", async function () {
	const prev_pack			= pack1.toJSON();

	await pack1.$update({
	    "description": faker.lorem.paragraphs( 2 ),
	    "source_code_uri": faker.internet.url(),
	});

	log.normal("Updated WebApp package: %s", json.debug(pack1) );

	expect( pack1.description	).to.not.equal( prev_pack.description );
	expect( pack1.source_code_uri	).to.not.equal( prev_pack.source_code_uri );
    });

    it("should get WebApp Package using EntryHash", async function () {
	const pack1b			= await apphub_csr.get_webapp_package_entry( pack1.$addr );

	expect( pack1b			).to.deep.equal( pack1 );
    });

    it("should deprecate WebApp Package", async function () {
	await pack1.$deprecate("No longer mainained");

	log.normal("Updated WebApp package: %s", json.debug(pack1) );

	expect( pack1			).to.be.a("WebAppPackage");

	const all_apps			= await apphub_csr.get_all_webapp_packages();

	expect( all_apps		).to.have.length( 0 );
    });

    linearSuite("Errors", function () {

	it("should fail to create WebApp Package entry because maintainer doesn't match create author", async function () {
	    await expect_reject(async () => {
		const entry		= await apphub_csr.get_webapp_package_entry( pack1.$id );

		entry.maintainer.content = bobby_client.agent_id;

		await apphub_csr.create_webapp_package_entry( entry );
	    }, "Invalid maintainer" );
	});

	it("should fail to update WebApp Package because invalid maintainer", async function () {
	    await expect_reject(async () => {
		await bobby_apphub_csr.update_webapp_package({
		    "base": pack1.$action,
		    "properties": {
			"description": faker.lorem.paragraphs( 2 ),
		    },
		});
	    }, "Not authorized to update entry" );
	});

	it("should fail to update deprecated WebApp Package", async function () {
	    await expect_reject(async () => {
		await pack1.$update({
		    "description": faker.lorem.paragraphs( 2 ),
		});
	    }, "Cannot update deprecated entity" );
	});

	it("should fail to delete WebApp entry because author", async function () {
	    const pack			= await apphub_csr.create_webapp_package({
		"title": faker.commerce.productName(),
		"subtitle": faker.lorem.sentence(),
		"description": faker.lorem.paragraphs( 2 ),
		"icon": crypto.randomBytes( 1_000 ),
		"source_code_uri": faker.internet.url(),
	    });

	    await expect_reject(async () => {
		await bobby_apphub_csr.delete_webapp_package( pack.$id );
	    }, "Not authorized to delete entry created by author" );
	});

    });

}
