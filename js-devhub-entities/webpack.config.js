const webpack			= require('webpack');

module.exports = {
    target: 'node',
    mode: 'production', // production | development
    entry: [ './src/index.js' ],
    output: {
	filename: 'devhub-entities.bundled.js',
	globalObject: 'this',
	library: {
	    "name": "DevHubEntities",
	    "type": "umd",
	},
    },
    stats: {
	colors: true
    },
    devtool: 'source-map',
};
