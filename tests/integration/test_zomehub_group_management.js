import { Logger }                       from '@whi/weblogger';
const log                               = new Logger("test-zomehub-basic", process.env.LOG_LEVEL );

// import why                           from 'why-is-node-running';

import fs				from 'node:fs/promises';
import path                             from 'path';
import crypto                           from 'crypto';
import { faker }                        from '@faker-js/faker';

import { expect }                       from 'chai';

import json                             from '@whi/json';
import { Holochain }                    from '@spartan-hc/holochain-backdrop';

import {
    ZomeHubCell,
    ZOME_TYPES,
}                                       from '@holochain/zomehub-zomelets';
import {
    AppInterfaceClient,
}                                       from '@spartan-hc/app-interface-client';

import {
    expect_reject,
    linearSuite,
    delay,
}                                       from '../utils.js';


const __dirname                         = path.dirname( new URL(import.meta.url).pathname );
const ZOMEHUB_DNA_PATH                  = path.join( __dirname, "../../dnas/zomehub.dna" );
const ZOMEHUB_WASM_PATH                 = path.join( __dirname, "../../zomes/zomehub.wasm" );

const ZOMEHUB_DNA_NAME                  = "zomehub";
const MAIN_ZOME                         = "zomehub_csr";
const MERE_ZOME                         = "mere_memory_api";
const COOP_ZOME                         = "coop_content_csr";

let client, installations;

describe("ZomeHub", function () {
    const holochain                     = new Holochain({
        "timeout": 120_000,
        "default_stdout_loggers": log.level_rank > 3,
    });

    before(async function () {
        this.timeout( 120_000 );

        installations                   = await holochain.install([
            "alice",
            "bobby",
        ], [
            {
                "app_name": "test",
                "bundle": {
                    [ZOMEHUB_DNA_NAME]: ZOMEHUB_DNA_PATH,
                },
            },
        ]);

        const app_port                  = await holochain.ensureAppPort();

        client                          = new AppInterfaceClient( app_port, {
            "logging": process.env.LOG_LEVEL || "fatal",
        });
    });

    setup_tests();

    linearSuite("Phase 1 - Zome Package", phase1_tests );
    linearSuite("Phase 2 - Zome Package Version", phase2_tests );

    after(async function () {
        await client.close();
        await holochain.destroy();
    });
});


// Script
//
//   - Alice creates a zome package
//   - Alice adds Bobby to the collaborators list
//   - Bobby makes an update to the package
//   - Alice sees the update
//   - Bobby creates a package version
//   - Alice removes bobby from group
//   - Alice updates package version
//   - Bobby cannot update package version
//

let alice_client;
let bobby_client;

let alice_zomehub;
let bobby_zomehub;

let alice_mere_memory;
let bobby_mere_memory;

let alice_coop_content;
let bobby_coop_content;

let group1_addr_with_bobby;

function setup_tests () {

    before(async function () {
        this.timeout( 30_000 );

        {
            const app_token             = installations.alice.test.auth.token;
            alice_client                = await client.app( app_token );

            const {
                zomehub,
            }                           = alice_client.createInterface({
                [ZOMEHUB_DNA_NAME]: ZomeHubCell,
            });

            alice_zomehub               = zomehub.zomes.zomehub_csr.functions;
            alice_mere_memory           = zomehub.zomes.mere_memory_api.functions;
            alice_coop_content          = zomehub.zomes.coop_content_csr.functions;

	    const hash_path = await alice_mere_memory.make_hash_path( "hash.path" );
            log.normal("Alice mere_memory", { hash_path });
        }

        {
            const app_token             = installations.bobby.test.auth.token;
            bobby_client                = await client.app( app_token );

            const {
                zomehub,
            }                           = bobby_client.createInterface({
                [ZOMEHUB_DNA_NAME]: ZomeHubCell,
            });

            bobby_zomehub               = zomehub.zomes.zomehub_csr.functions;
            bobby_mere_memory           = zomehub.zomes.mere_memory_api.functions;
            bobby_coop_content          = zomehub.zomes.coop_content_csr.functions;

	    const hash_path = await alice_mere_memory.make_hash_path( "hash.path" );
            log.normal("Bobby mere_memory", { hash_path });
        }

        await alice_zomehub.whoami();
        await bobby_zomehub.whoami();

        const agents                    = await alice_zomehub.list_all_agents();
        expect( agents                  ).to.have.length( 2 );
    });

}


