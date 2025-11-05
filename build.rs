fn main() {
    tonic_prost_build::compile_protos("proto/szproduct.proto")
        .unwrap_or_else(|e| panic!("Failed to compile proto/szproduct.proto {:?}", e));
}
