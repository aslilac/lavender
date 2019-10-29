const path = require( 'path' );
const HtmlWebpackPlugin = require( 'html-webpack-plugin' );
const WasmPackPlugin = require( '@wasm-tool/wasm-pack-plugin' );

module.exports = {
  entry: './client/client.tsx',
  mode: 'development',
  output: {
    path: path.resolve( __dirname, 'dist' ),
    filename: 'client.js',
  },
  plugins: [
    new HtmlWebpackPlugin({
      template: 'client/index.html'
    }),
    new WasmPackPlugin({
      crateDirectory: path.resolve( __dirname, '.' ),
      outDir: 'wasm'
    })
  ],
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
  },
  performance: {
    hints: false
  },
  devServer: {
    compress: true,
    host: '::',
    port: 1234
  }
};
