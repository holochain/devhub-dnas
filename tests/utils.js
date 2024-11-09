
import crypto				from 'crypto';
import { expect }			from 'chai';


export async function expect_reject ( cb, error, message ) {
    let failed				= false;
    try {
	await cb();
    } catch (err) {
	failed				= true;
	expect( () => { throw err }	).to.throw( error, message );
    }
    expect( failed			).to.be.true;
}


export function linearSuite ( name, setup_fn, args_fn ) {
    describe( name, function () {
	beforeEach(function () {
	    let parent_suite		= this.currentTest.parent;
	    if ( parent_suite.tests.some(test => test.state === "failed") )
		this.skip();
	    if ( parent_suite.parent?.tests.some(test => test.state === "failed") )
		this.skip();
	});
	setup_fn.call( this, args_fn );
    });
}


export function dnaConfig () {
    return {
	"name": "fake-dna-1",
	"integrity": {
	    "origin_time": "2023-01-01T00:00:00Z",
	    "zomes": [{
		"name": "fake-zome-1",
		"bytes": new Uint8Array( Array( 1_000 ).fill( 1 ) ),
	    }],
	},
	"coordinator": {
	    "zomes": [{
		"name": "fake-zome-2",
		"bytes": new Uint8Array( Array( 1_000 ).fill( 1 ) ),
		"dependencies": [
		    "fake-zome-1",
		],
	    }],
	},
    };
}


export function happConfig ( roles ) {
    return {
	"name": "fake-happ-1",
	"description": "Empty testing files",
	"roles": roles,
    };
}


export function webhappConfig ( happ_manifest ) {
    return {
	"name": "fake-webhapp-1",
	"ui": {
	    "bytes": new Uint8Array( Array( 1_000 ).fill( 1 ) ),
	},
	happ_manifest,
    };
}

export function sha256 ( bytes ) {
    const hash				= crypto.createHash("sha256");

    hash.update( bytes );

    return hash.digest("hex");
}


export function delay ( n = 1 ) {
    return new Promise( f => setTimeout(f, n * 1_000) );
}


export default {
    expect_reject,
    linearSuite,
    dnaConfig,
    happConfig,
    webhappConfig,
    sha256,
    delay,
};
