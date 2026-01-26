// use serde_json::Value;

#[cfg(test)]
mod tests;

pub mod szdiagnostic_y {

    tonic::include_proto!("szdiagnostic");

    use std::sync::OnceLock;
    use sz_diagnostic_client::SzDiagnosticClient;
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

    pub struct SzDiagnostic {
        pub grpc_client: SzDiagnosticClient<Channel>,
    }

    impl SzDiagnostic {
        pub fn destroy(&mut self) -> Result<(), Box<dyn std::error::Error>> {
            Ok(())
        }

        pub fn check_repository_performance(
            &mut self,
            seconds_to_run: i32,
        ) -> Result<String, Box<dyn std::error::Error>> {
            let mut client = self.grpc_client.clone();
            let rt = get_runtime();

            let response = rt.block_on(async move {
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
            let rt = get_runtime();

            let response = rt.block_on(async move {
                let request = tonic::Request::new(GetFeatureRequest { feature_id });
                client.get_feature(request).await
            })?;

            Ok(response.get_ref().clone().result)
        }

        pub fn get_repository_info(&mut self) -> Result<String, Box<dyn std::error::Error>> {
            let mut client = self.grpc_client.clone();
            let rt = get_runtime();

            let response = rt.block_on(async move {
                let request = tonic::Request::new(GetRepositoryInfoRequest {});
                client.get_repository_info(request).await
            })?;

            Ok(response.get_ref().clone().result)
        }

        pub fn purge_repository(&mut self) -> Result<(), Box<dyn std::error::Error>> {
            let mut client = self.grpc_client.clone();
            let rt = get_runtime();

            rt.block_on(async move {
                let request = tonic::Request::new(PurgeRepositoryRequest {});
                client.purge_repository(request).await
            })?;

            Ok(())
        }

        pub fn reinitialize(&mut self, config_id: i64) -> Result<(), Box<dyn std::error::Error>> {
            let mut client = self.grpc_client.clone();
            let rt = get_runtime();

            rt.block_on(async move {
                let request = tonic::Request::new(ReinitializeRequest { config_id });
                client.reinitialize(request).await
            })?;

            Ok(())
        }
    }
}
