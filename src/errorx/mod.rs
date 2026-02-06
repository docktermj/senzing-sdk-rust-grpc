#[cfg(test)]
mod tests;

pub mod errortypes;

// Re-export the macro so it's accessible as crate::errorx::extract_senzing_error
pub use crate::extract_senzing_error;

use serde_json::Value;
use std::any::Any;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result};

// ----------------------------------------------------------------------------
// Traits
// ----------------------------------------------------------------------------

pub trait SzErrorTrait: Debug + Display + Error + Any {
    fn as_any(&self) -> &dyn Any;
    fn error_type(&self) -> SzErrorTypes;
    fn is_bad_input_error(&self) -> bool;
    fn is_general_error(&self) -> bool;
    fn is_retryable_error(&self) -> bool;
    fn is_senzing_error(&self) -> bool;
    fn is_unrecoverable_error(&self) -> bool;
    fn kind(&self, szerror: SzErrorTypes) -> bool;
    fn message(&self) -> &str;
}

// ----------------------------------------------------------------------------
// Structs
// ----------------------------------------------------------------------------

#[derive(Debug, Default)]
pub struct SzBadInputError;

#[derive(Debug, Default)]
pub struct SzConfigurationError;

#[derive(Debug, Default)]
pub struct SzDatabaseConnectionLostError;

#[derive(Debug, Default)]
pub struct SzDatabaseError;

#[derive(Debug, Default)]
pub struct SzDatabaseTransientError;

#[derive(Debug, Default)]
pub struct SzGeneralError;

#[derive(Debug, Default)]
pub struct SzLicenseError;

#[derive(Debug, Default)]
pub struct SzNotFoundError;

#[derive(Debug, Default)]
pub struct SzNotInitializedError;

#[derive(Debug, Default)]
pub struct SzReplaceConflictError;

#[derive(Debug, Default)]
pub struct SzRetryableError;

#[derive(Debug, Default)]
pub struct SzRetryTimeoutExceededError;

#[derive(Debug, Default)]
pub struct SzSdkError;

#[derive(Debug, Default)]
pub struct SzUnhandledError;

#[derive(Debug, Default)]
pub struct SzUnknownDataSourceError;

#[derive(Debug, Default)]
pub struct SzUnrecoverableError;

// ----------------------------------------------------------------------------
// Enums
// ----------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[non_exhaustive]
pub enum SzErrorTypes {
    BadInputError,
    ConfigurationError,
    DatabaseConnectionLostError,
    DatabaseError,
    DatabaseTransientError,
    #[default]
    Error,
    GeneralError,
    LicenseError,
    NotFoundError,
    NotInitializedError,
    ReplaceConflictError,
    RetryableError,
    RetryTimeoutExceededError,
    SdkError,
    UnhandledError,
    UnknownDataSourceError,
    UnrecoverableError,
}

// ----------------------------------------------------------------------------
// SzError
// ----------------------------------------------------------------------------

// For explanation of the following technique, view https://www.youtube.com/watch?v=_ccDqRTx-JU

#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[non_exhaustive]
pub struct SzError<State = SzErrorTypes> {
    error_hierarchy: Vec<SzErrorTypes>,
    error_type: SzErrorTypes,
    is_bad_input_error: bool,
    is_general_error: bool,
    is_retryable_error: bool,
    is_senzing_error: bool,
    is_unrecoverable_error: bool,
    message: String,
    state: std::marker::PhantomData<State>,
}

// impl SzError<SzBadInputError> {}
// impl SzError<SzConfigurationError> {}
// impl SzError<SzDatabaseConnectionLostError> {}

impl SzError<SzDatabaseError> {
    pub fn mjd_was_here(&self, suffix: String) {
        println!("    >>>>>> MJD was here -> {}", suffix)
    }
}

// impl SzError<SzDatabaseTransientError> {}
// impl SzError<SzError> {}
// impl SzError<SzGeneralError> {}
// impl SzError<SzLicenseError> {}
// impl SzError<SzNotFoundError> {}
// impl SzError<SzNotInitializedError> {}
// impl SzError<SzReplaceConflictError> {}
// impl SzError<SzRetryableError> {}
// impl SzError<SzRetryTimeoutExceededError> {}
// impl SzError<SzSdkError> {}
// impl SzError<SzUnknownDataSourceError> {}
// impl SzError<SzUnhandledError> {}
// impl SzError<SzUnrecoverableError> {}

