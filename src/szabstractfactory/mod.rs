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

    pub struct SzAbstractFactory {
        grpc_url: String,
    }

    impl SzAbstractFactory {
        pub fn new(grpc_url: String) -> Self {
            SzAbstractFactory { grpc_url }
        }

        pub fn create_product(&self) -> Result<szproduct_y::SzProduct, Box<dyn std::error::Error>> {
            let url = self.grpc_url.clone();
            let rt = get_runtime();

            let grpc_client = rt.block_on(async move { SzProductClient::connect(url).await })?;

            Ok(szproduct_y::SzProduct { grpc_client })
        }

        pub fn create_diagnostic(
            &self,
        ) -> Result<szdiagnostic_y::SzDiagnostic, Box<dyn std::error::Error>> {
            let url = self.grpc_url.clone();
            let rt = get_runtime();

            let grpc_client = rt.block_on(async move { SzDiagnosticClient::connect(url).await })?;

            Ok(szdiagnostic_y::SzDiagnostic { grpc_client })
        }
    }
}
