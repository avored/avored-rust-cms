const path = require('path');

module.exports = {
    mode: 'production',
    watch: true,
    entry: './resources/js/app.js',
    output: {
        filename: 'app.js',
        path: path.resolve(__dirname, 'public/js'),
    },
};
