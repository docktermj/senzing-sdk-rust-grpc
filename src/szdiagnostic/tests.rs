// use serde_json::Value;

use super::szdiagnostic_y;

#[cfg(test)]
mod test {
    use super::szdiagnostic_y::sz_diagnostic_client::SzDiagnosticClient;
    use tonic::transport::Channel;

    // ------------------------------------------------------------------------
    // Tests
    // ------------------------------------------------------------------------

    #[test]
    fn test_destroy() {
        let result = get_szdiagnostic().destroy();
        assert!(result.is_ok());
    }

    #[test]
    fn test_check_repository_performance() {
        let result = get_szdiagnostic().check_repository_performance(5);
        assert!(result.is_ok_and(is_valid_json));
    }

    // #[test]
    // fn test_get_feature() {
    //     let result = get_szdiagnostic().get_feature(1);
    //     assert!(result.is_ok_and(is_valid_json));
    // }

    #[test]
    fn test_get_repository_info() {
        let result = get_szdiagnostic().get_repository_info();
        assert!(result.is_ok_and(is_valid_json));
    }

    #[test]
    fn test_purge_repository() {
        let result = get_szdiagnostic().purge_repository();
        assert!(result.is_ok());
    }

    // #[test]
    // fn test_reinitialize() {
    //     let result = get_szdiagnostic().reinitialize(1);
    //     assert!(result.is_ok());
    // }

    // ------------------------------------------------------------------------
    // Test helper functions
    // ------------------------------------------------------------------------

    fn is_valid_json(s: String) -> bool {
        let result = serde_json::from_str::<serde_json::Value>(&s).is_ok();
        println!("\n>>>>>> is_valid_json:{:?}; JSON: {:?}", result, s);
        result
    }

    async fn get_grpc_client_async()
    -> Result<SzDiagnosticClient<Channel>, Box<dyn std::error::Error + Send>> {
        let result = SzDiagnosticClient::connect("http://0.0.0.0:8261").await;
        Ok(result.unwrap())
    }

    pub fn get_grpc_client() -> SzDiagnosticClient<Channel> {
        // Use the same global runtime
        let rt = super::szdiagnostic_y::get_runtime();
        rt.block_on(async move { get_grpc_client_async().await })
            .unwrap()
    }

    pub fn get_szdiagnostic() -> super::szdiagnostic_y::SzDiagnostic {
        super::szdiagnostic_y::SzDiagnostic {
            grpc_client: get_grpc_client(),
        }
    }
}