impl<State: 'static> SzError<State> {
    /// Attempts to downcast this error to a specific SzError type.
    ///
    /// # Returns
    ///
    /// * `Some(&T)` if the downcast succeeds
    /// * `None` if the downcast fails
    ///
    /// # Example
    ///
    /// ```ignore
    /// if let Some(general_error) = sz_error.downcast::<SzError<SzGeneralError>>() {
    ///     // Handle the general error specifically
    /// }
    /// ```
    pub fn downcast<T: 'static>(&self) -> Option<&T> {
        (self as &dyn Any).downcast_ref::<T>()
    }
}

// ----------------------------------------------------------------------------
// Trait methods
// ----------------------------------------------------------------------------

impl<State: Debug + 'static> Error for SzError<State> {}

impl<State> Display for SzError<State> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "SzError: {}", self.message)
    }
}

impl<State: Debug + 'static> SzErrorTrait for SzError<State> {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn error_type(&self) -> SzErrorTypes {
        self.error_type
    }

    fn is_bad_input_error(&self) -> bool {
        self.is_bad_input_error
    }

    fn is_general_error(&self) -> bool {
        self.is_general_error
    }

    fn is_retryable_error(&self) -> bool {
        self.is_retryable_error
    }

    fn is_senzing_error(&self) -> bool {
        self.is_senzing_error
    }

    fn is_unrecoverable_error(&self) -> bool {
        self.is_unrecoverable_error
    }

    fn kind(&self, szerror: SzErrorTypes) -> bool {
        self.error_hierarchy.contains(&szerror)
    }

    fn message(&self) -> &str {
        &self.message
    }
}

// ----------------------------------------------------------------------------
// Constructors
// ----------------------------------------------------------------------------

