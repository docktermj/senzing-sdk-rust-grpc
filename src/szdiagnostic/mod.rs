#[cfg(test)]
mod tests;

pub mod szdiagnostic_y {

    tonic::include_proto!("szdiagnostic");

    use crate::is_destroyed;
    use std::cell::Cell;
    use std::sync::Arc;
    use sz_diagnostic_client::SzDiagnosticClient;
    use tokio::runtime::Runtime;
    use tonic::transport::Channel;

    // ------------------------------------------------------------------------
    // SzDiagnosticGrpc
    // ------------------------------------------------------------------------

    // For explanation of this technique, view https://www.youtube.com/watch?v=_ccDqRTx-JU

    pub struct Uninitialized;
    pub struct Initialized;

    pub struct SzDiagnosticGrpc<State = Uninitialized> {
        grpc_client: SzDiagnosticClient<Channel>,
        runtime: Arc<Runtime>,
        is_destroyed: Cell<bool>,
        state: std::marker::PhantomData<State>,
    }

    impl SzDiagnosticGrpc<Uninitialized> {
        pub fn new(
            runtime: Arc<Runtime>,
            grpc_client: SzDiagnosticClient<Channel>,
        ) -> SzDiagnosticGrpc<Initialized> {
            SzDiagnosticGrpc {
                grpc_client,
                runtime,
                state: std::marker::PhantomData::<Initialized>,
                is_destroyed: Cell::new(false),
            }
        }
    }

    impl SzDiagnosticGrpc<Initialized> {
        pub fn destroy(&mut self) -> Result<(), Box<dyn std::error::Error>> {
            self.is_destroyed.set(true);
            Ok(())
        }

        pub fn check_repository_performance(
            &mut self,
            seconds_to_run: i32,
        ) -> Result<String, Box<dyn std::error::Error>> {
            is_destroyed!(self.is_destroyed, "SzDiagnostic has been destroyed");

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
            is_destroyed!(self.is_destroyed, "SzDiagnostic has been destroyed");

            let mut client = self.grpc_client.clone();

            let response = self.runtime.block_on(async move {
                let request = tonic::Request::new(GetFeatureRequest { feature_id });
                client.get_feature(request).await
            })?;

            Ok(response.get_ref().clone().result)
        }

        pub fn get_repository_info(&mut self) -> Result<String, Box<dyn std::error::Error>> {
            is_destroyed!(self.is_destroyed, "SzDiagnostic has been destroyed");

            let mut client = self.grpc_client.clone();

            let response = self.runtime.block_on(async move {
                let request = tonic::Request::new(GetRepositoryInfoRequest {});
                client.get_repository_info(request).await
            })?;

            Ok(response.get_ref().clone().result)
        }

        pub fn purge_repository(&mut self) -> Result<(), Box<dyn std::error::Error>> {
            is_destroyed!(self.is_destroyed, "SzDiagnostic has been destroyed");

            let mut client = self.grpc_client.clone();

            self.runtime.block_on(async move {
                let request = tonic::Request::new(PurgeRepositoryRequest {});
                client.purge_repository(request).await
            })?;

            Ok(())
        }

        pub fn reinitialize(&mut self, config_id: i64) -> Result<(), Box<dyn std::error::Error>> {
            is_destroyed!(self.is_destroyed, "SzDiagnostic has been destroyed");

            let mut client = self.grpc_client.clone();

            self.runtime.block_on(async move {
                let request = tonic::Request::new(ReinitializeRequest { config_id });
                client.reinitialize(request).await
            })?;

            Ok(())
        }
    }

    // ------------------------------------------------------------------------
    // SzDiagnostic trait implementation
    // ------------------------------------------------------------------------

    // Type alias for use in trait definitions and public API
    pub type SzDiagnostic = SzDiagnosticGrpc<Initialized>;

    impl crate::traits::SzDiagnostic for SzDiagnosticGrpc<Initialized> {
        fn destroy(&mut self) -> Result<(), Box<dyn std::error::Error>> {
            self.destroy()
        }

        fn check_repository_performance(
            &mut self,
            seconds_to_run: i32,
        ) -> Result<String, Box<dyn std::error::Error>> {
            self.check_repository_performance(seconds_to_run)
        }

        fn get_feature(&mut self, feature_id: i64) -> Result<String, Box<dyn std::error::Error>> {
            self.get_feature(feature_id)
        }

        fn get_repository_info(&mut self) -> Result<String, Box<dyn std::error::Error>> {
            self.get_repository_info()
        }

        fn purge_repository(&mut self) -> Result<(), Box<dyn std::error::Error>> {
            self.purge_repository()
        }

        fn reinitialize(&mut self, config_id: i64) -> Result<(), Box<dyn std::error::Error>> {
            self.reinitialize(config_id)
        }
    }
}
