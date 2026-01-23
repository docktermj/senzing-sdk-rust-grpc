// use serde_json::Value;

use super::szproduct_y;

#[cfg(test)]
mod test {
    use super::szproduct_y::sz_product_client::SzProductClient;
    use tonic::transport::Channel;

    // ------------------------------------------------------------------------
    // Tests
    // ------------------------------------------------------------------------

    #[test]
    fn test_destroy() {
        let result = get_szproduct().destroy();
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_license() {
        let result = get_szproduct().get_license();
        assert!(result.is_ok_and(is_valid_json));
    }

    #[test]
    fn test_get_version() {
        let result = get_szproduct().get_version();
        assert!(result.is_ok_and(is_valid_json));
    }

    // ------------------------------------------------------------------------
    // Test helper functions
    // ------------------------------------------------------------------------

    fn is_valid_json(s: String) -> bool {
        let result = serde_json::from_str::<serde_json::Value>(&s).is_ok();
        println!("\n>>>>>> is_valid_json:{:?}; JSON: {:?}", result, s);
        result
    }

    async fn get_grpc_client_async()
    -> Result<SzProductClient<Channel>, Box<dyn std::error::Error + Send>> {
        let result = SzProductClient::connect("http://0.0.0.0:8261").await;
        Ok(result.unwrap())
    }

    pub fn get_grpc_client() -> SzProductClient<Channel> {
        // Use the same global runtime
        let rt = super::szproduct_y::get_runtime();
        rt.block_on(async move { get_grpc_client_async().await })
            .unwrap()
    }

    pub fn get_szproduct() -> super::szproduct_y::SzProduct {
        super::szproduct_y::SzProduct::new(get_grpc_client())
    }
}
