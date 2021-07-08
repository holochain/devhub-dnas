
const HoloHashLib			= require('@whi/holo-hash');
const EntityArchitectLib		= require('@whi/entity-architect');

const { Architecture,
	EntityType,
	EntryHash,
	AgentPubKey }			= EntityArchitectLib;



//
// hApps Entity Types
//
const Happ				= new EntityType("happ");

Happ.model("info", function ( content ) {
    content.published_at	= new Date( content.published_at );
    content.last_updated	= new Date( content.last_updated );
    content.designer		= new AgentPubKey( content.designer );

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
    content.authorr		= new AgentPubKey( content.author );
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
    Happ,
    Profile, Dna, DnaVersion, DnaChunk,
    File, FileChunk,
]);


module.exports = {
    Schema,

    Happ,
    Profile,
    Dna,
    DnaVersion,
    DnaChunk,
    File,
    FileChunk,

    "EntityArchitect": EntityArchitectLib,
    "HoloHashes": HoloHashLib,
};
