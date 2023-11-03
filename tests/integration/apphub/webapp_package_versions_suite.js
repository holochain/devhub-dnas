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
    let moved_version;

    before(async function () {
	({
	    app_client,
	    zomehub,
	    dnahub,
	    apphub,
	    zomehub_csr,
	    dnahub_csr,
	    apphub_csr,
	    webapp1_addr,
	}				= args_fn());

	pack1				= await apphub_csr.create_webapp_package_entry({
	    "title": faker.commerce.productName(),
	    "subtitle": faker.lorem.sentence(),
	    "description": faker.lorem.paragraphs( 2 ),
	    "icon": crypto.randomBytes( 1_000 ),
	    "source_code_uri": faker.internet.url(),
	});
    });

    it("should create WebApp Package Version", async function () {
	pack1_v1			= await apphub_csr.create_webapp_package_version({
	    "version": "0.1.0",
	    "for_package": pack1.$id,
	    "webapp": webapp1_addr,
	    "source_code_uri": faker.internet.url(),
	});

	log.normal("Create WebApp package version: %s", json.debug(pack1_v1) );

	expect( pack1_v1		).to.be.a("WebAppPackageVersion");
    });

    it("should update WebApp Package Version", async function () {
	const prev_version		= pack1_v1.toJSON();

	await pack1_v1.$update({
	    "changelog": faker.lorem.paragraphs( 5 ),
	});

	log.normal("Updated WebApp package version: %s", json.debug(pack1_v1) );

	expect( pack1_v1		).to.be.a("WebAppPackageVersion");
	expect( pack1_v1.changelog	).to.not.equal( prev_version.changelog );
    });

    it("should get Version's WebApp Package", async function () {
	const result			= await pack1_v1.$getWebAppPackage();

	expect( result			).to.deep.equal( pack1 );
    });

    async function create_version ( vtag ) {
	return await apphub_csr.create_webapp_package_version({
	    "version": vtag,
	    "for_package": pack1.$id,
	    "webapp": webapp1_addr,
	    "source_code_uri": faker.internet.url(),
	});
    }

    it("should get WebApp Package versions (sorted with semver)", async function () {
	moved_version			= await create_version("0.2.0");
	await create_version("0.1.0-beta-rc.0");
	await create_version("0.1.0-beta-rc.1");
	await create_version("0.1.0-beta-rc.2");
	await create_version("0.1.0-beta-rc.11");

	const versions			= await pack1.$versions();
	const version_names		= versions.map( packv => packv.version );

	log.normal("WebApp package versions: %s", json.debug(versions) );
	expect( versions[0]		).to.be.a("WebAppPackageVersion");
	expect( version_names		).to.deep.equal([
	    "0.2.0",
	    "0.1.0",
	    "0.1.0-beta-rc.11",
	    "0.1.0-beta-rc.2",
	    "0.1.0-beta-rc.1",
	    "0.1.0-beta-rc.0",
	]);
    });

    it("should get WebApp Package's version links", async function () {
	const version_links		= await apphub_csr.get_webapp_package_version_links( pack1.$id );

	log.normal("Version links: %s", json.debug(version_links) );
    });

    it("should update a WebApp Package Version's parent package", async function () {
	const pack			= await apphub_csr.create_webapp_package_entry({
	    "title": faker.commerce.productName(),
	    "subtitle": faker.lorem.sentence(),
	    "description": faker.lorem.paragraphs( 2 ),
	    "icon": crypto.randomBytes( 1_000 ),
	    "source_code_uri": faker.internet.url(),
	});

	const version			= await apphub_csr.move_webapp_package_version({
	    "version": "0.2.0",
	    "webapp_package_version_id": moved_version.$id,
	    "webapp_package_ids": {
		"from": pack1.$id,
		"to": pack.$id,
	    },
	});
	log.normal("Moved WebApp package version: %s", json.debug(version) );

	expect( version.for_package	).to.deep.equal( pack.$id );
    });

    linearSuite("Errors", function () {

	it("should fail to create a version that already exists", async function () {
	    await expect_reject( async () => {
		await create_version("0.1.0");
	    }, "already exists for package" );
	});

    });
}