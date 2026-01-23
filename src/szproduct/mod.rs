// use serde_json::Value;

#[cfg(test)]
mod tests;

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
        pub fn destroy(&mut self) -> Result<(), Box<dyn std::error::Error>> {
            Ok(())
        }

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
