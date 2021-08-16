const path = require("path");

module.exports = function () {
  return {
    contentBase: path.join(__dirname, "build"),
    compress: true,
    port: 3000,
    proxy: {
      "/api": "http://localhost:8080",
    },
    historyApiFallback: true,
    host: "0.0.0.0",
    hot: true,
  };
};
