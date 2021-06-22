
function hex (n, pad = 8) {
    return ( "0".repeat(pad) + n.toString(16) ).substr(-pad);
}

function b64 ( buf ) {
    return typeof buf === "string"
	? Buffer.from( buf, "base64")
	: buf.toString("base64");
}

module.exports = {
    hex,
    b64,
};
