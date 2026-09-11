// Example: Synchronous method calling an async method

use std::time::Duration;

// An async function that simulates some async work
async fn fetch_data_async(id: u32) -> Result<String, Box<dyn std::error::Error + Send>> {
    // Simulate async work (e.g., network call, database query)
    tokio::time::sleep(Duration::from_millis(100)).await;
    Ok(format!("Data for ID: {}", id))
}

// A synchronous wrapper that calls the async function
pub fn fetch_data_sync(id: u32) -> Result<String, Box<dyn std::error::Error + Send>> {
    // Spawn a new OS thread to avoid runtime conflicts
    let handle = std::thread::spawn(move || {
        // Create a new tokio runtime in the isolated thread
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();

        // Run the async function on this runtime
        rt.block_on(async move { fetch_data_async(id).await })
    });

    // Wait for the thread to complete and return the result
    handle.join().unwrap()
}

// Example with a struct and method
pub struct DataService {
    pub base_url: String,
}

impl DataService {
    // Async method
    async fn get_async(&self, endpoint: &str) -> Result<String, Box<dyn std::error::Error + Send>> {
        tokio::time::sleep(Duration::from_millis(50)).await;
        Ok(format!("{}/{}", self.base_url, endpoint))
    }

    // Synchronous method that calls the async method
    pub fn get_sync(&self, endpoint: String) -> Result<String, Box<dyn std::error::Error + Send>> {
        let base_url = self.base_url.clone();

        let handle = std::thread::spawn(move || {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap();

            rt.block_on(async move {
                let service = DataService { base_url };
                service.get_async(&endpoint).await
            })
        });

        handle.join().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fetch_data_sync() {
        let result = fetch_data_sync(42);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Data for ID: 42");
    }

    #[test]
    fn test_data_service_sync() {
        let service = DataService {
            base_url: "https://api.example.com".to_string(),
        };

        let result = service.get_sync("users".to_string());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "https://api.example.com/users");
    }
}
