use senzing_sdk_rust_grpc::get_version;

pub mod szproduct {
    tonic::include_proto!("szproduct");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let mut client = SzProductClient::connect("http://0.0.0.0:8261").await?;
    // let request = tonic::Request::new(GetVersionRequest {});
    // let response = client.get_version(request).await?;

    let response = get_version();
    println!("RESPONSE={:?}", response);

    Ok(())
}