impl SzError {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(message: String) -> Box<dyn SzErrorTrait> {
        let error_type_x = extract_error_type(&message);
        if let Some(error_type) = error_type_x {
            match error_type {
                SzErrorTypes::BadInputError => Box::new(create_sz_error::<SzBadInputError>(
                    message,
                    error_type,
                    vec![SzErrorTypes::BadInputError, SzErrorTypes::Error],
                )),
                SzErrorTypes::ConfigurationError => {
                    Box::new(create_sz_error::<SzConfigurationError>(
                        message,
                        error_type,
                        vec![
                            SzErrorTypes::ConfigurationError,
                            SzErrorTypes::GeneralError,
                            SzErrorTypes::Error,
                        ],
                    ))
                }
                SzErrorTypes::DatabaseConnectionLostError => {
                    Box::new(create_sz_error::<SzDatabaseConnectionLostError>(
                        message,
                        error_type,
                        vec![
                            SzErrorTypes::DatabaseConnectionLostError,
                            SzErrorTypes::RetryableError,
                            SzErrorTypes::Error,
                        ],
                    ))
                }
                SzErrorTypes::DatabaseError => Box::new(create_sz_error::<SzDatabaseError>(
                    message,
                    error_type,
                    vec![
                        SzErrorTypes::DatabaseError,
                        SzErrorTypes::UnrecoverableError,
                        SzErrorTypes::Error,
                    ],
                )),
                SzErrorTypes::DatabaseTransientError => {
                    Box::new(create_sz_error::<SzDatabaseTransientError>(
                        message,
                        error_type,
                        vec![
                            SzErrorTypes::DatabaseTransientError,
                            SzErrorTypes::RetryableError,
                            SzErrorTypes::Error,
                        ],
                    ))
                }
                SzErrorTypes::GeneralError => Box::new(create_sz_error::<SzGeneralError>(
                    message,
                    error_type,
                    vec![SzErrorTypes::GeneralError, SzErrorTypes::Error],
                )),
                SzErrorTypes::LicenseError => Box::new(create_sz_error::<SzLicenseError>(
                    message,
                    error_type,
                    vec![
                        SzErrorTypes::LicenseError,
                        SzErrorTypes::UnrecoverableError,
                        SzErrorTypes::Error,
                    ],
                )),
                SzErrorTypes::NotFoundError => Box::new(create_sz_error::<SzNotFoundError>(
                    message,
                    error_type,
                    vec![
                        SzErrorTypes::NotFoundError,
                        SzErrorTypes::BadInputError,
                        SzErrorTypes::Error,
                    ],
                )),
                SzErrorTypes::NotInitializedError => {
                    Box::new(create_sz_error::<SzNotInitializedError>(
                        message,
                        error_type,
                        vec![
                            SzErrorTypes::NotInitializedError,
                            SzErrorTypes::UnrecoverableError,
                            SzErrorTypes::Error,
                        ],
                    ))
                }
                SzErrorTypes::ReplaceConflictError => {
                    Box::new(create_sz_error::<SzReplaceConflictError>(
                        message,
                        error_type,
                        vec![
                            SzErrorTypes::ReplaceConflictError,
                            SzErrorTypes::GeneralError,
                            SzErrorTypes::Error,
                        ],
                    ))
                }
                SzErrorTypes::RetryableError => Box::new(create_sz_error::<SzRetryableError>(
                    message,
                    error_type,
                    vec![SzErrorTypes::RetryableError, SzErrorTypes::Error],
                )),
                SzErrorTypes::RetryTimeoutExceededError => {
                    Box::new(create_sz_error::<SzRetryTimeoutExceededError>(
                        message,
                        error_type,
                        vec![
                            SzErrorTypes::RetryTimeoutExceededError,
                            SzErrorTypes::RetryableError,
                            SzErrorTypes::Error,
                        ],
                    ))
                }
                SzErrorTypes::SdkError => Box::new(create_sz_error::<SzSdkError>(
                    message,
                    error_type,
                    vec![
                        SzErrorTypes::SdkError,
                        SzErrorTypes::GeneralError,
                        SzErrorTypes::Error,
                    ],
                )),
                SzErrorTypes::UnhandledError => Box::new(create_sz_error::<SzUnhandledError>(
                    message,
                    error_type,
                    vec![
                        SzErrorTypes::UnhandledError,
                        SzErrorTypes::UnrecoverableError,
                        SzErrorTypes::Error,
                    ],
                )),
                SzErrorTypes::UnknownDataSourceError => {
                    Box::new(create_sz_error::<SzUnknownDataSourceError>(
                        message,
                        error_type,
                        vec![
                            SzErrorTypes::UnknownDataSourceError,
                            SzErrorTypes::BadInputError,
                            SzErrorTypes::Error,
                        ],
                    ))
                }
                SzErrorTypes::UnrecoverableError => {
                    Box::new(create_sz_error::<SzUnrecoverableError>(
                        message,
                        error_type,
                        vec![SzErrorTypes::UnrecoverableError, SzErrorTypes::Error],
                    ))
                }
                SzErrorTypes::Error => Box::new(create_sz_error::<SzErrorTypes>(
                    message,
                    error_type,
                    vec![SzErrorTypes::Error],
                )),
            }
        } else {
            // No error type could be extracted, return generic SzError variant.
            Box::new(create_sz_error::<SzErrorTypes>(
                message,
                SzErrorTypes::Error,
                vec![SzErrorTypes::Error],
            ))
        }
    }

    pub fn error_is(err: &Box<dyn Error>, senzing_type: impl SzErrorTrait) -> bool {
        let target_type_id = senzing_type.as_any().type_id();

        // Check the error itself
        if let Some(senzing_error) = extract_senzing_error!(err) {
            if senzing_error.as_any().type_id() == target_type_id {
                return true;
            }
        }

        // Walk the source chain
        let mut source = err.source();
        while let Some(err) = source {
            if let Some(senzing_error) = extract_senzing_error!(err) {
                if senzing_error.as_any().type_id() == target_type_id {
                    return true;
                }
            }
            source = err.source();
        }

        false
    }

