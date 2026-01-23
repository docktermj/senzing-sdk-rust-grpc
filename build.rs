fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_prost_build::compile_protos("proto/szconfig.proto")
        .unwrap_or_else(|e| panic!("Failed to compile proto/szconfig.proto {:?}", e));

    tonic_prost_build::compile_protos("proto/szconfigmanager.proto")
        .unwrap_or_else(|e| panic!("Failed to compile proto/szconfig.proto {:?}", e));

    tonic_prost_build::compile_protos("proto/szdiagnostic.proto")
        .unwrap_or_else(|e| panic!("Failed to compile proto/szdiagnostic.proto {:?}", e));

    tonic_prost_build::compile_protos("proto/szengine.proto")
        .unwrap_or_else(|e| panic!("Failed to compile proto/szengine.proto {:?}", e));

    tonic_prost_build::compile_protos("proto/szproduct.proto")
        .unwrap_or_else(|e| panic!("Failed to compile proto/szproduct.proto {:?}", e));

    Ok(())
}
