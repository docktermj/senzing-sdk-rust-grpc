use super::szproduct_y;

#[cfg(test)]
mod test {
    use super::szproduct_y::sz_product_client::SzProductClient;
    use crate::helpers::create_grpc_channel_blocking;
    use crate::helpers::json::is_valid_json;
    use crate::helpers::runtime::build_runtime;
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

    pub fn get_grpc_client(runtime: &tokio::runtime::Runtime) -> SzProductClient<Channel> {
        let grpc_channel = create_grpc_channel_blocking("0.0.0.0:8261", runtime).unwrap();
        SzProductClient::new(grpc_channel)
    }

    pub fn get_szproduct() -> super::szproduct_y::SzProduct {
        let runtime = build_runtime();
        let grpc_client = get_grpc_client(&runtime);
        super::szproduct_y::SzProductGrpc::new(Arc::new(runtime), grpc_client)
    }
}
