pub trait SzAbstractFactory {
    // fn new(grpc_url: String) -> Self;
    fn create_product(&self) -> Result<impl SzProduct, Box<dyn std::error::Error>>;
    fn create_diagnostic(&self) -> Result<impl SzDiagnostic, Box<dyn std::error::Error>>;
    fn close(&self) -> Result<(), Box<dyn std::error::Error>>;
}

pub trait SzDiagnostic {
    fn destroy(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    fn check_repository_performance(
        &mut self,
        seconds_to_run: i32,
    ) -> Result<String, Box<dyn std::error::Error>>;
    fn get_feature(&mut self, feature_id: i64) -> Result<String, Box<dyn std::error::Error>>;
    fn get_repository_info(&mut self) -> Result<String, Box<dyn std::error::Error>>;
    fn purge_repository(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    fn reinitialize(&mut self, config_id: i64) -> Result<(), Box<dyn std::error::Error>>;
}

pub trait SzProduct {
    fn destroy(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    fn get_license(&mut self) -> Result<String, Box<dyn std::error::Error>>;
    fn get_version(&mut self) -> Result<String, Box<dyn std::error::Error>>;
}