let group1;
let pack1;

function phase1_tests () {
    beforeEach(async function () {
	const hash_path = await alice_mere_memory.make_hash_path( "hash.path" );
        log.normal("Alice mere_memory", { hash_path });
    });

    it("(alice) should create organization", async function () {
        group1                          = await alice_coop_content.create_group({
            "admins":           [ alice_client.agent_id ],
            "members":          [],

            "published_at":     Date.now(),
            "last_updated":     Date.now(),
            "metadata":         {},
        });

        log.normal("New organization group: %s", json.debug(group1) );

        await alice_zomehub.create_named_group_link([ "abc_devs", group1.$id ]);

        const group_links               = await alice_zomehub.get_my_group_links();
        log.normal("My group links: %s", json.debug(group_links) );
    });

    it("(alice) should create zome package", async function () {
        const title                     = faker.commerce.productName();

        pack1                           = await alice_zomehub.create_zome_package({
            "name":             "@abc_devs/" + title.toLowerCase(/\s/g, '-'),
            title,
            "description":      faker.lorem.paragraphs( 2 ),
            "zome_type":        "integrity",
            "maintainer": {
                "type":         "group",
                "content":      [ group1.$id, group1.$action ],
            },
        });

        log.normal("New zome package: %s", json.debug(pack1) );
    });

    it("(bobby) should get all orgs", async function () {
        const org_links                 = await bobby_zomehub.get_all_org_group_links();

        expect( org_links               ).to.have.length( 1 );
    });

    it("(bobby) should fail to update zome package", async function () {
        await expect_reject(async () => {
            await bobby_zomehub.update_zome_package({
                "base": pack1.$action,
                "properties": {
                    "title": faker.commerce.productName(),
                },
            });
        }, "not authorized in group" );
    });

    it("(alice) should add bobby to organization", async function () {
        group1                          = await alice_coop_content.update_group({
            "base": group1.$action,
            "entry": Object.assign( {}, group1, {
                "members":          [ bobby_client.agent_id ],
                "last_updated":     Date.now(),
            }),
        });
        group1_addr_with_bobby          = group1.$action;

        log.normal("Updated organization group: %s", json.debug(group1) );
    });

    it("(bobby) should update zome package", async function () {
        await bobby_zomehub.accept_invitation_to_group( "abc_devs" );

        {
            const group_links           = await bobby_zomehub.get_my_group_links();
            expect( group_links         ).to.have.length( 1 );
        }

        pack1                           = await bobby_zomehub.update_zome_package({
            "base": pack1.$action,
            "properties": {
                "title":            faker.commerce.productName(),
                "description":      faker.lorem.paragraphs( 2 ),
            },
        });

        log.normal("Updated zome package: %s", json.debug(pack1) );

        {
            const removed               = await bobby_zomehub.remove_named_group_link( "abc_devs" );
            console.log("Removed org links: %s", json.debug(removed) );

            const group_links           = await bobby_zomehub.get_my_group_links();
            expect( group_links         ).to.have.length( 0 );
        }
    });

    it("(bobby) should update zome package", async function () {
        const zome_package              = await bobby_zomehub.get_zome_package( pack1.$id );
        log.normal("Fetched zome package (%s): %s", pack1.$id, json.debug(zome_package) );

        expect( zome_package.$action    ).to.deep.equal( pack1.$action );
    });

    it("(alice) should get latest package", async function () {
        const latest                    = await alice_coop_content.get_group_content_latest({
            "group_id": group1.$id,
            "content_id": pack1.$id,
        });

        log.normal("Latest group content link: %s", json.debug(latest) );

        expect( latest                  ).to.deep.equal( pack1.$action );
    });

    it("(alice) should get zome packages for group", async function () {
        const zome_packages             = await alice_zomehub.get_zome_packages_for_group( group1.$id );

        log.normal("Group zome packages: %s", json.debug(zome_packages) );
        expect( zome_packages           ).to.have.length( 1 );
    });

    it("(alice) should get zome packages for org", async function () {
        const zome_packages             = await alice_zomehub.get_zome_packages_for_org( "@abc_devs" );

        log.normal("Org zome packages: %s", json.debug(zome_packages) );
        expect( zome_packages           ).to.have.length( 1 );
    });

    it("(alice) should get all zome package links", async function () {
        const zome_package_links        = await alice_zomehub.get_all_zome_package_links();

        log.normal("All zome package links: %s", json.debug(zome_package_links) );
        expect( zome_package_links      ).to.have.length( 1 );
    });
}

