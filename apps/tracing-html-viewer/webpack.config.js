const path = require('path');



module.exports = {
    entry: './src/index.tsx',
    output: {
        path: path.resolve(__dirname, 'dist')
    },
    module: {
        rules: [
            {
                test: /\.ts|\.tsx$/,
                exclude: /(node_modules|bower_components)/,
                use: {
                    loader: "swc-loader",
                    options: {
                        jsc: {
                            parser: {
                                syntax: "typescript"
                            }
                        }
                    }
                }
            }
        ],
    },

    resolve: {
        extensions: [".js", ".ts", ".json", ".tsx", ".css"],
    },
}