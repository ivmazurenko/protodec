extern crate protoc_rust;

fn main() {
    generate_test_data();
}

fn generate_test_data() {
    protoc_rust::Codegen::new()
        .out_dir("src/tests")
        .inputs(&["./src/tests/test_data.proto"])
        .include("./src/tests/")
        .run()
        .expect("Running protoc failed.");
}
