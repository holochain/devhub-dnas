const path				= require('path');
const log				= require('@whi/stdlog')(path.basename( __filename ), {
    level: process.env.LOG_LEVEL || 'fatal',
});

const expect				= require('chai').expect;

const { HoloHash }			= require('@whi/holo-hash');
const { Schema }			= require('../../src/index.js');


const AGENT				= (new HoloHash("uhCAkocJKdTlSkQFVmjPW_lA_A5kusNOORPrFYJqT8134Pag45Vjf")).bytes();
const ID				= (new HoloHash("uhCEkEvFsj08QdtgiUDBlEhwlcW5lsfqD4vKRcaGIirSBx0Wl7MVf")).bytes();
const HEADER				= (new HoloHash("uhCkkn_kIobHe9Zt4feh751we8mDGyJuBXR50X5LBqtcSuGLalIBa")).bytes();
const ADDRESS				= (new HoloHash("uhCEkU7zcM5NFGXIljSHjJS3mk62FfVRpniZQlg6f92zWHkOZpb2z")).bytes();

let dna_entity_payload = {
    "id": ID,
    "header": HEADER,
    "address": ADDRESS,
    "type": {
	"name": "dna",
	"model": "info",
    },
    "content": {
	"name": "Game Turns (new)",
	"description": "A tool for turn-based games to track the order of player actions",
	"published_at": 1624661323383,
	"last_updated": 1624661325451,
	"developer": {
	    "pubkey": AGENT,
	},
	"icon": null,
	"collaborators": null,
	"deprecation": null
    }
};

let dna_version_entity_payload = {
    "id": ID,
    "header": HEADER,
    "address": ADDRESS,
    "type": {
	"name": "dna_version",
	"model": "summary",
    },
    "content": {
	"for_dna": ID,
	"version": 1,
	"published_at": 1624661323383,
	"last_updated": 1624661325451,
	"wasm_hash": "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9",
	"zomes": [{
	    "name": "zome name",
	    "zome": ADDRESS,
	    "version": ADDRESS,
	    "resource": ADDRESS,
	    "resource_hash": "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9",
	}],
    },
};


function basic_tests () {
    it("should deconstruct 'dna' entity", async () => {
	let data			= Schema.deconstruct( "entity", dna_entity_payload );

	expect( data.developer.pubkey		).to.be.instanceof( HoloHash );
	expect( String(data.developer.pubkey)	).to.equal("uhCAkocJKdTlSkQFVmjPW_lA_A5kusNOORPrFYJqT8134Pag45Vjf");

	expect( data.published_at		).to.be.instanceof( Date );
	expect( data.published_at.toISOString()	).to.equal("2021-06-25T22:48:43.383Z");

	expect( data.last_updated		).to.be.instanceof( Date );
	expect( data.last_updated.toISOString()	).to.equal("2021-06-25T22:48:45.451Z");
    });

    it("should deconstruct 'dna_version' entity", async () => {
	let data			= Schema.deconstruct( "entity", dna_version_entity_payload );

	expect( data.published_at		).to.be.instanceof( Date );
	expect( data.published_at.toISOString()	).to.equal("2021-06-25T22:48:43.383Z");

	expect( data.last_updated		).to.be.instanceof( Date );
	expect( data.last_updated.toISOString()	).to.equal("2021-06-25T22:48:45.451Z");
    });
}

describe("Entities", () => {

    describe("Basic", basic_tests );

});
