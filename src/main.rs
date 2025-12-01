use senzing_sdk_rust_grpc::get_version;

fn main() {
    let response = get_version();
    println!("RESPONSE={:?}", response);
}
