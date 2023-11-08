import { Logger }			from '@whi/weblogger';
const log				= new Logger("test-webapp-upload", process.env.LOG_LEVEL );

import crypto				from 'crypto';
import { expect }			from 'chai';
import { faker }			from '@faker-js/faker';

import json				from '@whi/json';

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
    });

    it("should create WebApp Package entry", async function () {
	pack1				= await apphub_csr.create_webapp_package_entry({
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

	expect( pack1b			).to.deep.equal( pack1.toJSON() );
    });

    it("should deprecate WebApp Package", async function () {
	await pack1.$deprecate("No longer mainained");

	log.normal("Updated WebApp package: %s", json.debug(pack1) );

	expect( pack1			).to.be.a("WebAppPackage");

	const all_apps			= await apphub_csr.get_all_webapp_packages();

	expect( all_apps		).to.have.length( 0 );
    });

    linearSuite("Errors", function () {
    });
}
