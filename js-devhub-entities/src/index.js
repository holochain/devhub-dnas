
const { AppWebsocket }			= require('@holochain/conductor-api');
const { Translator }			= require('@whi/essence');
const HoloHashLib			= require('@whi/holo-hash');
const EntityArchitectLib		= require('@whi/entity-architect');

const { Architecture,
	EntityType,
	EntryHash,
	AgentPubKey }			= EntityArchitectLib;


let debug				= false;
function log ( msg, ...args ) {
    let datetime			= (new Date()).toISOString();
    console.log(`${datetime} [ src/index. ]  INFO: ${msg}`, ...args );
}



//
// hApps Entity Types
//
const Happ				= new EntityType("happ");

Happ.model("info", function ( content ) {
    content.published_at	= new Date( content.published_at );
    content.last_updated	= new Date( content.last_updated );
    content.designer		= new AgentPubKey( content.designer );

    if ( content.gui ) {
	content.gui.asset_group_id	= new EntryHash( content.gui.asset_group_id );
    }

    return content;
});

Happ.model("summary", function ( content ) {
    content.published_at	= new Date( content.published_at );
    content.last_updated	= new Date( content.last_updated );
    content.designer		= new AgentPubKey( content.designer );

    return content;
});

const HappRelease			= new EntityType("happ_release");

HappRelease.model("info", function ( content ) {
    content.published_at	= new Date( content.published_at );
    content.last_updated	= new Date( content.last_updated );
    content.for_happ		= Schema.deconstruct( "entity", content.for_happ );

    for (let k in content.resources ) {
	content.resources[k]	= new EntryHash( content.resources[k] );
    }

    return content;
});

HappRelease.model("summary", function ( content ) {
    content.published_at	= new Date( content.published_at );
    content.last_updated	= new Date( content.last_updated );
    content.for_happ		= new EntryHash( content.for_happ );

    return content;
});


//
// DNA Repository Entity Types
//
const Profile				= new EntityType("profile");

Profile.model("info");


const Dna				= new EntityType("dna");

Dna.model("info", function ( content ) {
    content.published_at	= new Date( content.published_at );
    content.last_updated	= new Date( content.last_updated );
    content.developer.pubkey	= new AgentPubKey( content.developer.pubkey );

    return content;
});
Dna.model("summary", function ( content ) {
    content.published_at	= new Date( content.published_at );
    content.last_updated	= new Date( content.last_updated );
    content.developer		= new AgentPubKey( content.developer );

    return content;
});


const DnaVersion			= new EntityType("dna_version");

DnaVersion.model("package", function ( content ) {
    content.for_dna		= Schema.deconstruct( "entity", content.for_dna );
    content.published_at	= new Date( content.published_at );
    content.last_updated	= new Date( content.last_updated );
    content.bytes		= new Uint8Array(content.bytes);

    content.contributors.forEach( ([email, pubkey], i) => {
	content.contributors[i]		= {
	    email,
	    "agent": pubkey === null ? null : new AgentPubKey(pubkey),
	};
    });

    return content;
});
DnaVersion.model("info", function ( content ) {
    content.for_dna		= Schema.deconstruct( "entity", content.for_dna );
    content.published_at	= new Date( content.published_at );
    content.last_updated	= new Date( content.last_updated );

    content.contributors.forEach( ([email, pubkey], i) => {
	content.contributors[i]		= {
	    email,
	    "agent": pubkey === null ? null : new AgentPubKey(pubkey),
	};
    });

    content.chunk_addresses.forEach( (addr, i) => {
	content.chunk_addresses[i]	= new EntryHash(addr);
    });

    return content;
});
DnaVersion.model("summary", function ( content ) {
    content.published_at	= new Date( content.published_at );
    content.last_updated	= new Date( content.last_updated );

    return content;
});


const DnaChunk				= new EntityType("dna_chunk");

DnaChunk.model("info");


//
// Web Asset Entity Types
//
const File				= new EntityType("file");

File.model("info", function ( content ) {
    content.author		= new AgentPubKey( content.author );
    content.published_at	= new Date( content.published_at );

    content.chunk_addresses.forEach( (addr, i) => {
	content.chunk_addresses[i]	= new EntryHash(addr);
    });

    return content;
});

const FileChunk				= new EntityType("file_chunk");

FileChunk.model("info");


//
// Grouping Entity Definitions
//
const Schema				= new Architecture([
    Happ, HappRelease,
    Profile, Dna, DnaVersion, DnaChunk,
    File, FileChunk,
]);


const Interpreter			= new Translator(["AppError", "UtilsError", "DNAError", "UserError", "WasmError"], {
    "rm_stack_lines": 2,
});

class Client {
    constructor ( port, dna_hash, agent_pubkey ) {
	this.port			= port;
	this.dna_hash			= dna_hash;
	this.agent_pubkey		= agent_pubkey;
	this.cell_id			= [ this.dna_hash, this.agent_pubkey ];
    }

    async connect () {
	this._client			= await AppWebsocket.connect( "ws://localhost:" + this.port );
    }

    async destroy () {
	this._client.socket.terminate();
    }

    async call ( zome_name, fn_name, args = null ) {
	debug && log("Calling conductor: %s->%s({ %s })", zome_name, fn_name, Object.keys(args || {}).join(", ") );

	let response;
	try {
	    response			= await this._client.callZome({
		"cell_id":	this.cell_id,
		"zome_name":	zome_name,
		"fn_name":	fn_name,
		"payload":	args,
		"provenance":	this.agent_pubkey, // AgentPubKey
	    });
	} catch ( err ) {
	    debug && log("Conductor returned error: %s", err );
	    if ( err instanceof Error )
		console.error( err );
	    throw err;
	}

	let pack;
	try {
	    pack			= Interpreter.parse( response );
	} catch ( err ) {
	    debug && log("Failed to interpret Essence package: %s", String(err) );
	    console.error( err.stack );
	    throw err;
	}

	let payload			= pack.value();

	if ( payload instanceof Error ) {
	    debug && log("Throwing error package: %s::%s( %s )", payload.kind, payload.name, payload.message );
	    throw payload;
	}

	let composition			= pack.metadata('composition');

	try {
	    return Schema.deconstruct( composition, payload );
	} catch ( err ) {
	    debug && log("Failed to deconstruct payload: %s", String(err) );
	    console.error( err.stack );
	    throw err;
	}
    };
}


module.exports = {
    Client,
    Schema,

    Happ,
    HappRelease,
    Profile,
    Dna,
    DnaVersion,
    DnaChunk,
    File,
    FileChunk,

    "EntityArchitect": EntityArchitectLib,
    "HoloHashes": HoloHashLib,

    logging () {
	debug				= true;
    },
};
