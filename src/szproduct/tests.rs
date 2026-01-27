// use serde_json::Value;

use super::szproduct_y;

#[cfg(test)]
mod test {
    use super::szproduct_y::sz_product_client::SzProductClient;
    use crate::helper::create_grpc_channel_blocking;
    use crate::helper::json::is_valid_json;
    use crate::helper::runtime::get_runtime;
    use std::sync::Arc;
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

    pub fn get_grpc_client() -> SzProductClient<Channel> {
        let runtime = get_runtime();
        let grpc_channel = create_grpc_channel_blocking("0.0.0.0:8261", runtime).unwrap();
        SzProductClient::new(grpc_channel)
    }

    pub fn get_szproduct() -> super::szproduct_y::SzProduct {
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Failed to create tokio runtime");
        super::szproduct_y::SzProduct::new(get_grpc_client(), Arc::new(runtime))
    }
}
