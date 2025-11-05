use szproduct::GetVersionRequest;
use szproduct::sz_product_client::SzProductClient;

pub mod szproduct {
    tonic::include_proto!("szproduct");
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[tokio::main]
pub async fn get_version() -> Result<String, Box<dyn std::error::Error>> {
    let mut client = SzProductClient::connect("http://0.0.0.0:8261").await?;
    let request = tonic::Request::new(GetVersionRequest {});
    let response = client.get_version(request).await?;
    let response = response.get_ref().clone();
    Ok(response.result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
