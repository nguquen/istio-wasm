const http = require("http");

console.log("server started at :8080");

http
  .createServer(function (req, res) {
    res.write("Hello\n");
    res.end();
  })
  .listen(8080);
