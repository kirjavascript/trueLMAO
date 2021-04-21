const webpack = require('webpack');
const MiniCssExtractPlugin = require('mini-css-extract-plugin');

module.exports = (env={}, args={}) => {

    const config = {
        mode: env.dev ? 'development' : 'production',
        entry : {
            main: './app/main.js',
            styles: './styles/main.scss',
        },
        output: {
            path:     __dirname + '/../static/',
            filename: '[name].js',
        },
        module: {
            rules: [
                {
                    test: /\.jsx?$/,
                    exclude: env.dev ? /node_modules/ : void 0,
                    use: [
                        {
                            loader: 'babel-loader',
                            options: {
                                babelrc: false,
                                presets: [
                                    '@babel/preset-env',
                                    '@babel/preset-react',
                                ],
                            }
                        }
                    ],
                },
                {
                    test:  /\.s?[ac]ss$/,
                    use: [
                        MiniCssExtractPlugin.loader, // in lieu of style-loader
                        { loader:'css-loader', options: { url: false } },
                        { loader:'sass-loader' },
                    ],
                },
            ],
        },
        plugins: [
            new webpack.DefinePlugin({
                __DEV__: env.dev
            }),
            new MiniCssExtractPlugin({
                filename: "[name].css",
            }),
        ],
        resolve: {
            extensions: ['.js', '.json', '.jsx'],
            alias: {
                '#wasm': __dirname + '/../pkg/',
            },
        },
        devtool: env.dev && 'source-map',
    };

    return config;
};
