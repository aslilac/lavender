const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const path = require("path");
const { DefinePlugin } = require("webpack");
const WebpackBar = require("webpackbar");


module.exports = (env, argv) => ({
	// ------ webpack ------
	entry: "./client/client.tsx",
	mode: "development",
	devtool: "source-map",
	stats: "minimal",

	resolve: {
		extensions: [".js", ".ts", ".tsx", ".wasm"],
	},

	output: {
		path: path.resolve(__dirname, "dist"),
		filename: "client.js",
	},

	performance: {
		hints: false,
	},

	devServer: {
		compress: true,
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
			template: "client/index.html",
		}),

		// ------ Define variables at build time ------
		new DefinePlugin({
			'webpack_mode': JSON.stringify(argv.mode),
		}),

		// ------ wasm-pack ------
		new WasmPackPlugin({
			crateDirectory: path.resolve(__dirname, "."),
			outDir: "target/wasm-pack",
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
