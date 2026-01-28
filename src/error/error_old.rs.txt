//! Error conversion utilities for gRPC responses.
//!
//! This module provides utilities for converting gRPC status errors
//! into Senzing-formatted errors with proper error code extraction.

use serde_json::Value;
use std::error::Error;
use std::fmt;

const MAX_REASONS: usize = 10;

/// A Senzing-specific error extracted from a gRPC error response.
#[derive(Debug)]
pub struct SenzingError {
    pub error_code: Option<i32>,
    pub message: String,
    pub reason: Option<String>,
}

impl fmt::Display for SenzingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(code) = self.error_code {
            write!(f, "SenzingError({}): {}", code, self.message)
        } else {
            write!(f, "SenzingError: {}", self.message)
        }
    }
}

impl Error for SenzingError {}

/// Converts a gRPC error into a Senzing error if possible.
///
/// This function examines the error message for embedded JSON containing
/// Senzing-specific error information and extracts it into a `SenzingError`.
///
/// # Arguments
///
/// * `original_error` - The error received from a gRPC call
///
/// # Returns
///
/// * `Some(SenzingError)` if the error contains Senzing error information
/// * `None` if the error is not a Senzing gRPC error
///
/// # Example
///
/// ```ignore
/// use senzing_sdk_rust_grpc::helper::convert_grpc_error;
///
/// let result = some_grpc_call().await;
/// if let Err(e) = result {
///     if let Some(senzing_error) = convert_grpc_error(&e) {
///         println!("Senzing error code: {:?}", senzing_error.error_code);
///     }
/// }
/// ```
pub fn convert_grpc_error<E: Error>(original_error: &E) -> Option<SenzingError> {
    let error_message = original_error.to_string();
    convert_grpc_error_message(&error_message)
}

/// Converts a gRPC error message string into a Senzing error if possible.
///
/// # Arguments
///
/// * `grpc_error_message` - The error message string from a gRPC call
///
/// # Returns
///
/// * `Some(SenzingError)` if the message contains Senzing error information
/// * `None` if the message is not a Senzing gRPC error
pub fn convert_grpc_error_message(grpc_error_message: &str) -> Option<SenzingError> {
    // Look for the "desc = " field in the gRPC error
    const DESC_PREFIX: &str = " desc = ";

    let desc_index = grpc_error_message.find(DESC_PREFIX)?;
    let senzing_error_message = &grpc_error_message[desc_index + DESC_PREFIX.len()..];

    // Find the start of JSON
    let brace_index = senzing_error_message.find('{')?;
    let senzing_error_json = &senzing_error_message[brace_index..];

    // Try to parse as JSON
    let json_value: Value = serde_json::from_str(senzing_error_json).ok()?;

    // Extract reason from JSON
    let reason = extract_reason_from_json(&json_value)?;

    Some(create_error_from_reason(senzing_error_json, &reason))
}

/// Extracts the "reason" field from a JSON error structure.
fn extract_reason_from_json(json_value: &Value) -> Option<String> {
    // Try to get "reason" directly
    if let Some(reason) = json_value.get("reason").and_then(|v| v.as_str()) {
        return Some(reason.to_string());
    }

    // Try to get "reason" from nested "error" object
    if let Some(error_obj) = json_value.get("error").and_then(|v| v.as_object()) {
        if let Some(reason) = error_obj.get("reason").and_then(|v| v.as_str()) {
            return Some(reason.to_string());
        }
        // Recursively check nested error
        if let Some(nested_error) = error_obj.get("error") {
            return extract_reason_from_json(nested_error);
        }
    }

    // Return the JSON as a string if no reason found
    Some(json_value.to_string())
}

/// Creates a SenzingError from a reason string.
fn create_error_from_reason(error_message: &str, reason: &str) -> SenzingError {
    if reason.len() < MAX_REASONS {
        return SenzingError {
            error_code: None,
            message: format!("errorMessage: {}; reason: {}", error_message, reason),
            reason: Some(reason.to_string()),
        };
    }

    // Try to extract Senzing error code from reason (format: "XXXX1234...")
    // The error code is typically at positions 4-8 in the reason string
    let error_code = if reason.len() >= 8 {
        reason[4..8].parse::<i32>().ok()
    } else {
        None
    };

    SenzingError {
        error_code,
        message: error_message.to_string(),
        reason: Some(reason.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_reason_from_json_direct() {
        let json: Value = serde_json::json!({
            "reason": "SZSDK00010001"
        });
        let reason = extract_reason_from_json(&json);
        assert_eq!(reason, Some("SZSDK00010001".to_string()));
    }

    #[test]
    fn test_extract_reason_from_json_nested() {
        let json: Value = serde_json::json!({
            "error": {
                "reason": "SZSDK00020002"
            }
        });
        let reason = extract_reason_from_json(&json);
        assert_eq!(reason, Some("SZSDK00020002".to_string()));
    }

    #[test]
    fn test_create_error_from_reason_with_code() {
        // Test with a reason string where positions 4-8 contain a parseable integer
        // The Go code extracts reason[4:8] and parses it as an integer
        let error = create_error_from_reason("test message", "XXXX0001ZZZZ Something went wrong");
        assert_eq!(error.error_code, Some(1));
        assert!(error.reason.is_some());
    }

    #[test]
    fn test_create_error_from_reason_short() {
        let error = create_error_from_reason("test message", "short");
        assert_eq!(error.error_code, None);
        assert!(error.message.contains("short"));
    }

    #[test]
    fn test_convert_grpc_error_message_valid() {
        // Test with a reason string where positions 4-8 contain a parseable integer
        let msg = r#"rpc error: code = Unknown desc = {"reason": "XXXX0042ZZZZ Error occurred"}"#;
        let result = convert_grpc_error_message(msg);
        assert!(result.is_some());
        let error = result.unwrap();
        assert_eq!(error.error_code, Some(42));
    }

    #[test]
    fn test_convert_grpc_error_message_no_desc() {
        let msg = "rpc error: code = Unknown";
        let result = convert_grpc_error_message(msg);
        assert!(result.is_none());
    }

    #[test]
    fn test_convert_grpc_error_message_no_json() {
        let msg = "rpc error: code = Unknown desc = plain text error";
        let result = convert_grpc_error_message(msg);
        assert!(result.is_none());
    }
}
