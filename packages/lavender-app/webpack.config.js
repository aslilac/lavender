"use strict";

const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const path = require("path");
const webpack = require("webpack");
const WebpackBar = require("webpackbar");

/**
 * @type {webpack.ConfigurationFactory}
 */
module.exports = (env, argv) => ({
	// ------ webpack ------
	entry: "./src/app.tsx",
	mode: "development",
	devtool: "source-map",
	stats: "minimal",

	resolve: {
		extensions: [".js", ".ts", ".tsx", ".wasm"],
	},

	output: {
		path: path.resolve(__dirname, "target"),
		filename: "client.js",
	},

	performance: {
		hints: false,
	},

	devServer: {
		compress: true,
		contentBase: path.join(__dirname, "public"),
		host: "::",
		port: 1234,
	},

	plugins: [
		// ------ Progress!! ------
		new WebpackBar({
			color: "#9084e6",
		}),

		// ------ Creates our index.html file ------
		new HtmlWebpackPlugin({
			template: "src/index.html",
		}),

		// ------ Define variables at build time ------
		new webpack.DefinePlugin({
			webpack_mode: JSON.stringify(argv.mode),
		}),

		// ------ wasm-pack ------
		new WasmPackPlugin({
			crateDirectory: path.join(__dirname, "../lavender-core"),
			outDir: "target",
		}),
	],

	// ------ babel ------
	module: {
		rules: [
			{
				test: /\.tsx?$/,
				exclude: /node_modules/,
				use: {
					loader: require.resolve("babel-loader"),
					options: {
						presets: [
							require.resolve("@babel/preset-env"),
							require.resolve("@babel/preset-react"),
							require.resolve("@babel/preset-typescript"),
						],
					},
				},
			},
		],
	},
});
