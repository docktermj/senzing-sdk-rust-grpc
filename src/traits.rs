pub trait SzProduct {
    fn get_version(&self) -> Result<String, Box<dyn std::error::Error>>;
}
