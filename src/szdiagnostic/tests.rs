use super::szdiagnostic_y;

#[cfg(test)]
mod test {

    use super::szdiagnostic_y::sz_diagnostic_client::SzDiagnosticClient;
    use crate::error::build_senzing_error;
    use crate::helper::create_grpc_channel_blocking;
    use crate::helper::json::is_valid_json;
    use crate::helper::runtime::build_runtime;

    use std::sync::Arc;
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

    #[test]
    fn test_get_feature() {
        let result = get_szdiagnostic().get_feature(1);
        if let Err(e) = &result {
            let message = e.to_string();
            println!(">>>>>> Error message: |{}|", message);
        }
        if let Err(e) = result.as_ref() {
            let senzing_error = build_senzing_error(e);
            let reason = senzing_error.reason();
            // Verify we can extract the reason from the error
            if !reason.is_empty() {
                println!("Extracted reason: {}", reason);
            }
        }
        assert!(result.is_err());
    }

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

    pub fn get_grpc_client(runtime: &tokio::runtime::Runtime) -> SzDiagnosticClient<Channel> {
        let grpc_channel = create_grpc_channel_blocking("0.0.0.0:8261", runtime).unwrap();
        SzDiagnosticClient::new(grpc_channel)
    }

    pub fn get_szdiagnostic() -> super::szdiagnostic_y::SzDiagnostic {
        let runtime = build_runtime();
        let grpc_client = get_grpc_client(&runtime);
        super::szdiagnostic_y::SzDiagnosticGrpc::new(Arc::new(runtime), grpc_client)
    }
}
