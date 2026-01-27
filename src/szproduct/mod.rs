// use serde_json::Value;

#[cfg(test)]
mod tests;

pub mod szproduct_y {

    tonic::include_proto!("szproduct");

    use std::sync::Arc;
    use sz_product_client::SzProductClient;
    use tokio::runtime::Runtime;
    use tonic::transport::Channel;

    pub struct SzProduct {
        grpc_client: SzProductClient<Channel>,
        runtime: Arc<Runtime>,
    }

    impl SzProduct {
        pub fn new(runtime: Arc<Runtime>, grpc_client: SzProductClient<Channel>) -> Self {
            Self {
                grpc_client,
                runtime,
            }
        }

        pub fn destroy(&mut self) -> Result<(), Box<dyn std::error::Error>> {
            Ok(())
        }

        pub fn get_license(&mut self) -> Result<String, Box<dyn std::error::Error>> {
            let mut client = self.grpc_client.clone();

            let response = self.runtime.block_on(async move {
                let request = tonic::Request::new(GetLicenseRequest {});
                client.get_license(request).await
            })?;

            Ok(response.get_ref().clone().result)
        }

        pub fn get_version(&mut self) -> Result<String, Box<dyn std::error::Error>> {
            let mut client = self.grpc_client.clone();

            let response = self.runtime.block_on(async move {
                let request = tonic::Request::new(GetVersionRequest {});
                client.get_version(request).await
            })?;

            Ok(response.get_ref().clone().result)
        }
    }
}
