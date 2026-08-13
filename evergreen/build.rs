

fn main() {
    protobuf_codegen::CodeGen::new()
    .include("proto")
    .input("types.proto")
    .generate_and_compile()
    .unwrap();
}