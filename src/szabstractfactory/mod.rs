#[cfg(test)]
mod tests;

pub mod szabstractfactory_y {

    use crate::szdiagnostic::szdiagnostic_y;
    use crate::szproduct::szproduct_y;
    use std::sync::OnceLock;
    use szdiagnostic_y::sz_diagnostic_client::SzDiagnosticClient;
    use szproduct_y::sz_product_client::SzProductClient;
    use tokio::runtime::Runtime;

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

    // For explanation of this technique, view https://www.youtube.com/watch?v=_ccDqRTx-JU

    pub struct Uninitialized;
    pub struct Initialized;

    pub struct SzAbstractFactory<State = Uninitialized> {
        grpc_url: String,
        state: std::marker::PhantomData<State>,
    }

    impl SzAbstractFactory<Uninitialized> {
        pub fn new(grpc_url: String) -> SzAbstractFactory<Initialized> {
            SzAbstractFactory {
                grpc_url: grpc_url,
                state: std::marker::PhantomData::<Initialized>,
            }
        }
    }

    impl SzAbstractFactory<Initialized> {
        pub fn create_product(&self) -> Result<szproduct_y::SzProduct, Box<dyn std::error::Error>> {
            let url = self.grpc_url.clone();
            let rt = get_runtime();

            let grpc_client = rt.block_on(async move { SzProductClient::connect(url).await })?;

            Ok(szproduct_y::SzProduct::new(grpc_client))
        }

        pub fn create_diagnostic(
            &self,
        ) -> Result<szdiagnostic_y::SzDiagnostic, Box<dyn std::error::Error>> {
            let url = self.grpc_url.clone();
            let rt = get_runtime();

            let grpc_client = rt.block_on(async move { SzDiagnosticClient::connect(url).await })?;

            Ok(szdiagnostic_y::SzDiagnostic { grpc_client })
        }

        pub fn destroy() -> SzAbstractFactory<Uninitialized> {
            SzAbstractFactory {
                grpc_url: String::from(""),
                state: std::marker::PhantomData::<Uninitialized>,
            }
        }
    }

    impl<State> SzAbstractFactory<State> {
        pub fn xx(grpc_url: String) -> SzAbstractFactory<Initialized> {
            SzAbstractFactory {
                grpc_url: grpc_url,
                state: std::marker::PhantomData::<Initialized>,
            }
        }
    }

    impl SzAbstractFactory {
        pub fn new_from_url(grpc_url: String) -> SzAbstractFactory<Initialized> {
            SzAbstractFactory::new(grpc_url)
        }
    }
}
