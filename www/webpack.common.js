const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require("path");

module.exports = {
  entry: "./src/index.ts",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "index.js",
  },
  plugins: [
    new CopyWebpackPlugin([
      "./public/index.html",
      "./public/index.css",
      "./public/favicon.ico",
      "./public/bootstrap.js",
    ]),
  ],
  resolve: {
    extensions: [".ts", ".js"],
  },
  module: {
    rules: [{ test: /\.ts$/, loader: "ts-loader", exclude: /node_modules/ }],
  },
  experiments: {
    asyncWebAssembly: true,
    topLevelAwait: true,
  },
};
