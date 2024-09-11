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

    let pack1
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
    });

    it("should create Zome Package entry", async function () {
        const title                     = faker.commerce.productName();
        pack1_name                      = title.toLowerCase(/\s/g, '-');

	pack1				= await zomehub_csr.create_zome_package({
	    "name": pack1_name,
	    title,
	    "description": faker.lorem.paragraphs( 2 ),
	    "zome_type": "integrity",
	});

	log.normal("Create Zome package: %s", json.debug(pack1) );

	expect( pack1			).to.be.a("ZomePackage");
    });

    it("should get Zome Package entry", async function () {
	const zome_package		= await zomehub_csr.get_zome_package_entry( pack1.$id );

	log.normal("Get Zome package: %s", json.debug(zome_package) );

	expect( zome_package		).to.be.a("ZomePackage");
    });

    it("should get my Zome Packages", async function () {
	const zome_packages		= await zomehub_csr.get_zome_packages_for_agent();
	const package_list		= Object.values( zome_packages );

	log.normal("Get Zome packages: %s", json.debug(zome_packages) );

	expect( package_list		).to.have.length( 1 );
    });

    it("should get Zome Package by name", async function () {
	const zome_package              = await zomehub_csr.get_zome_package_by_name( pack1_name );

	log.normal("Get Zome package: %s", json.debug(zome_package) );
    });

    linearSuite("Errors", function () {

    });

}