let zomehub_wasm_bytes                  = await fs.readFile( ZOMEHUB_WASM_PATH );
let zome1;
let pack1_v1;

function phase2_tests () {

    it("(bobby) should create zome package version", async function () {
        this.timeout( 120_000 );

	zome1				= await bobby_zomehub.save_integrity( zomehub_wasm_bytes );
        log.normal("New zome: %s", json.debug(zome1) );

	pack1_v1			= await bobby_zomehub.create_zome_package_version({
	    "version": "0.1.0",
	    "for_package": pack1.$id,
	    "zome_entry": zome1.$addr,
	    "source_code_revision_uri": faker.internet.url(),
            "api_compatibility": {
                "build_with": {
                    "hdi_version": faker.system.semver(),
                    "hdk_version": faker.system.semver(),
                },
                "tested_with": faker.system.semver(),
            },
	});

        log.normal("New zome package version: %s", json.debug(pack1_v1) );
    });

    it("(alice) should remove bobby from organization", async function () {
        group1                          = await alice_coop_content.update_group({
            "base": group1.$action,
            "entry": Object.assign( {}, group1, {
                "members":          [],
                "last_updated":     Date.now(),
            }),
        });

        log.normal("Updated organization group: %s", json.debug(group1) );
    });

    it("(bobby) should fail to create zome package version", async function () {
        await expect_reject(async () => {
	    await bobby_zomehub.update_zome_package_version({
                "base": pack1_v1.$action,
                "properties": {
                    "changelog": faker.lorem.paragraphs( 5 ),
	            "source_code_revision_uri": faker.internet.url(),
                    "api_compatibility": {
                        "build_with": {
                            "hdi_version": faker.system.semver(),
                            "hdk_version": faker.system.semver(),
                        },
                        "tested_with": faker.system.semver(),
                    },
                },
	    });
        }, "not authorized in group" );
    });

    it("should not get bobby's forced zome package version update", async function () {
	const bobbys_update             = await bobby_zomehub.update_zome_package_version({
            "base": pack1_v1.$action,
            "properties": {
                "maintainer": {
                    "type": "group",
                    "content": [ group1.$id, group1_addr_with_bobby ],
                },
                "changelog": faker.lorem.paragraphs( 5 ),
            },
	});
        log.normal("Forced update (%s): %s", bobbys_update.$action, json.debug(bobbys_update) );

        const latest                    = await alice_coop_content.get_group_content_latest({
            "group_id": group1.$id,
            "content_id": pack1_v1.$id,
        });

        expect( bobbys_update.$action   ).to.not.deep.equal( latest );

        const zome_pack_vers            = await alice_zomehub.get_zome_package_version( pack1_v1.$id );
        log.normal("Fetched zome package version (%s): %s", pack1_v1.$id, json.debug(zome_pack_vers) );

        expect( zome_pack_vers.$action  ).to.not.deep.equal( bobbys_update.$action );
        expect( zome_pack_vers.$action  ).to.deep.equal( latest );
    });

    it("should not get bobby's forced zome package update", async function () {
        const bobbys_update             = await bobby_zomehub.update_zome_package({
            "base": pack1.$action,
            "properties": {
                "title":            faker.commerce.productName(),
                "description":      faker.lorem.paragraphs( 2 ),
                "maintainer": {
                    "type": "group",
                    "content": [ group1.$id, group1_addr_with_bobby ],
                },
            },
        });
        log.normal("Forced update (%s): %s", bobbys_update.$action, json.debug(bobbys_update) );

        const latest                    = await alice_coop_content.get_group_content_latest({
            "group_id": group1.$id,
            "content_id": pack1.$id,
        });

        expect( bobbys_update.$action   ).to.not.deep.equal( latest );

        const zome_package              = await alice_zomehub.get_zome_package( pack1.$id );
        log.normal("Fetched zome package (%s): %s", pack1.$id, json.debug(zome_package) );

        expect( zome_package.$action    ).to.not.deep.equal( bobbys_update.$action );
        expect( zome_package.$action    ).to.deep.equal( latest );
    });

}
