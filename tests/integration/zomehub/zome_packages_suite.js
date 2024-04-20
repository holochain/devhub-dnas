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

    let bobby_client, bobby_zomehub_csr;

    let pack1;

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

	const app_token			= installations.bobby.test.auth.token;
	bobby_client			= await client.app( app_token );
	bobby_zomehub_csr	= bobby_client
	      .createCellInterface( "zomehub", ZomeHubCell )
	      .zomes.zomehub_csr.functions;
    });

    it("should create Zome Package entry", async function () {
	pack1				= await zomehub_csr.create_zome_package({
	    "name": faker.commerce.productName(),
	    "description": faker.lorem.paragraphs( 2 ),
	    "zome_type": "integrity",
	});

	log.normal("Create Zome package: %s", json.debug(pack1) );

	expect( pack1			).to.be.a("ZomePackage");
    });

    linearSuite("Errors", function () {

    });

}
