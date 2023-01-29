const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require("path");

module.exports = {
  entry: "./public/bootstrap.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "bootstrap.js",
  },
  plugins: [
    new CopyWebpackPlugin(["./public/index.html", "./public/index.css"]),
  ],
  module: { rules: [{ test: "/.ts$/", loader: "ts-loader" }] },
  experiments: {
    syncWebAssembly: true,
  },
};
