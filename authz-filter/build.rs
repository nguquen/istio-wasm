extern crate protoc_rust;

fn main() {
    protoc_rust::Codegen::new()
        .out_dir("src")
        .inputs(&["auth.proto"])
        .include(".")
        .run()
        .expect("Running protoc failed.");
}