    pub fn is_senzing_retryable_error(err: &Box<dyn Error>) -> bool {
        // Check the error itself
        if let Some(senzing_error) = extract_senzing_error!(err) {
            return senzing_error.is_retryable_error();
        }
        // Walk the source chain
        let mut source = err.source();
        while let Some(err) = source {
            if let Some(senzing_error) = extract_senzing_error!(err) {
                return senzing_error.is_retryable_error();
            }
            source = err.source();
        }
        false
    }

    pub fn is_senzing_error(err: &Box<dyn Error>) -> bool {
        // Check the error itself
        if let Some(senzing_error) = extract_senzing_error!(err) {
            return senzing_error.is_senzing_error();
        }
        // Walk the source chain
        let mut source = err.source();
        while let Some(err2) = source {
            if let Some(senzing_error) = extract_senzing_error!(err2) {
                return senzing_error.is_senzing_error();
            }
            source = err2.source();
        }
        false
    }

    // pub fn is_senzing_retryable_x(err: &(dyn std::error::Error + 'static)) -> bool {
    //     // Check the error itself
    //     if let Some(sz) = try_downcast_to_senzing_error(err) {
    //         return sz.is_retryable_error();
    //     }
    //     // Walk the source chain
    //     let mut source = err.source();
    //     while let Some(err) = source {
    //         if let Some(sz) = try_downcast_to_senzing_error(err) {
    //             return sz.is_retryable_error();
    //         }
    //         source = err.source();
    //     }
    //     false
    // }
}

// ----------------------------------------------------------------------------
// Macros
// ----------------------------------------------------------------------------

/// Attempts to downcast a `Box<dyn Error>` to `Option<&dyn SzErrorTrait>`.
///
/// This macro tries to downcast the error to all possible `SzError<T>` variants
/// and returns the first successful match as `&dyn SzErrorTrait`.
///
/// # Arguments
///
/// * `$error` - A boxed error (`Box<dyn Error>`)
///
/// # Returns
///
/// * `Option<&dyn SzErrorTrait>` - Some if the error is any variant of `SzError<T>`, None otherwise
///
/// # Example
///
/// ```ignore
/// let boxed_error: Box<dyn Error> = get_some_error();
/// let mary: Option<&dyn SzErrorTrait> = try_downcast_senzing_error!(boxed_error);
/// ```
#[macro_export]
macro_rules! extract_senzing_error {
    ($error:expr) => {
        (&*$error)
            .downcast_ref::<$crate::errorx::SzError<$crate::errorx::SzBadInputError>>()
            .map(|e| e as &dyn $crate::errorx::SzErrorTrait)
            .or_else(|| {
                (&*$error)
                    .downcast_ref::<$crate::errorx::SzError<$crate::errorx::SzConfigurationError>>()
                    .map(|e| e as &dyn $crate::errorx::SzErrorTrait)
            })
            .or_else(|| {
                (&*$error)
                    .downcast_ref::<$crate::errorx::SzError<$crate::errorx::SzDatabaseConnectionLostError>>()
                    .map(|e| e as &dyn $crate::errorx::SzErrorTrait)
            })
            .or_else(|| {
                (&*$error)
                    .downcast_ref::<$crate::errorx::SzError<$crate::errorx::SzDatabaseError>>()
                    .map(|e| e as &dyn $crate::errorx::SzErrorTrait)
            })
            .or_else(|| {
                (&*$error)
                    .downcast_ref::<$crate::errorx::SzError<$crate::errorx::SzDatabaseTransientError>>()
                    .map(|e| e as &dyn $crate::errorx::SzErrorTrait)
            })
            .or_else(|| {
                (&*$error)
                    .downcast_ref::<$crate::errorx::SzError<$crate::errorx::SzGeneralError>>()
                    .map(|e| e as &dyn $crate::errorx::SzErrorTrait)
            })
            .or_else(|| {
                (&*$error)
                    .downcast_ref::<$crate::errorx::SzError<$crate::errorx::SzLicenseError>>()
                    .map(|e| e as &dyn $crate::errorx::SzErrorTrait)
            })
            .or_else(|| {
                (&*$error)
                    .downcast_ref::<$crate::errorx::SzError<$crate::errorx::SzNotFoundError>>()
                    .map(|e| e as &dyn $crate::errorx::SzErrorTrait)
            })
            .or_else(|| {
                (&*$error)
                    .downcast_ref::<$crate::errorx::SzError<$crate::errorx::SzNotInitializedError>>()
                    .map(|e| e as &dyn $crate::errorx::SzErrorTrait)
            })
            .or_else(|| {
                (&*$error)
                    .downcast_ref::<$crate::errorx::SzError<$crate::errorx::SzReplaceConflictError>>()
                    .map(|e| e as &dyn $crate::errorx::SzErrorTrait)
            })
            .or_else(|| {
                (&*$error)
                    .downcast_ref::<$crate::errorx::SzError<$crate::errorx::SzRetryableError>>()
                    .map(|e| e as &dyn $crate::errorx::SzErrorTrait)
            })
            .or_else(|| {
                (&*$error)
                    .downcast_ref::<$crate::errorx::SzError<$crate::errorx::SzRetryTimeoutExceededError>>()
                    .map(|e| e as &dyn $crate::errorx::SzErrorTrait)
            })
            .or_else(|| {
                (&*$error)
                    .downcast_ref::<$crate::errorx::SzError<$crate::errorx::SzSdkError>>()
                    .map(|e| e as &dyn $crate::errorx::SzErrorTrait)
            })
            .or_else(|| {
                (&*$error)
                    .downcast_ref::<$crate::errorx::SzError<$crate::errorx::SzUnhandledError>>()
                    .map(|e| e as &dyn $crate::errorx::SzErrorTrait)
            })
            .or_else(|| {
                (&*$error)
                    .downcast_ref::<$crate::errorx::SzError<$crate::errorx::SzUnknownDataSourceError>>()
                    .map(|e| e as &dyn $crate::errorx::SzErrorTrait)
            })
            .or_else(|| {
                (&*$error)
                    .downcast_ref::<$crate::errorx::SzError<$crate::errorx::SzUnrecoverableError>>()
                    .map(|e| e as &dyn $crate::errorx::SzErrorTrait)
            })
            .or_else(|| {
                (&*$error)
                    .downcast_ref::<$crate::errorx::SzError<$crate::errorx::SzError>>()
                    .map(|e| e as &dyn $crate::errorx::SzErrorTrait)
            })
    };
}

