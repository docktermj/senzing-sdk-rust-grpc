#[cfg(test)]
mod tests;

pub mod szabstractfactory_y {

    use crate::szdiagnostic::szdiagnostic_y;
    use crate::szproduct::szproduct_y;
    use std::cell::Cell;
    use std::sync::OnceLock;
    use szdiagnostic_y::sz_diagnostic_client::SzDiagnosticClient;
    use szproduct_y::sz_product_client::SzProductClient;
    use tokio::runtime::Runtime;
    use tonic::transport::Channel;

    // ------------------------------------------------------------------------
    // Global runtime that persists for the lifetime of the program
    // ------------------------------------------------------------------------

    static RUNTIME: OnceLock<Runtime> = OnceLock::new();

    pub(crate) fn get_runtime() -> &'static Runtime {
        RUNTIME.get_or_init(|| {
            tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .expect("Failed to create tokio runtime")
        })
    }

    // ------------------------------------------------------------------------
    // SzAbstractFactory
    // ------------------------------------------------------------------------

    pub struct SzAbstractFactory;

    impl SzAbstractFactory {
        pub fn new_from_url(grpc_url: String) -> SzAbstractFactoryGrpc<Initialized> {
            let rt = get_runtime();
            let grpc_channel: tonic::transport::Channel = rt
                .block_on(async { tonic::transport::Endpoint::new(grpc_url)?.connect().await })
                .expect("Failed to connect to gRPC server");
            SzAbstractFactoryGrpc::new(grpc_channel)
        }

        pub fn new_from_grpc_channel(
            grpc_channel: tonic::transport::Channel,
        ) -> SzAbstractFactoryGrpc<Initialized> {
            SzAbstractFactoryGrpc::new(grpc_channel)
        }
    }

    // ------------------------------------------------------------------------
    // SzAbstractFactoryGrpc
    // ------------------------------------------------------------------------

    // For explanation of this technique, view https://www.youtube.com/watch?v=_ccDqRTx-JU

    pub struct Uninitialized;
    pub struct Initialized;

    pub struct SzAbstractFactoryGrpc<State = Uninitialized> {
        grpc_channel: tonic::transport::Channel,
        is_closed: Cell<bool>,
        state: std::marker::PhantomData<State>,
    }

    impl SzAbstractFactoryGrpc<Uninitialized> {
        pub fn new(grpc_channel: tonic::transport::Channel) -> SzAbstractFactoryGrpc<Initialized> {
            SzAbstractFactoryGrpc {
                grpc_channel,
                state: std::marker::PhantomData::<Initialized>,
                is_closed: Cell::new(false),
            }
        }
    }

    impl SzAbstractFactoryGrpc<Initialized> {
        pub fn create_product(&self) -> Result<szproduct_y::SzProduct, Box<dyn std::error::Error>> {
            if self.is_closed.get() {
                return Err("AbstractFactory has been destroyed".into());
            }
            let grpc_client: SzProductClient<Channel> =
                SzProductClient::new(self.grpc_channel.clone());
            Ok(szproduct_y::SzProduct::new(grpc_client))
        }

        pub fn create_diagnostic(
            &self,
        ) -> Result<szdiagnostic_y::SzDiagnostic, Box<dyn std::error::Error>> {
            if self.is_closed.get() {
                return Err("AbstractFactory has been destroyed".into());
            }
            let grpc_client: SzDiagnosticClient<Channel> =
                SzDiagnosticClient::new(self.grpc_channel.clone());
            Ok(szdiagnostic_y::SzDiagnostic { grpc_client })
        }

        pub fn close(&self) -> Result<(), Box<dyn std::error::Error>> {
            self.is_closed.set(true);
            Ok(())
        }
    }

    // impl<State> SzAbstractFactoryGrpc<State> {
    //     pub fn xx(grpc_url: String) -> SzAbstractFactoryGrpc<Initialized> {
    //     }
    // }

    // impl SzAbstractFactoryGrpc {
    //     pub fn new_from_url(grpc_url: String) -> SzAbstractFactoryGrpc<Initialized> {
    //         SzAbstractFactoryGrpc::new(grpc_url)
    //     }
    // }

    // ------------------------------------------------------------------------
    // SzAbstractFactory traits for SzAbstractFactoryGrpc
    // ------------------------------------------------------------------------

    impl crate::traits::SzAbstractFactory for SzAbstractFactoryGrpc<Initialized> {
        fn close(&self) -> Result<(), Box<dyn std::error::Error>> {
            self.close()
        }

        fn create_product(&self) -> Result<szproduct_y::SzProduct, Box<dyn std::error::Error>> {
            self.create_product()
        }

        fn create_diagnostic(
            &self,
        ) -> Result<szdiagnostic_y::SzDiagnostic, Box<dyn std::error::Error>> {
            self.create_diagnostic()
        }
    }
}
