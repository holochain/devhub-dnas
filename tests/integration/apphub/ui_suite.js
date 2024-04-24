import { Logger }			from '@whi/weblogger';
const log				= new Logger("ui-suite", process.env.LOG_LEVEL );

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


const UI_BYTES				= new Uint8Array( Array( 1_000 ).fill( 1 ) );


export default function ( args_fn ) {
    let client;
    let app_client;
    let zomehub;
    let zomehub_csr;
    let dnahub;
    let dnahub_csr;
    let apphub;
    let apphub_csr;
    let ui1;

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

    it("should create UI", async function () {
	ui1				= await apphub_csr.save_ui( UI_BYTES );

	expect( ui1.$addr		).to.be.a("EntryHash");
    });

    it("should get my UI entries", async function () {
	const uis			= await apphub_csr.get_ui_entries_for_agent();

	log.normal("My UI entries: %s", json.debug(uis) );

	expect( uis			).to.have.length( 1 );
    });

    it("should get UI package", async function () {
	const ui_pack			= await apphub_csr.get_ui_package( ui1.$addr );
	log.normal("%s", json.debug(ui_pack) );

	expect( ui_pack		).to.have.any.keys( "bytes" );
    });

    linearSuite("Errors", function () {

	it("should fail to create wasm entry because of wrong file size", async function () {
	    await expect_reject(async () => {
		await apphub_csr.create_ui_entry({
		    "mere_memory_addr": ui1.mere_memory_addr,
		    "file_size": 0,
		});
	    }, "file size does not match memory address" );
	});

	it("should fail to update UI entry");

	it("should fail to delete UI entry because author", async function () {
	    let ui			= await apphub_csr.save_ui( UI_BYTES );

	    const bobby_client		= await client.app( "test-bobby" );
	    const bobby_apphub_csr	= bobby_client
		  .createCellInterface( "apphub", AppHubCell )
		  .zomes.apphub_csr.functions;

	    await expect_reject(async () => {
		await bobby_apphub_csr.delete_ui( ui.$id );
	    }, "Not authorized to delete entry created by author" );
	});

    });

}
