import { Logger }                       from '@whi/weblogger';
const log                               = new Logger("test-zomehub-basic", process.env.LOG_LEVEL );

// import why                           from 'why-is-node-running';

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

    linearSuite("Basic", basic_tests );

    after(async () => {
        await holochain.destroy();
    });
});


// Script
//
//   - Alice creates a zome package
//   - Alice adds Bobby to the collaborators list
//   - Bobby makes an update to the package
//   - Alice sees the update
//

let alice_client;
let bobby_client;

let alice_zomehub;
let bobby_zomehub;

let alice_mere_memory;
let bobby_mere_memory;

let alice_coop_content;
let bobby_coop_content;

let group1;
let pack1;

function basic_tests () {

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
        }

        await alice_zomehub.whoami();
        await bobby_zomehub.whoami();
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
    });

    it("(alice) should create zome package", async function () {
        const title                     = faker.commerce.productName();

        pack1                           = await alice_zomehub.create_zome_package({
            "name":             title.toLowerCase(/\s/g, '-'),
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

        log.normal("Updated organization group: %s", json.debug(group1) );
    });

    it("(bobby) should update zome package", async function () {
        pack1                           = await bobby_zomehub.update_zome_package({
            "base": pack1.$action,
            "properties": {
                "title":            faker.commerce.productName(),
                "description":      faker.lorem.paragraphs( 2 ),
                "maintainer": {
                    "type":         "group",
                    "content":      [ group1.$id, group1.$action ],
                },
            },
        });

        log.normal("Updated zome package: %s", json.debug(pack1) );
    });

    it("(alice) should get latest package", async function () {
        const latest                    = await alice_coop_content.get_group_content_latest({
            "group_id": group1.$id,
            "content_id": pack1.$id,
        });

        log.normal("Latest group content link: %s", json.debug(latest) );
    });

    after(async function () {
        await client.close();
    });
}
