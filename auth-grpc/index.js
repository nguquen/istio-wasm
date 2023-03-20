const PROTO_PATH = __dirname + "/auth.proto";

const grpc = require("@grpc/grpc-js");
const protoLoader = require("@grpc/proto-loader");
const packageDefinition = protoLoader.loadSync(PROTO_PATH, {
  keepCase: true,
  longs: String,
  enums: String,
  defaults: true,
  oneofs: true,
});

const auth_proto = grpc.loadPackageDefinition(packageDefinition).auth;

const checkToken = (call, callback) => {
  callback(null, { success: call.request.token == "secured" });
};

const main = () => {
  console.log("server started at 5005");
  var server = new grpc.Server();
  server.addService(auth_proto.Auth.service, { checkToken });
  server.bindAsync(
    "0.0.0.0:5005",
    grpc.ServerCredentials.createInsecure(),
    () => {
      server.start();
    }
  );
};

main();
