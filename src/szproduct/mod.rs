// use serde_json::Value;

pub mod szproduct_y {

    tonic::include_proto!("szproduct");

    use std::sync::OnceLock;
    use sz_product_client::SzProductClient;
    use tokio::runtime::Runtime;
    use tonic::transport::Channel;

    // Global runtime that persists for the lifetime of the program
    static RUNTIME: OnceLock<Runtime> = OnceLock::new();

    pub(crate) fn get_runtime() -> &'static Runtime {
        RUNTIME.get_or_init(|| {
            tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .expect("Failed to create tokio runtime")
        })
    }

    pub struct SzProduct {
        pub grpc_client: SzProductClient<Channel>,
    }

    impl SzProduct {
        pub fn get_license(&mut self) -> Result<String, Box<dyn std::error::Error>> {
            let mut client = self.grpc_client.clone();
            let rt = get_runtime();

            let response = rt.block_on(async move {
                let request = tonic::Request::new(GetLicenseRequest {});
                client.get_license(request).await
            })?;

            Ok(response.get_ref().clone().result)
        }

        pub fn get_version(&mut self) -> Result<String, Box<dyn std::error::Error>> {
            let mut client = self.grpc_client.clone();
            let rt = get_runtime();

            let response = rt.block_on(async move {
                let request = tonic::Request::new(GetVersionRequest {});
                client.get_version(request).await
            })?;

            Ok(response.get_ref().clone().result)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::szproduct_y::sz_product_client::SzProductClient;
    use tonic::transport::Channel;

    fn is_valid_json(s: String) -> bool {
        let result = serde_json::from_str::<serde_json::Value>(&s).is_ok();
        println!(">>>>>> is_valid_json:{:?}; JSON: {:?}", result, s);
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
        super::szproduct_y::SzProduct {
            grpc_client: get_grpc_client(),
        }
    }

    #[test]
    fn test_get_license() {
        let result = get_szproduct().get_license();
        dbg!(&result);
        assert!(result.is_ok_and(is_valid_json));
    }

    #[test]
    fn test_get_version() {
        let result = get_szproduct().get_version();
        dbg!(&result);
        assert!(result.is_ok_and(is_valid_json));
    }
}