// ----------------------------------------------------------------------------
// Private functions
// ----------------------------------------------------------------------------

/// Helper function to create an SzError with computed boolean flags from hierarchy
fn create_sz_error<State>(
    message: String,
    error_type: SzErrorTypes,
    error_hierarchy: Vec<SzErrorTypes>,
) -> SzError<State> {
    let is_bad_input_error = error_hierarchy.contains(&SzErrorTypes::BadInputError);
    let is_general_error = error_hierarchy.contains(&SzErrorTypes::GeneralError);
    let is_retryable_error = error_hierarchy.contains(&SzErrorTypes::RetryableError);
    let is_unrecoverable_error = error_hierarchy.contains(&SzErrorTypes::UnrecoverableError);

    SzError {
        error_hierarchy,
        error_type,
        is_bad_input_error,
        is_general_error,
        is_retryable_error,
        is_senzing_error: true,
        is_unrecoverable_error,
        message,
        state: std::marker::PhantomData,
    }
}

fn extract_error_type(message: &str) -> Option<SzErrorTypes> {
    extract_json_from_message(message)
        .and_then(|json_str| serde_json::from_str::<Value>(&json_str).ok())
        .and_then(|json_value| extract_reason_from_json(&json_value))
        .and_then(|reason| extract_error_id_from_reason(&reason))
        .and_then(get_error_type_for_error_id)
}

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

fn get_error_type_for_error_id(error_id: i32) -> Option<SzErrorTypes> {
    errortypes::SZ_ERROR_TYPES.get(&error_id).copied()
}

/// Used to test that types have "normal" traits.
#[allow(dead_code)]
fn is_normal<T: Sized + Send + Sync + Unpin>() {}
