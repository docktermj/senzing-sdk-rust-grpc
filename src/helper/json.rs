//! Test utilities for the Senzing gRPC SDK.
//!
//! This module provides common test helper functions used across test modules.

/// Checks if a string is valid JSON.
///
/// This function attempts to parse the string as JSON and returns true if successful.
/// It also prints the result and JSON content for debugging purposes.
///
/// # Arguments
///
/// * `s` - The string to validate as JSON
///
/// # Returns
///
/// * `true` if the string is valid JSON
/// * `false` otherwise
pub fn is_valid_json(s: String) -> bool {
    let result = serde_json::from_str::<serde_json::Value>(&s).is_ok();
    println!("\n>>>>>> is_valid_json: {:?}; JSON: {:?}", result, s);
    result
}
