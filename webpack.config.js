const path = require('path');

module.exports = {
  mode: 'production',
  entry: './index.js',

  output: {
    path: path.resolve(__dirname, 'docs'),
    filename: 'index.js',
    publicPath: 'docs'
  }
};
