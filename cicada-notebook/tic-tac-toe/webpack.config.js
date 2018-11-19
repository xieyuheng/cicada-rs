const HtmlWebPackPlugin = require("html-webpack-plugin");

module.exports = {
    entry: "./src/index.tsx",
    mode: "development",
    resolve: {
        // Add '.ts' and '.tsx' as resolvable extensions.
        extensions: [".ts", ".tsx", ".js", ".json"],
    },
    module: {
        rules: [
            { test: /\.js$/, exclude: /node_modules/, use: ["babel-loader"] },
            { test: /\.css$/, use: ["style-loader", "css-loader"] },
            { test: /\.tsx?$/, loader: "awesome-typescript-loader" },
            { enforce: "pre", test: /\.js$/, loader: "source-map-loader" },
        ]
    },
    externals: {
        "react": "React",
        "react-dom": "ReactDOM"
    },
    plugins: [
        new HtmlWebPackPlugin({ template: "./src/index.html" }),
    ],
};
