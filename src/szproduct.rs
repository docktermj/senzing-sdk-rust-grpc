// use serde_json::Value;
use szproduct_y::GetVersionRequest;
use szproduct_y::sz_product_client::SzProductClient;

pub mod szproduct_y {
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

    // fn is_valid_json(s: String) -> bool {
    //     println!("<<<<<<<< is_valid_json={:?}", s);

    //     serde_json::from_str::<serde_json::Value>(&s).is_ok()
    // }

    #[test]
    fn test_get_version() {
        let result = get_version();
        println!(">>>>> is_valid_json: {:?}", result);

        // assert!(result.is_ok_and(is_valid_json));
    }
}
