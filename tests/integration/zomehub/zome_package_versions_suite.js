import { Logger }			from '@whi/weblogger';
const log				= new Logger("zome-packages-suite", process.env.LOG_LEVEL );

import crypto				from 'crypto';
import { expect }			from 'chai';
import { faker }			from '@faker-js/faker';

import json				from '@whi/json';
import {
    ZomeHubCell,
}					from '@holochain/zomehub-zomelets';

import {
    expect_reject,
    linearSuite,
}					from '../../utils.js';


export default function ( args_fn ) {
    let installations;
    let client;
    let app_client;
    let zomehub;
    let zomehub_csr;
    let zome1_addr, zome1;

    let pack1;
    let pack1_v1;
    let pack1_name;

    before(async function () {
	({
	    installations,
	    client,
	    app_client,
	    zomehub,
	    zomehub_csr,
	    zome1_addr,
	    zome1,
	}				= args_fn());

        const title                     = faker.commerce.productName();
        pack1_name                      = title.toLowerCase(/\s/g, '-');

	pack1				= await zomehub_csr.create_zome_package({
	    "name": pack1_name,
	    title,
	    "description": faker.lorem.paragraphs( 2 ),
	    "zome_type": "integrity",
	});
    });

    it("should create Zome Package Version entry", async function () {
	pack1_v1			= await zomehub_csr.create_zome_package_version({
	    "version": "0.1.0",
	    "for_package": pack1.$id,
	    "zome_entry": zome1_addr,
	    "source_code_revision_uri": faker.internet.url(),
            "api_compatibility": {
                "build_with": {
                    "hdi_version": faker.system.semver(),
                    "hdk_version": faker.system.semver(),
                },
                "tested_with": faker.system.semver(),
            },
	});

	log.normal("Create Zome package version: %s", json.debug(pack1_v1) );

	expect( pack1_v1		).to.be.a("ZomePackageVersion");
    });

    it("should get Version's Zome Package", async function () {
	const result			= await pack1_v1.$getZomePackage();

	expect( result			).to.deep.equal( pack1 );
    });

    async function create_version ( vtag ) {
	return await zomehub_csr.create_zome_package_version({
	    "version": vtag,
	    "for_package": pack1.$id,
	    "zome_entry": zome1_addr,
	    "source_code_revision_uri": faker.internet.url(),
            "api_compatibility": {
                "build_with": {
                    "hdi_version": faker.system.semver(),
                    "hdk_version": null,
                },
                "tested_with": faker.system.semver(),
            },
	});
    }

    it("should get Zome Package versions (sorted with semver)", async function () {
	await create_version("0.2.0");
	await create_version("0.1.0-beta-rc.0");
	await create_version("0.1.0-beta-rc.1");
	await create_version("0.1.0-beta-rc.2");
	await create_version("0.1.0-beta-rc.11");

	const versions			= await pack1.$versions();
	const version_names		= versions.map( packv => packv.version );

	log.normal("Zome package versions: %s", json.debug(versions) );
	expect( versions[0]		).to.be.a("ZomePackageVersion");
	expect( version_names		).to.deep.equal([
	    "0.2.0",
	    "0.1.0",
	    "0.1.0-beta-rc.11",
	    "0.1.0-beta-rc.2",
	    "0.1.0-beta-rc.1",
	    "0.1.0-beta-rc.0",
	]);
    });

    it("should get Zome Package's version links", async function () {
	const version_links		= await zomehub_csr.get_zome_package_version_links( pack1.$id );

	log.normal("Version links: %s", json.debug(version_links) );
    });

    it("should download latest version", async function () {
	const latest_version		= await zomehub_csr.download_zome_package( pack1_name );

	log.normal("Latest package version: %s", json.debug(latest_version) );
    });

    linearSuite("Errors", function () {

    });

}
