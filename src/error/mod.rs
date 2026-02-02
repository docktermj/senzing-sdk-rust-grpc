#[cfg(test)]
mod tests;

pub mod errortypes;

use serde_json::Value;
use std::error::Error;
use std::fmt;

// ----------------------------------------------------------------------------
// Enums
// ----------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
// - Mostly experimental
// ----------------------------------------------------------------------------

#[macro_export]
macro_rules! senzing_error_type1 {
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

#[macro_export]
macro_rules! senzing_error_type2 {
    (SzError::SzBadInputError, $value:expr) => {
        [
            SzError::SzBadInputError,
            SzError::SzNotFoundError,
            SzError::SzUnknownDataSourceError,
        ]
        .contains(&$value)
    };
    (SzError::SzGeneralError, $value:expr) => {
        [
            SzError::SzGeneralError,
            SzError::SzConfigurationError,
            SzError::SzReplaceConflictError,
            SzError::SzSdkError,
        ]
        .contains(&$value)
    };
    (SzError::SzRetryableError, $value:expr) => {
        [
            SzError::SzRetryableError,
            SzError::SzDatabaseConnectionLostError,
            SzError::SzDatabaseTransientError,
            SzError::SzRetryTimeoutExceededError,
        ]
        .contains(&$value)
    };
    (SzError::SzUnrecoverableError, $value:expr) => {
        [
            SzError::SzUnrecoverableError,
            SzError::SzDatabaseError,
            SzError::SzLicenseError,
            SzError::SzNotInitializedError,
            SzError::SzUnhandledError,
        ]
        .contains(&$value)
    };
    (SzError::SzError, $value:expr) => {
        [
            SzError::SzBadInputError,
            SzError::SzConfigurationError,
            SzError::SzDatabaseConnectionLostError,
            SzError::SzDatabaseError,
            SzError::SzDatabaseTransientError,
            SzError::SzError,
            SzError::SzGeneralError,
            SzError::SzLicenseError,
            SzError::SzNotFoundError,
            SzError::SzNotInitializedError,
            SzError::SzReplaceConflictError,
            SzError::SzRetryableError,
            SzError::SzRetryTimeoutExceededError,
            SzError::SzSdkError,
            SzError::SzUnhandledError,
            SzError::SzUnknownDataSourceError,
            SzError::SzUnrecoverableError,
        ]
        .contains(&$value)
    };
    ($other:pat, $value:expr) => {
        compile_error!(
            "senzing_error_type2! only accepts: SzError::SzBadInputError, SzError::SzGeneralError, SzError::SzRetryableError, SzError::SzUnrecoverableError, or SzError::SzError"
        )
    };
}

#[macro_export]
macro_rules! senzing_error_type3 {
    (SzError::SzBadInputError) => {
        [SzError::SzBadInputError, SzError::SzNotFoundError, SzError::SzUnknownDataSourceError]
    };
    (SzError::SzGeneralError) => {
        [SzError::SzGeneralError, SzError::SzConfigurationError, SzError::SzReplaceConflictError, SzError::SzSdkError]
    };
    (SzError::SzRetryableError) => {
        [SzError::SzRetryableError, SzError::SzDatabaseConnectionLostError, SzError::SzDatabaseTransientError, SzError::SzRetryTimeoutExceededError]
    };
    (SzError::SzUnrecoverableError) => {
        [SzError::SzUnrecoverableError, SzError::SzDatabaseError, SzError::SzLicenseError, SzError::SzNotInitializedError, SzError::SzUnhandledError]
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

// #[macro_export]
// macro_rules! senzing_error_type4 {
//     (SzError::SzBadInputError) => {
//         x if [SzError::SzBadInputError, SzError::SzNotFoundError, SzError::SzUnknownDataSourceError].contains(&x)
//     };
//     (SzError::SzGeneralError) => {
//         x if [SzError::SzGeneralError, SzError::SzConfigurationError, SzError::SzReplaceConflictError, SzError::SzSdkError].contains(&x)
//     };
//     (SzError::SzRetryableError) => {
//         x if [SzError::SzRetryableError, SzError::SzDatabaseConnectionLostError, SzError::SzDatabaseTransientError, SzError::SzRetryTimeoutExceededError].contains(&x)
//     };
//     (SzError::SzUnrecoverableError) => {
//         x if [SzError::SzUnrecoverableError, SzError::SzDatabaseError, SzError::SzLicenseError, SzError::SzNotInitializedError, SzError::SzUnhandledError].contains(&x)
//     };
//     // FIXME: This match arm also process non-SzError errors.
//     // It needs to match only SzError errors.
//     (SzError::SzError) => {
//         _
//     };
//     ($other:pat) => {
//         compile_error!(
//             "senzing_error_type! only accepts: SzError::SzBadInputError, SzError::SzGeneralError, SzError::SzRetryableError, SzError::SzUnrecoverableError, or SzError::SzError"
//         )
//     };
// }

#[macro_export]
macro_rules! senzing_error_type5 {
    (SzError::SzBadInputError) => {
        [
            SzError::SzBadInputError,
            SzError::SzNotFoundError,
            SzError::SzUnknownDataSourceError,
        ]
    };
    (SzError::SzGeneralError) => {
        [
            SzError::SzGeneralError,
            SzError::SzConfigurationError,
            SzError::SzReplaceConflictError,
            SzError::SzSdkError,
        ]
    };
    (SzError::SzRetryableError) => {
        [
            SzError::SzRetryableError,
            SzError::SzDatabaseConnectionLostError,
            SzError::SzDatabaseTransientError,
            SzError::SzRetryTimeoutExceededError,
        ]
    };
    (SzError::SzUnrecoverableError) => {
        [
            SzError::SzUnrecoverableError,
            SzError::SzDatabaseError,
            SzError::SzLicenseError,
            SzError::SzNotInitializedError,
            SzError::SzUnhandledError,
        ]
    };
    (SzError::SzError) => {
        [
            SzError::SzBadInputError,
            SzError::SzConfigurationError,
            SzError::SzDatabaseConnectionLostError,
            SzError::SzDatabaseError,
            SzError::SzDatabaseTransientError,
            SzError::SzError,
            SzError::SzGeneralError,
            SzError::SzLicenseError,
            SzError::SzNotFoundError,
            SzError::SzNotInitializedError,
            SzError::SzReplaceConflictError,
            SzError::SzRetryableError,
            SzError::SzRetryTimeoutExceededError,
            SzError::SzSdkError,
            SzError::SzUnhandledError,
            SzError::SzUnknownDataSourceError,
            SzError::SzUnrecoverableError,
        ]
    }; // ($other:pat) => {
       //     compile_error!(
       //         "senzing_error_type! only accepts: SzError::SzBadInputError, SzError::SzGeneralError, SzError::SzRetryableError, SzError::SzUnrecoverableError, or SzError::SzError"
       //     )
       // };
}

// ----------------------------------------------------------------------------
// SenzingError
// ----------------------------------------------------------------------------

/// A Senzing-specific error extracted from a gRPC error response.
#[derive(Debug)]
pub struct SenzingError {
    message: String,
    json: Option<String>,
}

pub fn as_senzing_error(error: impl ToString) -> SenzingError {
    let message = error.to_string();
    let json = extract_json_from_message(&message);
    SenzingError { message, json }
}

// pub fn as_senzing_error_from_err(error: Box<dyn std::error::Error>) -> SenzingError {
//     let message = error.to_string();
//     let json = extract_json_from_message(&message);
//     SenzingError { message, json }
// }

// ----------------------------------------------------------------------------
// SenzingError - methods
// ----------------------------------------------------------------------------

impl SenzingError {
    pub fn reason(&self) -> Option<String> {
        self.json
            .as_ref()
            .and_then(|json_str| serde_json::from_str::<Value>(json_str).ok())
            .and_then(|json_value| extract_reason_from_json(&json_value))
    }

    pub fn error_type(&self) -> Option<SzError> {
        self.json
            .as_ref()
            .and_then(|json_str| serde_json::from_str::<Value>(json_str).ok())
            .and_then(|json_value| extract_reason_from_json(&json_value))
            .and_then(|reason| extract_error_id_from_reason(&reason))
            .and_then(get_error_type_for_error_id)
    }

    pub fn is_error_type(&self, haystack: SzError) -> bool {
        let hay: &[SzError] = match haystack {
            SzError::SzError => &[
                SzError::SzBadInputError,
                SzError::SzConfigurationError,
                SzError::SzDatabaseConnectionLostError,
                SzError::SzDatabaseError,
                SzError::SzDatabaseTransientError,
                SzError::SzError,
                SzError::SzGeneralError,
                SzError::SzLicenseError,
                SzError::SzNotFoundError,
                SzError::SzNotInitializedError,
                SzError::SzReplaceConflictError,
                SzError::SzRetryableError,
                SzError::SzRetryTimeoutExceededError,
                SzError::SzSdkError,
                SzError::SzUnhandledError,
                SzError::SzUnknownDataSourceError,
                SzError::SzUnrecoverableError,
            ],
            SzError::SzBadInputError => &[
                SzError::SzBadInputError,
                SzError::SzNotFoundError,
                SzError::SzUnknownDataSourceError,
            ],
            SzError::SzConfigurationError => &[SzError::SzConfigurationError],
            SzError::SzDatabaseConnectionLostError => &[SzError::SzDatabaseConnectionLostError],
            SzError::SzDatabaseError => &[SzError::SzDatabaseError],
            SzError::SzDatabaseTransientError => &[SzError::SzDatabaseTransientError],

            SzError::SzGeneralError => &[
                SzError::SzGeneralError,
                SzError::SzConfigurationError,
                SzError::SzReplaceConflictError,
                SzError::SzSdkError,
            ],
            SzError::SzLicenseError => &[SzError::SzLicenseError],
            SzError::SzNotFoundError => &[SzError::SzNotFoundError],
            SzError::SzNotInitializedError => &[SzError::SzNotInitializedError],
            SzError::SzReplaceConflictError => &[SzError::SzReplaceConflictError],
            SzError::SzRetryableError => &[
                SzError::SzRetryableError,
                SzError::SzDatabaseConnectionLostError,
                SzError::SzDatabaseTransientError,
                SzError::SzRetryTimeoutExceededError,
            ],
            SzError::SzRetryTimeoutExceededError => &[SzError::SzRetryTimeoutExceededError],
            SzError::SzSdkError => &[SzError::SzSdkError],
            SzError::SzUnhandledError => &[SzError::SzUnhandledError],
            SzError::SzUnknownDataSourceError => &[SzError::SzUnknownDataSourceError],
            SzError::SzUnrecoverableError => &[
                SzError::SzUnrecoverableError,
                SzError::SzDatabaseError,
                SzError::SzLicenseError,
                SzError::SzNotInitializedError,
                SzError::SzUnhandledError,
            ],
        };

        // Return answer.

        let error_type = self.error_type();
        match error_type {
            Some(needle) => hay.contains(&needle),
            None => false,
        }
    }
}

impl Error for SenzingError {}

impl fmt::Display for SenzingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SenzingError: {}", self.message)
    }
}

// ----------------------------------------------------------------------------
// Functions
// ----------------------------------------------------------------------------

fn extract_json_from_message(message: &str) -> Option<String> {
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

            // Validate it's actually JSON
            if serde_json::from_str::<Value>(&unescaped_json).is_ok() {
                return Some(unescaped_json);
            }
        }
    }

    // Fallback: Look for JSON by finding opening braces
    let brace_index = message.find('{')?;
    let json_str = &message[brace_index..];

    // Validate it's actually JSON and return
    if serde_json::from_str::<Value>(json_str).is_ok() {
        return Some(json_str.to_string());
    }

    None
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

/// Extracts the error ID from a reason string.
///
/// The reason string is expected to be in the format "SENZNNNN|description"
/// where NNNN is a numeric error code.
///
/// # Arguments
///
/// * `reason` - The reason string from a Senzing error
///
/// # Returns
///
/// * `Some(i32)` if a valid error ID was found
/// * `None` if the reason string doesn't match the expected format
///
/// # Example
///
/// ```ignore
/// let error_id = extract_error_id_from_reason("SENZ0060|Unknown feature ID");
/// assert_eq!(error_id, Some(60));
/// ```
fn extract_error_id_from_reason(reason: &str) -> Option<i32> {
    // Check if reason starts with "SENZ"
    let after_prefix = reason.strip_prefix("SENZ")?;

    // Find the position of '|' which separates the code from the description
    let pipe_pos = after_prefix.find('|')?;

    // Extract the numeric part and parse it
    let code_str = &after_prefix[..pipe_pos];
    code_str.parse::<i32>().ok()
}

fn get_error_type_for_error_id(error_id: i32) -> Option<SzError> {
    errortypes::SZ_ERROR_TYPES.get(&error_id).copied()
}

// fn try_thing() -> SzError {
//     SzError::SzBadInputError
// }
