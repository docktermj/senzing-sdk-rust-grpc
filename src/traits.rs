pub trait SzProduct {
    fn get_license(&self) -> Result<String, Box<dyn std::error::Error>>;
    fn get_version(&self) -> Result<String, Box<dyn std::error::Error>>;
    fn destroy(&self) -> Result<(), Box<dyn std::error::Error>>;
}
