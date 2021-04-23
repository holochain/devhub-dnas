
function hex (n, pad = 8) {
    return ( "0".repeat(pad) + n.toString(16) ).substr(-pad);
}

function default_replacer_truncate_buffers (key, value) {
    if (typeof value === 'object' && value !== null && value.type === "Buffer") {
	let hexstr = value.data.slice(0,8).map(n => hex(n,2)).join(' ') + " ...";
	return `<Buffer ${hexstr}>`;
    }
    return value;
}
function default_replacer (key, value) {
    if (typeof value === 'object' && value !== null && value.type === "Buffer") {
	let hexstr =  value.data.map(n => hex(n,2)).join(' ');
	return `<Buffer ${hexstr}>`;
    }
    return value;
}

function json ( input, indent = 4, replacer = default_replacer_truncate_buffers ) {
    if ( typeof input === "string" )
	return JSON.parse( input );
    else
	return JSON.stringify( input, replacer, indent );
}
function jsonraw ( input, indent = 4 ) {
    if ( typeof input === "string" )
	return JSON.parse( input );
    else
	return JSON.stringify( input, default_replacer, indent );
}

function b64 ( buf ) {
    return typeof buf === "string"
	? Buffer.from( buf, "base64")
	: buf.toString("base64");
}

module.exports = {
    hex,
    json,
    jsonraw,
    b64,
};
