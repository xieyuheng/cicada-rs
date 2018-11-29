const path = require("path");
const HtmlWebpackPlugin = require ("html-webpack-plugin");
const webpack = require ("webpack");

module.exports = {
    entry: "./src/index.tsx",
    mode: "development",
    resolve: {
        extensions: [ ".ts", ".tsx", ".js", ".json", ".wasm" ],
    },
    module: {
        rules: [
            { test: /\.js$/, exclude: /node_modules/, use: ["babel-loader"] },
            { test: /\.css$/, use: ["style-loader", "css-loader"] },
            { test: /\.tsx?$/, loader: "awesome-typescript-loader" },
            { test: /\.wasm$/, type: "webassembly/experimental" },
            { enforce: "pre", test: /\.js$/, loader: "source-map-loader" },
        ]
    },
    externals: {
        "react": "React",
        "react-dom": "ReactDOM",
    },
    plugins: [
        new HtmlWebpackPlugin({
            template: "./src/index.html",
        }),
        // Have this example work in Edge
        // which doesn"t ship `TextEncoder` or
        // `TextDecoder` at this time.
        new webpack.ProvidePlugin({
            TextDecoder: ["text-encoding", "TextDecoder"],
            TextEncoder: ["text-encoding", "TextEncoder"],
        }),
    ],
};
