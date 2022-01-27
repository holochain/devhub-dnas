const webpack			= require('webpack');
const TerserPlugin		= require('terser-webpack-plugin');

module.exports = {
    target: "web",
    // mode: "development",
    mode: "production",
    entry: [ "./src/index.js" ],
    resolve: {
	mainFields: ["main"],
    },
    output: {
	filename: "devhub-entities.bundled.js",
	globalObject: "this",
	library: {
	    "name": "DevHubEntities",
	    "type": "umd",
	},
    },
    stats: {
	colors: true
    },
    devtool: "source-map",
    optimization: {
	minimizer: [
	    new TerserPlugin({
		terserOptions: {
		    keep_classnames: true,
		},
	    }),
	],
    },
};
