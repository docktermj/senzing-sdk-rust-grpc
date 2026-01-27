#[cfg(test)]
mod test {
    use crate::helper::json::is_valid_json;
    use crate::helper::{create_grpc_channel, create_grpc_channel_blocking};
    use crate::szabstractfactory::szabstractfactory_y::{self};
    use crate::szdiagnostic::szdiagnostic_y;
    use crate::szproduct::szproduct_y;
    use crate::traits::SzAbstractFactory as SzAbstractFactoryTrait;

    // ------------------------------------------------------------------------
    // Tests
    // ------------------------------------------------------------------------

    #[test]
    fn test_create_product() {
        let factory = get_szabstractfactory();
        let result: Result<szproduct_y::SzProduct, Box<dyn std::error::Error>> =
            factory.create_product();
        assert!(result.is_ok());

        if let Ok(mut product) = result {
            let version_result = product.get_version();
            assert!(version_result.is_ok_and(is_valid_json));
        }
    }

    #[test]
    fn test_create_product_via_trait() {
        let factory = get_szabstractfactory_as_trait();
        let result = factory.create_product();
        assert!(result.is_ok());

        if let Ok(mut product) = result {
            let version_result = product.get_version();
            assert!(version_result.is_ok_and(is_valid_json));
        }
    }

    #[test]
    fn test_create_product_via_channel() {
        let factory = get_szabstractfactory();
        let result = factory.create_product();
        assert!(result.is_ok());

        if let Ok(mut product) = result {
            let version_result = product.get_version();
            assert!(version_result.is_ok_and(is_valid_json));
        }
    }

    #[test]
    fn test_create_diagnostic() {
        let factory = get_szabstractfactory();

        let result: Result<szdiagnostic_y::SzDiagnostic, Box<dyn std::error::Error>> =
            factory.create_diagnostic();
        assert!(result.is_ok());

        if let Ok(mut diagnostic) = result {
            let repo_info_result = diagnostic.get_repository_info();
            assert!(repo_info_result.is_ok_and(is_valid_json));
        }
    }

    #[test]
    fn test_factory_creates_working_product() {
        let factory = get_szabstractfactory();
        let mut product = factory.create_product().expect("Failed to create product");

        let license_result = product.get_license();
        assert!(license_result.is_ok_and(is_valid_json));

        let version_result = product.get_version();
        assert!(version_result.is_ok_and(is_valid_json));

        let destroy_result = product.destroy();
        assert!(destroy_result.is_ok());
    }

    #[test]
    fn test_factory_creates_working_diagnostic() {
        let factory = get_szabstractfactory();
        let mut diagnostic = factory
            .create_diagnostic()
            .expect("Failed to create diagnostic");

        let repo_info_result = diagnostic.get_repository_info();
        assert!(repo_info_result.is_ok_and(is_valid_json));

        let destroy_result = diagnostic.destroy();
        assert!(destroy_result.is_ok());
    }

    #[test]
    fn test_factory_creates_multiple_instances() {
        let factory = get_szabstractfactory();

        let product1 = factory.create_product();
        assert!(product1.is_ok());

        let product2 = factory.create_product();
        assert!(product2.is_ok());

        let diagnostic1 = factory.create_diagnostic();
        assert!(diagnostic1.is_ok());

        let diagnostic2 = factory.create_diagnostic();
        assert!(diagnostic2.is_ok());
    }

    // ------------------------------------------------------------------------
    // Test helper functions
    // ------------------------------------------------------------------------

    // fn get_szabstractfactory() -> szabstractfactory_y::SzAbstractFactoryGrpc<Initialized> {
    //     szabstractfactory_y::SzAbstractFactory::new_from_url("http://0.0.0.0:8261".to_string())
    // }

    // Uses helper.create_grpc_channel_blocking.
    fn get_szabstractfactory() -> impl crate::traits::SzAbstractFactory {
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Failed to create tokio runtime");
        let grpc_channel = create_grpc_channel_blocking("0.0.0.0:8261", &runtime).unwrap();
        szabstractfactory_y::SzAbstractFactory::new_using_tonic_and_tokio(grpc_channel, runtime)
    }

    // Uses helper.create_grpc_channel.
    fn get_szabstractfactory_as_trait() -> impl crate::traits::SzAbstractFactory {
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Failed to create tokio runtime");
        let grpc_channel = runtime
            .block_on(create_grpc_channel("0.0.0.0:8261"))
            .unwrap();
        szabstractfactory_y::SzAbstractFactory::new_using_tonic_and_tokio(grpc_channel, runtime)
    }
}
