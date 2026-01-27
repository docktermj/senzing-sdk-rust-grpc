#[cfg(test)]
mod tests;

pub mod szproduct_y {

    tonic::include_proto!("szproduct");

    use crate::is_destroyed;
    use std::cell::Cell;
    use std::sync::Arc;
    use sz_product_client::SzProductClient;
    use tokio::runtime::Runtime;
    use tonic::transport::Channel;

    // ------------------------------------------------------------------------
    // SzProductGrpc
    // ------------------------------------------------------------------------

    // For explanation of this technique, view https://www.youtube.com/watch?v=_ccDqRTx-JU

    pub struct Uninitialized;
    pub struct Initialized;

    pub struct SzProductGrpc<State = Uninitialized> {
        grpc_client: SzProductClient<Channel>,
        runtime: Arc<Runtime>,
        is_destroyed: Cell<bool>,
        state: std::marker::PhantomData<State>,
    }

    impl SzProductGrpc<Uninitialized> {
        pub fn new(
            runtime: Arc<Runtime>,
            grpc_client: SzProductClient<Channel>,
        ) -> SzProductGrpc<Initialized> {
            SzProductGrpc {
                grpc_client,
                runtime,
                state: std::marker::PhantomData::<Initialized>,
                is_destroyed: Cell::new(false),
            }
        }
    }

    impl SzProductGrpc<Initialized> {
        pub fn destroy(&mut self) -> Result<(), Box<dyn std::error::Error>> {
            self.is_destroyed.set(true);
            Ok(())
        }

        pub fn get_license(&mut self) -> Result<String, Box<dyn std::error::Error>> {
            is_destroyed!(self.is_destroyed, "SzProduct has been destroyed");

            let mut client = self.grpc_client.clone();

            let response = self.runtime.block_on(async move {
                let request = tonic::Request::new(GetLicenseRequest {});
                client.get_license(request).await
            })?;

            Ok(response.get_ref().clone().result)
        }

        pub fn get_version(&mut self) -> Result<String, Box<dyn std::error::Error>> {
            is_destroyed!(self.is_destroyed, "SzProduct has been destroyed");

            let mut client = self.grpc_client.clone();

            let response = self.runtime.block_on(async move {
                let request = tonic::Request::new(GetVersionRequest {});
                client.get_version(request).await
            })?;

            Ok(response.get_ref().clone().result)
        }
    }

    // ------------------------------------------------------------------------
    // SzProduct traits for SzProductGrpc
    // ------------------------------------------------------------------------

    // Type alias for use in trait definitions and public API
    pub type SzProduct = SzProductGrpc<Initialized>;

    impl crate::traits::SzProduct for SzProductGrpc<Initialized> {
        fn destroy(&mut self) -> Result<(), Box<dyn std::error::Error>> {
            self.destroy()
        }

        fn get_license(&mut self) -> Result<String, Box<dyn std::error::Error>> {
            self.get_license()
        }

        fn get_version(&mut self) -> Result<String, Box<dyn std::error::Error>> {
            self.get_version()
        }
    }
}
