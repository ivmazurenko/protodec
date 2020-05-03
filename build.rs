extern crate protoc_rust;

fn main() {
    protoc_rust::Codegen::new()
        .out_dir("src/")
        .inputs(&["./src/test_data.proto"])
        .include("./src/")
        .run()
        .expect("Running protoc failed.");
}
