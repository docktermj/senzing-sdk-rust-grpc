// use serde_json::Value;

#[cfg(test)]
mod tests;

pub mod szdiagnostic_y {

    tonic::include_proto!("szdiagnostic");

    use std::sync::Arc;
    use sz_diagnostic_client::SzDiagnosticClient;
    use tokio::runtime::Runtime;
    use tonic::transport::Channel;

    pub struct SzDiagnostic {
        grpc_client: SzDiagnosticClient<Channel>,
        runtime: Arc<Runtime>,
    }

    impl SzDiagnostic {
        pub fn new(runtime: Arc<Runtime>, grpc_client: SzDiagnosticClient<Channel>) -> Self {
            Self {
                grpc_client,
                runtime,
            }
        }

        pub fn destroy(&mut self) -> Result<(), Box<dyn std::error::Error>> {
            Ok(())
        }

        pub fn check_repository_performance(
            &mut self,
            seconds_to_run: i32,
        ) -> Result<String, Box<dyn std::error::Error>> {
            let mut client = self.grpc_client.clone();

            let response = self.runtime.block_on(async move {
                let request =
                    tonic::Request::new(CheckRepositoryPerformanceRequest { seconds_to_run });
                client.check_repository_performance(request).await
            })?;

            Ok(response.get_ref().clone().result)
        }

        pub fn get_feature(
            &mut self,
            feature_id: i64,
        ) -> Result<String, Box<dyn std::error::Error>> {
            let mut client = self.grpc_client.clone();

            let response = self.runtime.block_on(async move {
                let request = tonic::Request::new(GetFeatureRequest { feature_id });
                client.get_feature(request).await
            })?;

            Ok(response.get_ref().clone().result)
        }

        pub fn get_repository_info(&mut self) -> Result<String, Box<dyn std::error::Error>> {
            let mut client = self.grpc_client.clone();

            let response = self.runtime.block_on(async move {
                let request = tonic::Request::new(GetRepositoryInfoRequest {});
                client.get_repository_info(request).await
            })?;

            Ok(response.get_ref().clone().result)
        }

        pub fn purge_repository(&mut self) -> Result<(), Box<dyn std::error::Error>> {
            let mut client = self.grpc_client.clone();

            self.runtime.block_on(async move {
                let request = tonic::Request::new(PurgeRepositoryRequest {});
                client.purge_repository(request).await
            })?;

            Ok(())
        }

        pub fn reinitialize(&mut self, config_id: i64) -> Result<(), Box<dyn std::error::Error>> {
            let mut client = self.grpc_client.clone();

            self.runtime.block_on(async move {
                let request = tonic::Request::new(ReinitializeRequest { config_id });
                client.reinitialize(request).await
            })?;

            Ok(())
        }
    }
}
