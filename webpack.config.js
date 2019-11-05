const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');

module.exports = {
  // ------ webpack ------
  entry: './client/client.tsx',
  mode: 'development',
  resolve: {
    extensions: ['.js', '.ts', '.tsx', '.wasm']
  },
  output: {
    path: path.resolve(__dirname, 'dist'),
    filename: 'client.js',
  },
  performance: {
    hints: false
  },
  devServer: {
    compress: true,
    host: '::',
    port: 1234
  },

  // ------ wasm-pack ------
  plugins: [
    new HtmlWebpackPlugin({
      template: 'client/index.html'
    }),
    new WasmPackPlugin({
      crateDirectory: path.resolve(__dirname, '.'),
      outDir: 'target/wasm-pack'
    })
  ],

  // ------ babel ------
  module: {
    rules: [
      {
        test: /\.tsx?$/,
        exclude: /node_modules/,
        use: {
          loader: 'babel-loader',
          options: {
            presets: ['@babel/preset-env', '@babel/preset-react', '@babel/preset-typescript']
          }
        }
      }
    ],
  }
};
