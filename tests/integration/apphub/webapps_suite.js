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

    before(async function () {
	({
	    app_client,
	    zomehub,
	    dnahub,
	    apphub,
	    zomehub_csr,
	    dnahub_csr,
	    apphub_csr,
	}				= args_fn());
    });

    it("should get my webapp entries", async function () {
	const webapps			= await apphub_csr.get_webapp_entries_for_agent();

	log.normal("My App entries: %s", json.debug(webapps) );

	expect( webapps			).to.have.length( 1 );
    });

    linearSuite("Errors", function () {
    });
}
