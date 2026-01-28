#[cfg(test)]
mod tests;

use serde_json::Value;
use std::error::Error;
use std::fmt;

// ----------------------------------------------------------------------------
// Enums
// ----------------------------------------------------------------------------

pub enum SzError {
    SzBadInputError,
    SzConfigurationError,
    SzDatabaseConnectionLostError,
    SzDatabaseError,
    SzDatabaseTransientError,
    SzError,
    SzGeneralError,
    SzLicenseError,
    SzNotFoundError,
    SzNotInitializedError,
    SzReplaceConflictError,
    SzRetryableError,
    SzRetryTimeoutExceededError,
    SzSdkError,
    SzUnhandledError,
    SzUnknownDataSourceError,
    SzUnrecoverableError,
}

// ----------------------------------------------------------------------------
// Macros
// ----------------------------------------------------------------------------

#[macro_export]
macro_rules! senzing_error_type {
    (SzError::SzBadInputError) => {
        SzError::SzBadInputError | SzError::SzNotFoundError | SzError::SzUnknownDataSourceError
    };
    (SzError::SzGeneralError) => {
        SzError::SzGeneralError
            | SzError::SzConfigurationError
            | SzError::SzReplaceConflictError
            | SzError::SzSdkError
    };
    (SzError::SzRetryableError) => {
        SzError::SzRetryableError
            | SzError::SzDatabaseConnectionLostError
            | SzError::SzDatabaseTransientError
            | SzError::SzRetryTimeoutExceededError
    };
    (SzError::SzUnrecoverableError) => {
        SzError::SzUnrecoverableError
            | SzError::SzDatabaseError
            | SzError::SzLicenseError
            | SzError::SzNotInitializedError
            | SzError::SzUnhandledError
    };
    // FIXME: This match arm also process non-SzError errors.
    // It needs to match only SzError errors.
    (SzError::SzError) => {
        _
    };
    ($other:pat) => {
        compile_error!(
            "senzing_error_type! only accepts: SzError::SzBadInputError, SzError::SzGeneralError, SzError::SzRetryableError, SzError::SzUnrecoverableError, or SzError::SzError"
        )
    };
}

// ----------------------------------------------------------------------------
// SenzingError
// ----------------------------------------------------------------------------

/// A Senzing-specific error extracted from a gRPC error response.
#[derive(Debug)]
pub struct SenzingError {
    message: String,
}

/// Builds a SenzingError from any type that can be converted to a string.
///
/// # Arguments
///
/// * `error` - Any value that implements `ToString` (errors, strings, etc.)
///
/// # Example
///
/// ```ignore
/// // From an error
/// let senzing_err = build_senzing_error(&some_error);
///
/// // From a string
/// let senzing_err = build_senzing_error("error message");
/// ```
pub fn build_senzing_error(error: impl ToString) -> SenzingError {
    SenzingError {
        message: error.to_string(),
    }
}

// ----------------------------------------------------------------------------
// SenzingError - methods
// ----------------------------------------------------------------------------

impl SenzingError {
    /// Extracts the "reason" field from JSON embedded in the error message.
    ///
    /// This method searches for JSON in the error message and attempts to extract
    /// the "reason" field. The JSON may contain the reason directly or nested within
    /// an "error" object.
    ///
    /// # Returns
    ///
    /// The reason string if found, otherwise an empty string.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let error = build_senzing_error(Box::new(std::io::Error::other(
    ///     r#"rpc error: code = Unknown desc = {"reason": "SZSDK00010001"}"#
    /// )));
    /// assert_eq!(error.reason(), "SZSDK00010001");
    /// ```
    pub fn reason(&self) -> String {
        extract_reason_from_message(&self.message).unwrap_or_default()
    }
}

impl fmt::Display for SenzingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SenzingError: {}", self.message)
    }
}

impl Error for SenzingError {}

/// Extracts the "reason" field from an error message containing JSON.
///
/// This function looks for JSON in the error message and attempts to extract
/// the "reason" field from it. It handles both direct JSON and escaped JSON strings.
///
/// # Arguments
///
/// * `message` - The error message string that may contain embedded JSON
///
/// # Returns
///
/// * `Some(String)` if a reason field was found in the JSON
/// * `None` if no JSON or reason field was found
fn extract_reason_from_message(message: &str) -> Option<String> {
    // Try to find and parse escaped JSON in the "self:" field
    if let Some(self_index) = message.find("self: \"") {
        let json_start = self_index + 7; // Length of "self: \""
        let remaining = &message[json_start..];

        // Find the end of the escaped JSON string (look for closing quote)
        // We need to handle escaped quotes within the JSON
        if let Some(json_end) = find_json_string_end(remaining) {
            let escaped_json = &remaining[..json_end];

            // Unescape the JSON string by replacing escape sequences
            let unescaped_json = escaped_json
                .replace("\\\"", "\"")
                .replace("\\n", "\n")
                .replace("\\\\", "\\");

            // Try to parse as JSON
            if let Ok(json_value) = serde_json::from_str::<Value>(&unescaped_json)
                && let Some(reason) = extract_reason_from_json(&json_value)
            {
                return Some(reason);
            }
        }
    }

    // Fallback: Look for JSON by finding opening braces
    let brace_index = message.find('{')?;
    let json_str = &message[brace_index..];

    // Try to parse as JSON
    let json_value: Value = serde_json::from_str(json_str).ok()?;

    // Extract the reason field from the JSON
    extract_reason_from_json(&json_value)
}

/// Finds the end of an escaped JSON string by looking for the closing quote.
///
/// This function handles escaped quotes within the JSON string.
///
/// # Arguments
///
/// * `s` - The string starting with an escaped JSON value
///
/// # Returns
///
/// * `Some(usize)` - The index of the closing quote
/// * `None` - If no closing quote is found
fn find_json_string_end(s: &str) -> Option<usize> {
    let mut in_escape = false;
    for (i, c) in s.chars().enumerate() {
        if in_escape {
            in_escape = false;
            continue;
        }
        match c {
            '\\' => in_escape = true,
            '"' => return Some(i),
            _ => {}
        }
    }
    None
}

/// Extracts the "reason" field from a JSON value.
///
/// This function recursively searches for the "reason" field in the JSON,
/// checking both the root level and nested "error" objects.
///
/// # Arguments
///
/// * `json_value` - The parsed JSON value to search
///
/// # Returns
///
/// * `Some(String)` if a "reason" field was found
/// * `None` if no "reason" field exists
fn extract_reason_from_json(json_value: &Value) -> Option<String> {
    // Try to get "reason" directly at the root level
    if let Some(reason) = json_value.get("reason").and_then(|v| v.as_str()) {
        return Some(reason.to_string());
    }

    // Try to get "reason" from nested "error" object
    if let Some(error_obj) = json_value.get("error").and_then(|v| v.as_object()) {
        if let Some(reason) = error_obj.get("reason").and_then(|v| v.as_str()) {
            return Some(reason.to_string());
        }
        // Recursively check nested error objects
        if let Some(nested_error) = error_obj.get("error") {
            return extract_reason_from_json(nested_error);
        }
    }

    None
}

// fn try_thing() -> SzError {
//     SzError::SzBadInputError
// }
