
const fs				= require('fs');
const json				= require('@whi/json');
const msgpack				= require('@msgpack/msgpack');
const { ungzip }			= require('node-gzip');

function print( msg, ...args ) {
    console.log(`\x1b[37m${msg}\x1b[0m`, ...args );
}

(async function main ( dna_file ) {
    print("Inspecting DNA package: %s", dna_file );
    if ( dna_file === undefined )
	throw new TypeError(`First argument must be a path to the DNA file; not 'undefined'`);

    let dna_pack			= fs.readFileSync( dna_file );
    let msgpack_bytes			= await ungzip( dna_pack );
    let bundle				= msgpack.decode( msgpack_bytes );

    for (let path in bundle.resources) {
	bundle.resources[path] = Buffer.from( bundle.resources[path] );
    }

    console.log( json.debug(bundle) );
})( ...process.argv.slice(2) );
