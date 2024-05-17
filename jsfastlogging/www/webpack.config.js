const path = require('path');
const CopyWebpackPlugin = require("copy-webpack-plugin");
//const HtmlWebpackPlugin = require("html-webpack-plugin");
//const NodePolyfillPlugin = require("node-polyfill-webpack-plugin");
const webpack = require("webpack");

module.exports = {
    mode: "development",
    target: "web",
    entry: "./bootstrap.js",
    output: {
        path: path.resolve(__dirname, "dist"),
        filename: "bootstrap.js",
    },
    plugins: [
        //new HtmlWebpackPlugin({
        //    title: "example",
        //    template: "index.html",
        //}),
        // instead of fallback
        //new NodePolyfillPlugin()
        new CopyWebpackPlugin({
            patterns: ['index.html']
        })
    ],
    experiments: {
        asyncWebAssembly: true,
        syncWebAssembly: true,
    },
};
